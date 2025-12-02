use crate::models::AppState;
use crate::ui::{setup_terminal, cleanup_terminal};
use crate::ui::components::{draw_header, draw_footer};
use crate::device::EspClient;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::*;
use std::time::{Duration, Instant};

pub struct App {
    state: AppState,
    should_quit: bool,
    last_update: Instant,
    esp_client: Option<EspClient>,
    status_message: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::default(),
            should_quit: false,
            last_update: Instant::now(),
            esp_client: None,
            status_message: "Ready. Press 'c' to connect (or 'd' for demo mode)".to_string(),
        }
    }

    pub fn handle_key_event(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('c') => self.handle_connect(),
            KeyCode::Char('d') => self.handle_demo_mode(),
            KeyCode::Char('s') => self.handle_start(),
            KeyCode::Char('e') => self.handle_stop(),
            KeyCode::Char('t') => self.state.current_tab = (self.state.current_tab + 1) % 3,
            KeyCode::Esc => self.state.current_tab = 0,
            _ => {}
        }
    }

    fn handle_demo_mode(&mut self) {
        tracing::info!("Starting demo mode with mock CSI data...");
        self.status_message = "Demo mode: Generating mock CSI data...".to_string();
        
        let mut client = EspClient::new(
            self.state.device_config.port.clone(),
            self.state.device_config.baud_rate
        );
        
        client.enable_demo_mode();
        
        match client.connect() {
            Ok(_) => {
                self.state.is_connected = true;
                self.esp_client = Some(client);
                self.status_message = "Demo mode active!".to_string();
                tracing::info!("Demo mode activated");
            }
            Err(e) => {
                self.status_message = format!("Demo mode failed: {}", e);
                tracing::error!("Demo mode failed: {}", e);
            }
        }
    }

    fn handle_connect(&mut self) {
        tracing::info!("Attempting to connect to device...");
        self.status_message = "Connecting to /dev/ttyUSB0...".to_string();
        
        let mut client = EspClient::new(
            self.state.device_config.port.clone(),
            self.state.device_config.baud_rate
        );
        
        match client.connect() {
            Ok(_) => {
                self.state.is_connected = true;
                self.esp_client = Some(client);
                self.status_message = "Connected successfully! Press 's' to start.".to_string();
                tracing::info!("Connected to device");
            }
            Err(e) => {
                self.status_message = format!("Connection failed: {}. Try 'd' for demo mode.", e);
                tracing::error!("Connection failed: {}", e);
            }
        }
    }

    fn handle_start(&mut self) {
        if !self.state.is_connected {
            self.status_message = "Not connected! Press 'c' to connect or 'd' for demo.".to_string();
            return;
        }

        if let Some(ref mut client) = self.esp_client {
            match client.configure(&self.state.device_config) {
                Ok(_) => {
                    match client.start_collection() {
                        Ok(_) => {
                            self.state.is_collecting = true;
                            self.status_message = "Collecting data...".to_string();
                            tracing::info!("Started data collection");
                        }
                        Err(e) => {
                            self.status_message = format!("Failed to start: {}", e);
                            tracing::error!("Failed to start collection: {}", e);
                        }
                    }
                }
                Err(e) => {
                    self.status_message = format!("Failed to configure: {}", e);
                    tracing::error!("Failed to configure: {}", e);
                }
            }
        }
    }

    fn handle_stop(&mut self) {
        if let Some(ref mut client) = self.esp_client {
            match client.stop_collection() {
                Ok(_) => {
                    self.state.is_collecting = false;
                    self.status_message = "Data collection stopped".to_string();
                    tracing::info!("Stopped data collection");
                }
                Err(e) => {
                    self.status_message = format!("Failed to stop: {}", e);
                    tracing::error!("Failed to stop: {}", e);
                }
            }
        }
    }

    fn collect_data(&mut self) {
        if self.state.is_collecting {
            if let Some(ref mut client) = self.esp_client {
                match client.read_measurement() {
                    Ok(Some(measurement)) => {
                        self.state.measurements.push(measurement);
                        self.status_message = format!("Collected {} measurements", self.state.measurements.len());
                    }
                    Ok(None) => {
                        // No data available yet
                    }
                    Err(e) => {
                        tracing::warn!("Failed to read measurement: {}", e);
                    }
                }
            }
        }
    }

    pub fn ui(&self, frame: &mut Frame) {
        let main_layout = crate::ui::components::create_main_layout(frame.area());

        draw_header(frame, main_layout[0], &self.state);

        let content_layout = crate::ui::components::create_content_layout(main_layout[1]);
        self.draw_left_panel(frame, content_layout[0]);
        self.draw_right_panel(frame, content_layout[1]);

        self.draw_status_bar(frame, main_layout[2]);
    }

    fn draw_left_panel(&self, frame: &mut Frame, area: Rect) {
        use ratatui::widgets::{Block, Borders, Paragraph};

        let config = &self.state.device_config;
        let info = format!(
            "Device Config

Port: {}
Baud: {}
Channel: {}
BW: {} MHz
Interval: {} ms

Status:
{}

Measurements: {}",
            config.port, config.baud_rate, config.channel, 
            config.bandwidth, config.collection_interval_ms,
            if self.state.is_connected { "Connected" } else { "Disconnected" },
            self.state.measurements.len()
        );

        let widget = Paragraph::new(info)
            .block(Block::default().borders(Borders::ALL).title("Config"));

        frame.render_widget(widget, area);
    }

    fn draw_right_panel(&self, frame: &mut Frame, area: Rect) {
        use ratatui::widgets::{Block, Borders, Paragraph};

        let content = if self.state.measurements.is_empty() {
            match self.state.current_tab {
                0 => "2D Magnitude Plot

No data collected yet.
Press 's' to start collection.".to_string(),
                1 => "Phase Plot

No data collected yet.
Press 's' to start collection.".to_string(),
                2 => "Heatmap

No data collected yet.
Press 's' to start collection.".to_string(),
                _ => "Plot View".to_string(),
            }
        } else {
            let latest = &self.state.measurements[self.state.measurements.len() - 1];
            match self.state.current_tab {
                0 => {
                    let magnitudes: Vec<f32> = latest.subcarrier_data.iter().map(|c| c.magnitude()).collect();
                    let avg_mag = magnitudes.iter().sum::<f32>() / magnitudes.len() as f32;
                    format!("2D Magnitude Plot

RSSI: {} dBm
Subcarriers: {}
Avg Magnitude: {:.3}
Min: {:.3}
Max: {:.3}", 
                        latest.rssi, 
                        latest.subcarrier_data.len(),
                        avg_mag,
                        magnitudes.iter().cloned().fold(f32::INFINITY, f32::min),
                        magnitudes.iter().cloned().fold(f32::NEG_INFINITY, f32::max))
                },
                1 => {
                    let phases: Vec<f32> = latest.subcarrier_data.iter().map(|c| c.phase()).collect();
                    let avg_phase = phases.iter().sum::<f32>() / phases.len() as f32;
                    format!("Phase Plot

Channel: {}
Bandwidth: {} MHz
Avg Phase: {:.3} rad
Noise Floor: {} dBm", 
                        latest.channel, 
                        latest.bandwidth,
                        avg_phase,
                        latest.noise_floor)
                },
                2 => format!("Heatmap

Total Measurements: {}
Latest: {}
Collection Rate: ~10 Hz", 
                    self.state.measurements.len(), 
                    latest.timestamp.format("%H:%M:%S")),
                _ => "Plot".to_string(),
            }
        };

        let widget = Paragraph::new(content)
            .block(Block::default().borders(Borders::ALL).title(
                match self.state.current_tab {
                    0 => "Magnitude",
                    1 => "Phase",
                    2 => "Heatmap",
                    _ => "Visualization"
                }
            ));

        frame.render_widget(widget, area);
    }

    fn draw_status_bar(&self, frame: &mut Frame, area: Rect) {
        use ratatui::widgets::Paragraph;

        let status = format!("{} | q: Quit | c: Connect | d: Demo | s: Start | e: Stop | t: Tab", self.status_message);
        let widget = Paragraph::new(status)
            .style(Style::default().fg(Color::Yellow));

        frame.render_widget(widget, area);
    }
}

pub async fn run() -> Result<()> {
    setup_terminal()?;
    let mut app = App::new();
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

    loop {
        terminal.draw(|f| app.ui(f))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                app.handle_key_event(key.code);
            }
        }

        if app.should_quit {
            break;
        }

        // Collect data if active
        if app.last_update.elapsed() > Duration::from_millis(100) {
            app.collect_data();
            app.last_update = Instant::now();
        }
    }

    cleanup_terminal()?;
    Ok(())
}
