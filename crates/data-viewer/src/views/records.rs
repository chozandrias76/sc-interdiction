//! Records table view.

use crate::app::App;
use crate::types::View;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};

/// Maximum width for a column (normal mode).
const MAX_COL_WIDTH: usize = 30;
/// Minimum width for a column.
const MIN_COL_WIDTH: u16 = 8;
/// Maximum visible columns.
const MAX_VISIBLE_COLS: usize = 10;
/// Expanded column width (takes most of the space).
const EXPANDED_COL_WIDTH: u16 = 80;

/// Render the records table.
#[allow(clippy::too_many_lines)]
pub fn render_records(frame: &mut Frame, area: Rect, app: &App) {
    let is_focused = app.view == View::RecordList;

    let title = if let Some(name) = app.current_table_display() {
        format!(" {} ({} rows) ", name, app.total_records)
    } else {
        " No table selected ".to_string()
    };

    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Gray)
    };

    // Calculate column offset to keep selected column visible
    let column_offset = if app.selected_column >= MAX_VISIBLE_COLS {
        app.selected_column - MAX_VISIBLE_COLS + 1
    } else {
        0
    };

    // Visible columns (with horizontal scroll)
    let visible_col_indices: Vec<usize> = (column_offset..)
        .take(MAX_VISIBLE_COLS)
        .take_while(|&i| i < app.columns.len())
        .collect();

    // Header row with sort indicators and selected column highlight
    let header_cells: Vec<Cell> = visible_col_indices
        .iter()
        .filter_map(|&i| {
            let col = app.columns.get(i)?;
            let is_sorted = app.sort_column == Some(i);
            let is_selected = is_focused && app.selected_column == i;

            let header_text = if is_sorted {
                format!("{} {}", col.name, app.sort_direction.symbol())
            } else {
                col.name.clone()
            };

            let style = match (is_selected, is_sorted) {
                (true, true) => Style::default()
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
                    .fg(Color::Yellow)
                    .bg(Color::DarkGray),
                (true, false) => Style::default()
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
                    .fg(Color::Cyan)
                    .bg(Color::DarkGray),
                (false, true) => Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
                (false, false) => Style::default().add_modifier(Modifier::BOLD),
            };
            Some(Cell::from(header_text).style(style))
        })
        .collect();
    let header = Row::new(header_cells).height(1);

    // Determine if we have an expanded column in view
    let expanded_in_view = app
        .expanded_column
        .filter(|&ec| visible_col_indices.contains(&ec));

    // Data rows with selected column highlight
    let rows: Vec<Row> = app
        .records
        .iter()
        .map(|record| {
            let cells: Vec<Cell> = visible_col_indices
                .iter()
                .map(|&i| {
                    let is_expanded = app.expanded_column == Some(i);
                    let max_width = if is_expanded { 200 } else { MAX_COL_WIDTH };
                    let value = record
                        .get(i)
                        .map(|s| truncate(s, max_width))
                        .unwrap_or_default();
                    let is_selected_col = is_focused && app.selected_column == i;
                    let style = if is_selected_col {
                        Style::default().bg(Color::DarkGray)
                    } else {
                        Style::default()
                    };
                    Cell::from(value).style(style)
                })
                .collect();
            Row::new(cells)
        })
        .collect();

    // Column widths - expanded column gets more space, others shrink
    let widths: Vec<Constraint> = visible_col_indices
        .iter()
        .filter_map(|&i| {
            let col = app.columns.get(i)?;
            let is_expanded = app.expanded_column == Some(i);

            if is_expanded {
                Some(Constraint::Min(EXPANDED_COL_WIDTH))
            } else if expanded_in_view.is_some() {
                // Shrink other columns when one is expanded
                Some(Constraint::Max(MIN_COL_WIDTH))
            } else {
                let name_len = col.name.len().min(MAX_COL_WIDTH) as u16;
                Some(Constraint::Min(name_len.max(10)))
            }
        })
        .collect();

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .row_highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    let mut state = TableState::default();
    state.select(Some(app.record_index));

    // Calculate offset to keep selection centered
    // Visible height = area height - 2 (borders) - 1 (header)
    let visible_height = area.height.saturating_sub(3) as usize;
    if visible_height > 0 {
        let half_height = visible_height / 2;
        // Center the selection, but clamp at boundaries
        let centered_offset = app.record_index.saturating_sub(half_height);
        let max_offset = app.records.len().saturating_sub(visible_height);
        *state.offset_mut() = centered_offset.min(max_offset);
    }

    frame.render_stateful_widget(table, area, &mut state);

    // Show column indicator
    if !app.columns.is_empty() {
        let indicator = format!(" Col {}/{} ", app.selected_column + 1, app.columns.len());
        let indicator_area = Rect {
            x: area.x + area.width.saturating_sub(indicator.len() as u16 + 2),
            y: area.y,
            width: indicator.len() as u16,
            height: 1,
        };
        frame.render_widget(
            ratatui::widgets::Paragraph::new(indicator).style(Style::default().fg(Color::Yellow)),
            indicator_area,
        );
    }
}

/// Truncate a string to a maximum length.
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
