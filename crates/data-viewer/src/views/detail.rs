//! Record detail popup view.

use crate::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

/// Render the record detail popup.
pub fn render_detail(frame: &mut Frame, app: &App) {
    let area = centered_rect(70, 80, frame.area());

    // Clear the background
    frame.render_widget(Clear, area);

    let Some(record) = app.selected_record() else {
        return;
    };

    // Build content with field names and values
    let mut lines: Vec<Line> = Vec::new();

    for (i, col) in app.columns.iter().enumerate() {
        let value = record.get(i).map(|s| s.as_str()).unwrap_or("<missing>");
        let line = Line::from(vec![
            Span::styled(
                format!("{}: ", col.name),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(value),
        ]);
        lines.push(line);
    }

    let title = if let Some(name) = app.current_table_display() {
        format!(" Record Detail - {} ", name)
    } else {
        " Record Detail ".to_string()
    };

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .title(title)
                .title_bottom(" [Esc] Close ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

/// Create a centered rectangle.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let [_, middle_row, _] = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .areas(r);

    let [_, center, _] = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .areas(middle_row);

    center
}
