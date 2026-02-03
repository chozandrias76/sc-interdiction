//! Routes view rendering.

use crate::tui::app::App;
use crate::tui::text::scroll_text;
use crate::tui::types::RouteSort;
use crate::tui::widgets::format_value;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table},
};

/// Render the routes view showing profitable trade routes.
pub fn render_routes(frame: &mut Frame, app: &mut App, area: Rect) {
    // Set hovered line to selected row for scroll_text
    // Use offset 1000 to separate from targets view line indices
    app.scroll.set_hovered(Some(1000 + app.selected));

    let header_cells = [
        Cell::from("Commodity").style(Style::default().fg(Color::Yellow)),
        Cell::from("Origin").style(Style::default().fg(Color::Yellow)),
        Cell::from("Destination").style(Style::default().fg(Color::Yellow)),
        Cell::from("Profit/SCU").style(Style::default().fg(Color::Yellow)),
        Cell::from("Haul Value").style(Style::default().fg(Color::Yellow)),
        Cell::from("Wikelo").style(Style::default().fg(Color::Yellow)),
        Cell::from("Demand").style(Style::default().fg(Color::Yellow)),
        Cell::from("Ship").style(Style::default().fg(Color::Yellow)),
    ];
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows: Vec<Row> = app
        .routes
        .iter()
        .enumerate()
        .map(|(i, route)| {
            // Use scroll_text for locations - scrolls when selected
            let line_idx = 1000 + i;
            let origin = scroll_text(&route.origin, 25, line_idx, "", &app.scroll);
            let dest = scroll_text(&route.destination, 25, line_idx, "", &app.scroll);

            // Wikelo score with color coding
            let (wikelo_text, wikelo_color) = match route.wikelo_score {
                Some(s) if s > 50.0 => (format!("★{:.0}", s), Color::Magenta),
                Some(s) if s > 20.0 => (format!("{:.0}", s), Color::LightMagenta),
                Some(s) if s > 0.0 => (format!("{:.0}", s), Color::DarkGray),
                _ => ("-".to_string(), Color::DarkGray),
            };

            // Demand score with color coding
            let (demand_text, demand_color) = match route.demand_score {
                Some(s) if s > 50.0 => (format!("★{:.0}", s), Color::Cyan),
                Some(s) if s > 20.0 => (format!("{:.0}", s), Color::LightCyan),
                Some(s) if s > 0.0 => (format!("{:.0}", s), Color::DarkGray),
                _ => ("-".to_string(), Color::DarkGray),
            };

            let cells = vec![
                Cell::from(route.commodity.clone()),
                Cell::from(origin),
                Cell::from(dest),
                Cell::from(format!("{:.0}", route.profit_per_scu))
                    .style(Style::default().fg(Color::Green)),
                Cell::from(format_value(route.estimated_haul_value)),
                Cell::from(wikelo_text).style(Style::default().fg(wikelo_color)),
                Cell::from(demand_text).style(Style::default().fg(demand_color)),
                Cell::from(route.likely_ship.name.clone()),
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
        Constraint::Length(18),
        Constraint::Min(14),
        Constraint::Min(14),
        Constraint::Length(10),
        Constraint::Length(12),
        Constraint::Length(7),
        Constraint::Length(7),
        Constraint::Length(16),
    ];

    let sort_indicator = match app.route_sort {
        RouteSort::Profit => " (sort: Profit/SCU)",
        RouteSort::Value => " (sort: Haul Value)",
        RouteSort::Commodity => " (sort: Commodity)",
    };

    let table = Table::new(rows, widths).header(header).block(
        Block::default().borders(Borders::ALL).title(format!(
            " Hot Routes ({} total){} ",
            app.routes.len(),
            sort_indicator
        )),
    );

    frame.render_widget(table, area);

    // Scrollbar
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    let mut scrollbar_state = ScrollbarState::new(app.routes.len()).position(app.selected);
    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin::new(0, 1)),
        &mut scrollbar_state,
    );
}
