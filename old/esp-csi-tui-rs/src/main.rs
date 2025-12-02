// Module declarations
mod app;
mod config;
mod models;
mod csi;
mod serial;
mod storage;
mod visualization;

use anyhow::Result;
use app::App;
use config::load_config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::io;
use std::time::Duration;
use visualization::ASCIIPlotter;

fn main() -> Result<()> {
    env_logger::init();

    // Load configuration
    let config = load_config()?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    match App::new(&config.serial.port, config.serial.baud_rate) {
        Ok(mut app) => {
            let res = run_app(&mut terminal, &mut app);

            // Restore terminal
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;

            if let Err(err) = res {
                println!("Error: {:?}", err);
            }
            Ok(())
        }
        Err(e) => {
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;

            eprintln!("❌ Failed to connect to ESP32!");
            eprintln!("Error: {}", e);
            eprintln!("\nTroubleshooting:");
            eprintln!("1. Ensure ESP32 is plugged in via USB");
            eprintln!("2. Check: ls /dev/ttyUSB*");
            eprintln!("3. Run with sudo: sudo cargo run");
            eprintln!("4. Config file: ~/.esp-csi/config.toml");
            Err(e)
        }
    }
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    let plotter = ASCIIPlotter::new(60, 8);
    let mut last_plot_size = 0;
    let mut plot_cache = String::new();

    loop {
        app.read_data()?;

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Percentage(15),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(17),
                    Constraint::Length(12),
                ])
                .split(f.area());

            // Header
            let header = Paragraph::new(vec![
                Line::from(
                    vec![
                        Span::styled("ESP32 CSI Monitor", Style::default().fg(Color::Cyan)),
                        Span::raw(" - Real-time WiFi Channel State Information Visualization"),
                    ]
                ),
            ])
            .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Cyan)))
            .style(Style::default().fg(Color::White));
            f.render_widget(header, chunks[0]);

            // Status Panel
            let status_lines = vec![
                Line::from(vec![
                    Span::styled("Connection: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&app.connection_status),
                ]),
                Line::from(vec![
                    Span::styled("Last Command: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&app.last_command),
                ]),
            ];
            let status = Paragraph::new(status_lines)
                .block(Block::default().borders(Borders::ALL).title("Status"))
                .style(Style::default().fg(Color::Green));
            f.render_widget(status, chunks[1]);

            // Device Output Window (only show last N lines for speed)
            let start_idx = if app.output_buffer.len() > 15 {
                app.output_buffer.len() - 15
            } else {
                0
            };
            let output_text: Vec<Line> = app
                .output_buffer[start_idx..]
                .iter()
                .map(|line| Line::from(line.as_str()))
                .collect();

            let output = Paragraph::new(output_text)
                .block(Block::default().borders(Borders::ALL).title("Device Output"))
                .wrap(Wrap { trim: false })
                .style(Style::default().fg(Color::Gray));
            f.render_widget(output, chunks[2]);

            // Amplitude Plot (cache until buffer grows significantly)
            if !app.state.packet_buffer.is_empty() {
                let current_size = app.state.packet_buffer.len();
                // Only regenerate plot if buffer has grown by 50+ packets
                if current_size > last_plot_size + 50 || last_plot_size == 0 {
                    plot_cache = plotter.plot_amplitude(&app.state.packet_buffer);
                    last_plot_size = current_size;
                }
                let plot = Paragraph::new(plot_cache.clone())
                    .block(Block::default().borders(Borders::ALL).title("Amplitude Plot"))
                    .style(Style::default().fg(Color::Cyan));
                f.render_widget(plot, chunks[3]);
            }

            // Live Data Display
            let data_display = Paragraph::new(vec![
                Line::from(vec![
                    Span::styled("Packets: ", Style::default().fg(Color::Magenta)),
                    Span::raw(app.state.stats.packets_received.to_string()),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Latest Data: ", Style::default().fg(Color::Magenta)),
                ]),
                Line::from(Span::styled(&app.data_display, Style::default().fg(Color::White))),
            ])
            .block(Block::default().borders(Borders::ALL).title("Live Data"))
            .style(Style::default().fg(Color::Magenta));
            f.render_widget(data_display, chunks[4]);

            // Commands Panel
            let commands_text = vec![
                Line::from(Span::styled("━━━ QUICK COMMANDS ━━━", Style::default().fg(Color::Yellow))),
                Line::from(""),
                Line::from(vec![
                    Span::styled("w", Style::default().fg(Color::Green)),
                    Span::raw(" = Sniffer  |  "),
                    Span::styled("c", Style::default().fg(Color::Green)),
                    Span::raw(" = Start  |  "),
                    Span::styled("x", Style::default().fg(Color::Green)),
                    Span::raw(" = Stop"),
                ]),
                Line::from(vec![
                    Span::styled("1", Style::default().fg(Color::Green)),
                    Span::raw(" = Traffic 10Hz  |  "),
                    Span::styled("2", Style::default().fg(Color::Green)),
                    Span::raw(" = CSI Config"),
                ]),
                Line::from(vec![
                    Span::styled("s", Style::default().fg(Color::Green)),
                    Span::raw(" = Show Config  |  "),
                    Span::styled("h", Style::default().fg(Color::Green)),
                    Span::raw(" = Help"),
                ]),
                Line::from(vec![
                    Span::styled("e", Style::default().fg(Color::Cyan)),
                    Span::raw(" = Export CSV  |  "),
                    Span::styled("q", Style::default().fg(Color::Red)),
                    Span::raw(" = Quit"),
                ]),
            ];
            let commands = Paragraph::new(commands_text)
                .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Yellow)))
                .style(Style::default().fg(Color::White));
            f.render_widget(commands, chunks[5]);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Ok(Event::Key(key)) = event::read() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                    KeyCode::Char('h') | KeyCode::Char('H') => {
                        app.send_command("help")?;
                    }
                    KeyCode::Char('w') | KeyCode::Char('W') => {
                        app.send_command("set-wifi --mode=sniffer --set-channel=6")?;
                    }
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        app.send_command("start --duration=60")?;
                        app.is_collecting = true;
                    }
                    KeyCode::Char('x') | KeyCode::Char('X') => {
                        app.send_command("stop")?;
                        app.is_collecting = false;
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        app.send_command("show-config")?;
                    }
                    KeyCode::Char('1') => {
                        app.send_command("set-traffic --frequency-hz=10")?;
                    }
                    KeyCode::Char('2') => {
                        app.send_command("set-csi")?;
                    }
                    KeyCode::Char('e') | KeyCode::Char('E') => {
                        // Use new CSVExporter with metadata
                        match export_data(&app) {
                            Ok(filename) => {
                                app.last_command = format!("✓ Saved to {}", filename);
                                app.output_buffer.push(format!("✓ Data exported to: {}", filename));
                            }
                            Err(e) => {
                                app.last_command = format!("✗ Export failed: {}", e);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Export collected CSI data to CSV with metadata
fn export_data(app: &app::App) -> Result<String> {
    // Create export filename with timestamp
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let home_dir = dirs::home_dir()
        .ok_or(anyhow::anyhow!("Could not determine home directory"))?;
    let output_dir = home_dir.join(".esp-csi").join("exports");
    std::fs::create_dir_all(&output_dir)?;
    let filename = output_dir.join(format!("csi_data_{}.csv", timestamp));
    
    // Prepare metadata
    let metadata = vec![
        ("Export Time".to_string(), chrono::Local::now().to_string()),
        ("Packet Count".to_string(), app.state.packet_buffer.len().to_string()),
        ("Collection Status".to_string(), 
         if app.is_collecting { "Active" } else { "Stopped" }.to_string()),
    ];
    
    // Export with metadata
    let packets = app.state.packet_buffer.iter().map(|p| p.clone()).collect::<Vec<_>>();
    storage::csv::CSVExporter::export_with_metadata(&packets, &filename, &metadata)?;
    
    Ok(filename.to_string_lossy().to_string())
}
