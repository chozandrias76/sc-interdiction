//! Common UI widgets and utilities.

use crate::tui::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Tabs},
};

/// Render the header with view tabs.
pub fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    use crate::tui::types::View;

    let titles = vec!["[1] Targets", "[2] Routes", "[3] Map"];
    let selected = match app.view {
        View::Targets | View::Help => 0,
        View::Routes => 1,
        View::Map => 2,
    };

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" SC Interdiction - {} ", app.location))
                .title_style(Style::default().fg(Color::Cyan).bold()),
        )
        .select(selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).bold());

    frame.render_widget(tabs, area);
}

/// Render the status bar at the bottom.
#[allow(clippy::indexing_slicing)] // Layout guarantees required indices exist
pub fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let status = if let Some(ref error) = app.error {
        Paragraph::new(error.as_str()).style(Style::default().fg(Color::Red))
    } else {
        Paragraph::new(app.status.as_str()).style(Style::default().fg(Color::Gray))
    };

    let keys = " q:Quit | 1/2/3:Views | Tab:Next | ?:Help | s:Sort ";

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let status_block = status.block(Block::default().borders(Borders::ALL));
    let keys_block = Paragraph::new(keys)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Right)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(status_block, chunks[0]);
    frame.render_widget(keys_block, chunks[1]);
}

/// Format a value in aUEC with K/M suffixes.
pub fn format_value(value: f64) -> String {
    if value >= 1_000_000.0 {
        format!("{:.2}M", value / 1_000_000.0)
    } else if value >= 1_000.0 {
        format!("{:.1}K", value / 1_000.0)
    } else {
        format!("{:.0}", value)
    }
}
