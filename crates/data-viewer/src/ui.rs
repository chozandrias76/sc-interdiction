//! Main UI rendering.

use crate::app::App;
use crate::types::View;
use crate::views::{render_detail, render_records, render_tables};
use ratatui::{prelude::*, widgets::Paragraph};

/// Render the entire UI.
pub fn render(frame: &mut Frame, app: &App) {
    let [title_area, main_area, status_area] = Layout::vertical([
        Constraint::Length(1), // Title
        Constraint::Min(1),    // Main content
        Constraint::Length(1), // Status bar
    ])
    .areas(frame.area());

    // Title bar
    render_title(frame, title_area, app);

    // Main content (tables sidebar + records)
    let [tables_area, records_area] = Layout::horizontal([
        Constraint::Length(25), // Tables sidebar
        Constraint::Min(1),     // Records
    ])
    .areas(main_area);

    render_tables(frame, tables_area, app);
    render_records(frame, records_area, app);

    // Status bar
    render_status(frame, status_area, app);

    // Detail popup (if active)
    if app.view == View::RecordDetail {
        render_detail(frame, app);
    }
}

fn render_title(frame: &mut Frame, area: Rect, _app: &App) {
    let title = Paragraph::new("Data Viewer - PostgreSQL (raw/silver/gold)")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);

    frame.render_widget(title, area);
}

fn render_status(frame: &mut Frame, area: Rect, app: &App) {
    let status_text = if app.search_mode {
        format!(
            "Search: {}_ (*/? globs, +col:val, -col: filters)",
            app.search_query
        )
    } else if let Some(ref err) = app.error {
        format!("Error: {err}")
    } else {
        let sort_info = app
            .sort_column_name()
            .map(|n| format!(" [sort:{}{}]", n, app.sort_direction.symbol()))
            .unwrap_or_default();
        let expand_info = app
            .expanded_column
            .and_then(|i| app.columns.get(i))
            .map(|c| format!(" [expand:{}]", c.name))
            .unwrap_or_default();
        format!(
            "{}{}{} | f:Find  h/l:Col  s:Sort  e:Expand  q:Quit",
            app.status, sort_info, expand_info
        )
    };

    let style = if app.error.is_some() {
        Style::default().fg(Color::Red)
    } else if app.search_mode {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };

    let status = Paragraph::new(status_text).style(style);
    frame.render_widget(status, area);
}
