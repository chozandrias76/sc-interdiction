//! Help view rendering.

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

/// Render the help screen.
pub fn render_help(frame: &mut Frame, area: Rect) {
    let help_text = r#"
  NAVIGATION
    ↑/k, ↓/j     Move up/down
    ←/h, →/l     Select hotspot (Map view)
    PgUp/PgDn    Page up/down
    Home/End     Jump to start/end

  VIEWS
    Tab          Switch view
    1            Targets view
    2            Routes view
    3            Map view
    ?            Toggle help

  MAP CONTROLS
    z/[          Zoom out
    Z/]          Zoom in
    0            Reset zoom
    n            Show fewer hotspots
    N            Show more hotspots
    a            Toggle all/top-1 hotspots

  FILTERING (Targets)
    i            Toggle inbound only
    o            Toggle outbound only
    +/-          Adjust min threat level

  SORTING
    s            Cycle sort column
    S            Toggle sort direction

  GENERAL
    q, Ctrl+C    Quit
"#;

    let help = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Help ")
                .title_style(Style::default().fg(Color::Cyan)),
        )
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });

    // Center the help panel
    let popup_area = centered_rect(60, 70, area);
    frame.render_widget(Clear, popup_area);
    frame.render_widget(help, popup_area);
}

/// Create a centered rectangle for popups.
#[allow(clippy::indexing_slicing)] // Layout guarantees required indices exist
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
