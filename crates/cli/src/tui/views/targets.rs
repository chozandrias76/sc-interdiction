//! Targets view rendering.

use crate::tui::app::App;
use crate::tui::text::scroll_text;
use crate::tui::types::TargetSort;
use crate::tui::widgets::format_value;
use intel::TrafficDirection;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table},
};

/// Render the targets view showing predicted interdiction targets.
pub fn render_targets(frame: &mut Frame, app: &mut App, area: Rect) {
    // Set hovered line to selected row for scroll_text (must be before borrowing targets)
    app.scroll.set_hovered(Some(app.selected));

    let filtered: Vec<_> = app.filtered_targets().collect();

    // Header row
    let header_cells = [
        Cell::from("Dir").style(Style::default().fg(Color::Yellow)),
        Cell::from("Ship").style(Style::default().fg(Color::Yellow)),
        Cell::from("Cargo").style(Style::default().fg(Color::Yellow)),
        Cell::from("Destination").style(Style::default().fg(Color::Yellow)),
        Cell::from("Value").style(Style::default().fg(Color::Yellow)),
        Cell::from("Threat").style(Style::default().fg(Color::Yellow)),
    ];
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    // Data rows
    let rows: Vec<Row> = filtered
        .iter()
        .enumerate()
        .map(|(i, target)| {
            let dir = match target.direction {
                TrafficDirection::Arriving => "▼ IN",
                TrafficDirection::Departing => "▲ OUT",
            };
            let dir_color = match target.direction {
                TrafficDirection::Arriving => Color::Green,
                TrafficDirection::Departing => Color::Blue,
            };

            let threat_color = match target.likely_ship.threat_level {
                0..=2 => Color::Green,
                3..=5 => Color::Yellow,
                6..=8 => Color::LightRed,
                _ => Color::Red,
            };

            let threat_bar = "█".repeat(target.likely_ship.threat_level as usize)
                + &"░".repeat(10 - target.likely_ship.threat_level as usize);

            // Use scroll_text for destination - scrolls when selected
            let dest = scroll_text(&target.destination, 35, i, "", &app.scroll);

            let cells = vec![
                Cell::from(dir).style(Style::default().fg(dir_color)),
                Cell::from(target.likely_ship.name),
                Cell::from(target.commodity.clone()),
                Cell::from(dest),
                Cell::from(format_value(target.estimated_cargo_value)),
                Cell::from(threat_bar).style(Style::default().fg(threat_color)),
            ];

            let style = if i == app.selected {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            };

            Row::new(cells).style(style)
        })
        .collect();

    let widths = [
        Constraint::Length(6),
        Constraint::Length(22),
        Constraint::Length(20),
        Constraint::Min(20),
        Constraint::Length(12),
        Constraint::Length(12),
    ];

    let sort_indicator = match app.target_sort {
        TargetSort::Value => " (sort: Value)",
        TargetSort::Threat => " (sort: Threat)",
        TargetSort::Ship => " (sort: Ship)",
        TargetSort::Commodity => " (sort: Cargo)",
    };

    let filter_info = match (app.filter_inbound, app.filter_outbound, app.min_threat) {
        (true, _, _) => format!(" [INBOUND ONLY] min threat: {}", app.min_threat),
        (_, true, _) => format!(" [OUTBOUND ONLY] min threat: {}", app.min_threat),
        (_, _, t) if t > 0 => format!(" min threat: {}", t),
        _ => String::new(),
    };

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(format!(
            " Targets ({} shown){}{} ",
            filtered.len(),
            sort_indicator,
            filter_info
        )))
        .row_highlight_style(Style::default().add_modifier(Modifier::BOLD));

    frame.render_widget(table, area);

    // Scrollbar
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    let mut scrollbar_state = ScrollbarState::new(filtered.len()).position(app.selected);
    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin::new(0, 1)),
        &mut scrollbar_state,
    );
}
