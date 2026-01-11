//! Table list sidebar view.

use crate::app::App;
use crate::types::{TableListItem, View};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};

/// Render the table list sidebar with collapsible schema groups.
pub fn render_tables(frame: &mut Frame, area: Rect, app: &App) {
    let is_focused = app.view == View::TableList;
    let visible_items = app.visible_list_items();

    let items: Vec<ListItem> = visible_items
        .iter()
        .map(|item| match item {
            TableListItem::SchemaHeader(schema) => {
                let expanded = app.expanded_schemas.contains(schema);
                let arrow = if expanded { "▼" } else { "▶" };
                let name = schema.name().to_uppercase();
                let content = format!("{} {}", arrow, name);
                ListItem::new(content).style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            }
            TableListItem::Table(idx) => {
                let content = app
                    .tables
                    .get(*idx)
                    .map(|table| format!("  {} ({})", table.name, table.row_count))
                    .unwrap_or_else(|| "  <unknown>".to_string());
                ListItem::new(content)
            }
        })
        .collect();

    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Gray)
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Tables ")
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    let mut state = ListState::default();
    state.select(Some(app.list_index));

    // Calculate offset to keep selection centered
    // Visible height = area height - 2 (borders)
    let visible_height = area.height.saturating_sub(2) as usize;
    if visible_height > 0 {
        let half_height = visible_height / 2;
        // Center the selection, but clamp at boundaries
        let centered_offset = app.list_index.saturating_sub(half_height);
        let max_offset = visible_items.len().saturating_sub(visible_height);
        *state.offset_mut() = centered_offset.min(max_offset);
    }

    frame.render_stateful_widget(list, area, &mut state);
}
