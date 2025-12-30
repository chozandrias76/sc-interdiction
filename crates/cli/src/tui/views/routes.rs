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

            let cells = vec![
                Cell::from(route.commodity.clone()),
                Cell::from(origin),
                Cell::from(dest),
                Cell::from(format!("{:.0}", route.profit_per_scu))
                    .style(Style::default().fg(Color::Green)),
                Cell::from(format_value(route.estimated_haul_value)),
                Cell::from(route.likely_ship.name),
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
        Constraint::Length(20),
        Constraint::Min(15),
        Constraint::Min(15),
        Constraint::Length(12),
        Constraint::Length(14),
        Constraint::Length(18),
    ];

    let sort_indicator = match app.route_sort {
        RouteSort::Profit => " (sort: Profit/SCU)",
        RouteSort::Value => " (sort: Haul Value)",
        RouteSort::Commodity => " (sort: Commodity)",
    };

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
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
