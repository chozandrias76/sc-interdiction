//! Targets view rendering.

use crate::tui::app::App;
use crate::tui::text::scroll_text;
use crate::tui::types::TargetSort;
use crate::tui::widgets::format_value;
use intel::TrafficDirection;
use ratatui::{
    prelude::*,
    widgets::{
        Block, Borders, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Table,
    },
};

/// Render the targets view showing predicted interdiction targets.
pub fn render_targets(frame: &mut Frame, app: &mut App, area: Rect) {
    // Set hovered line to selected row for scroll_text (must be before borrowing targets)
    app.scroll.set_hovered(Some(app.selected));

    let filtered: Vec<_> = app.filtered_targets().collect();

    // Get selected target's wikelo_flag for detail panel and hint
    let selected_wikelo_flag = filtered
        .get(app.selected)
        .and_then(|t| t.wikelo_flag.as_ref());

    // Determine layout: split if detail panel should be shown
    let (table_area, detail_area) = if app.target_detail_expanded && selected_wikelo_flag.is_some()
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area);
        (chunks[0], Some(chunks[1]))
    } else {
        (area, None)
    };

    // Header row
    let header_cells = [
        Cell::from("Dir").style(Style::default().fg(Color::Yellow)),
        Cell::from("Ship").style(Style::default().fg(Color::Yellow)),
        Cell::from("Cargo").style(Style::default().fg(Color::Yellow)),
        Cell::from("Destination").style(Style::default().fg(Color::Yellow)),
        Cell::from("Wikelo").style(Style::default().fg(Color::Yellow)),
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

            // Wikelo indicator: "★3" for high-value with 3 items, "3" for regular, "-" for none
            let (wikelo_text, wikelo_color) = match &target.wikelo_flag {
                Some(flag) if flag.has_high_value => {
                    (format!("★{}", flag.item_count), Color::Yellow)
                }
                Some(flag) => (format!("{}", flag.item_count), Color::Green),
                None => ("-".to_string(), Color::DarkGray),
            };

            let cells = vec![
                Cell::from(dir).style(Style::default().fg(dir_color)),
                Cell::from(target.likely_ship.name.clone()),
                Cell::from(target.commodity.clone()),
                Cell::from(dest),
                Cell::from(wikelo_text).style(Style::default().fg(wikelo_color)),
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
        Constraint::Length(10),
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

    // Add hint for Enter key when selected target has wikelo_flag
    let detail_hint = if selected_wikelo_flag.is_some() && !app.target_detail_expanded {
        " (Enter: details)"
    } else if app.target_detail_expanded {
        " (Enter: hide)"
    } else {
        ""
    };

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(format!(
            " Targets ({} shown){}{}{} ",
            filtered.len(),
            sort_indicator,
            filter_info,
            detail_hint
        )))
        .row_highlight_style(Style::default().add_modifier(Modifier::BOLD));

    frame.render_widget(table, table_area);

    // Scrollbar
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    let mut scrollbar_state = ScrollbarState::new(filtered.len()).position(app.selected);
    frame.render_stateful_widget(
        scrollbar,
        table_area.inner(Margin::new(0, 1)),
        &mut scrollbar_state,
    );

    // Render detail panel if expanded
    if let (Some(detail_rect), Some(flag)) = (detail_area, selected_wikelo_flag) {
        render_wikelo_detail(frame, flag, detail_rect);
    }
}

/// Render the Wikelo items detail panel.
fn render_wikelo_detail(frame: &mut Frame, flag: &intel::SourceFlag, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    // Show top items (up to 5)
    for item in flag.top_items.iter().take(5) {
        let mut spans = vec![Span::styled(
            &item.name,
            Style::default().add_modifier(Modifier::BOLD),
        )];

        // Add category
        spans.push(Span::raw(format!(" ({:?})", item.category)));

        // Add value if present
        if let Some(value) = item.estimated_value {
            spans.push(Span::styled(
                format!(" ~{} aUEC", format_auec(value)),
                Style::default().fg(Color::Yellow),
            ));
        }

        lines.push(Line::from(spans));
    }

    // Show total count if more items exist
    if flag.item_count > 5 {
        lines.push(Line::from(Span::styled(
            format!("... and {} more items", flag.item_count - 5),
            Style::default().fg(Color::DarkGray),
        )));
    }

    let detail_title = if flag.has_high_value {
        format!(" Wikelo Items at {} (HIGH VALUE) ", flag.location)
    } else {
        format!(" Wikelo Items at {} ", flag.location)
    };

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(detail_title))
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

/// Format aUEC value with K/M suffix.
fn format_auec(value: u64) -> String {
    if value >= 1_000_000 {
        format!("{:.1}M", value as f64 / 1_000_000.0)
    } else if value >= 1_000 {
        format!("{:.0}K", value as f64 / 1_000.0)
    } else {
        value.to_string()
    }
}
