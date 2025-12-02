use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use crate::models::AppState;

pub fn draw_header(frame: &mut Frame, area: Rect, state: &AppState) {
    let header = if state.is_connected {
        format!(
            "esp-csi-tui-rs | Connected | Collecting: {} | Measurements: {}",
            if state.is_collecting { "Yes" } else { "No" },
            state.measurements.len()
        )
    } else {
        "esp-csi-tui-rs | Disconnected".to_string()
    };

    let header_widget = Paragraph::new(header)
        .block(Block::default().borders(Borders::BOTTOM))
        .style(Style::default().fg(Color::Cyan).bold());

    frame.render_widget(header_widget, area);
}

pub fn draw_footer(frame: &mut Frame, area: Rect) {
    let footer = "q: Quit | c: Connect | s: Start | e: Stop | t: Toggle Tab | h: Help";
    let footer_widget = Paragraph::new(footer)
        .style(Style::default().fg(Color::Gray));

    frame.render_widget(footer_widget, area);
}
