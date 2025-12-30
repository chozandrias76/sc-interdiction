//! UI rendering for the TUI.

use super::app::App;
use super::text::{scroll_text, truncate};
use super::types::{MapLocation, MapLocationType, RouteSort, TargetSort, View};
use intel::TrafficDirection;
use ratatui::{
    prelude::*,
    widgets::{
        Block, Borders, Cell, Clear, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, Tabs, Wrap, canvas::{Canvas, Circle, Line as CanvasLine},
    },
};

/// Render the application.
pub fn render(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header/tabs
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Status bar
        ])
        .split(frame.area());

    render_header(frame, app, chunks[0]);

    match app.view {
        View::Targets => render_targets(frame, app, chunks[1]),
        View::Routes => render_routes(frame, app, chunks[1]),
        View::Map => render_map(frame, app, chunks[1]),
        View::Help => render_help(frame, chunks[1]),
    }

    render_status_bar(frame, app, chunks[2]);
}

fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let titles = vec!["[1] Targets", "[2] Routes", "[3] Map"];
    let selected = match app.view {
        View::Targets => 0,
        View::Routes => 1,
        View::Map => 2,
        View::Help => 0,
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

fn render_targets(frame: &mut Frame, app: &mut App, area: Rect) {
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
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    " Targets ({} shown){}{} ",
                    filtered.len(),
                    sort_indicator,
                    filter_info
                )),
        )
        .row_highlight_style(Style::default().add_modifier(Modifier::BOLD));

    frame.render_widget(table, area);

    // Scrollbar
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    let mut scrollbar_state =
        ScrollbarState::new(filtered.len()).position(app.selected);
    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin::new(0, 1)),
        &mut scrollbar_state,
    );
}

fn render_routes(frame: &mut Frame, app: &mut App, area: Rect) {
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
                .title(format!(" Hot Routes ({} total){} ", app.routes.len(), sort_indicator)),
        );

    frame.render_widget(table, area);

    // Scrollbar
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    let mut scrollbar_state =
        ScrollbarState::new(app.routes.len()).position(app.selected);
    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin::new(0, 1)),
        &mut scrollbar_state,
    );
}

fn render_map(frame: &mut Frame, app: &App, area: Rect) {
    // Split the area into map canvas (left) and hotspot details (right)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(area);

    render_system_canvas(frame, app, chunks[0]);
    render_hotspot_details(frame, app, chunks[1]);
}

/// Convert a location to its display (x, y) coordinates using angle and orbital radius.
fn location_to_xy(loc: &MapLocation, planet_positions: &std::collections::HashMap<String, (f64, f64)>) -> (f64, f64) {
    let (base_x, base_y) = if let Some(parent) = &loc.parent {
        // Get parent's position
        planet_positions.get(parent).copied().unwrap_or((0.0, 0.0))
    } else {
        (0.0, 0.0) // Orbits the star
    };

    // Calculate position from angle and radius
    let x = base_x + loc.orbital_radius * loc.angle.cos();
    let y = base_y + loc.orbital_radius * loc.angle.sin();
    (x, y)
}

fn render_system_canvas(frame: &mut Frame, app: &App, area: Rect) {
    // First pass: calculate planet positions (they orbit the star)
    let mut planet_positions: std::collections::HashMap<String, (f64, f64)> = std::collections::HashMap::new();

    for loc in &app.map_locations {
        if loc.loc_type == MapLocationType::Planet {
            let x = loc.orbital_radius * loc.angle.cos();
            let y = loc.orbital_radius * loc.angle.sin();
            planet_positions.insert(loc.name.clone(), (x, y));
        }
    }

    // Second pass: calculate moon positions (they orbit planets)
    let mut moon_positions: std::collections::HashMap<String, (f64, f64)> = std::collections::HashMap::new();
    for loc in &app.map_locations {
        if loc.loc_type == MapLocationType::Moon {
            if let Some(parent) = &loc.parent {
                if let Some(&(px, py)) = planet_positions.get(parent) {
                    let x = px + loc.orbital_radius * loc.angle.cos();
                    let y = py + loc.orbital_radius * loc.angle.sin();
                    moon_positions.insert(loc.name.clone(), (x, y));
                }
            }
        }
    }

    // Merge positions
    let mut all_positions = planet_positions.clone();
    all_positions.extend(moon_positions.clone());

    // Calculate bounds - include both map locations AND hotspots
    let mut min_x = -5.0_f64;
    let mut max_x = 5.0_f64;
    let mut min_y = -5.0_f64;
    let mut max_y = 5.0_f64;

    for loc in &app.map_locations {
        let (x, y) = location_to_xy(loc, &all_positions);
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    // Include visible hotspot positions in bounds (using actual x,y coordinates)
    for hotspot in app.visible_hotspots() {
        let x = hotspot.position.x;
        let y = hotspot.position.y;
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    // Add generous padding and make it square-ish
    let data_range = (max_x - min_x).max(max_y - min_y);
    let padding = data_range * 0.15 + 10.0; // 15% padding plus fixed margin
    let range = (data_range / 2.0 + padding) / app.map_zoom;
    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;

    // Clone data for the closure - only take visible hotspots
    let map_locations = app.map_locations.clone();
    let hotspots: Vec<_> = app.visible_hotspots().cloned().collect();
    let map_selected = app.map_selected;
    let positions = all_positions.clone();
    let hotspot_limit = app.hotspot_limit;
    let total_hotspots = app.hotspots.len();

    let zoom_pct = (app.map_zoom * 100.0) as u32;
    let hotspot_info = if hotspot_limit == total_hotspots {
        format!("all {} hotspots", total_hotspots)
    } else {
        format!("top {} of {} hotspots", hotspot_limit, total_hotspots)
    };
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} System Map | {} | {}% zoom | n/N:limit z/Z:zoom a:all ",
                    app.map_system, hotspot_info, zoom_pct))
                .title_style(Style::default().fg(Color::Cyan).bold()),
        )
        .x_bounds([center_x - range, center_x + range])
        .y_bounds([center_y - range, center_y + range])
        .paint(move |ctx| {
            // Draw star at center
            ctx.draw(&Circle {
                x: 0.0,
                y: 0.0,
                radius: 2.0,
                color: Color::Yellow,
            });

            // Draw orbital rings for planets (faint circles)
            for loc in &map_locations {
                if loc.loc_type == MapLocationType::Planet && loc.parent.is_none() {
                    // Draw orbit circle - approximate with line segments
                    let segments = 32;
                    for i in 0..segments {
                        let a1 = 2.0 * std::f64::consts::PI * i as f64 / segments as f64;
                        let a2 = 2.0 * std::f64::consts::PI * (i + 1) as f64 / segments as f64;
                        ctx.draw(&CanvasLine {
                            x1: loc.orbital_radius * a1.cos(),
                            y1: loc.orbital_radius * a1.sin(),
                            x2: loc.orbital_radius * a2.cos(),
                            y2: loc.orbital_radius * a2.sin(),
                            color: Color::DarkGray,
                        });
                    }
                }
            }

            // Draw locations
            for loc in &map_locations {
                let (x, y) = location_to_xy(loc, &positions);

                let (color, radius) = match loc.loc_type {
                    MapLocationType::Star => continue, // Already drawn
                    MapLocationType::Planet => (Color::Cyan, 1.5),
                    MapLocationType::Moon => (Color::Gray, 0.8),
                    MapLocationType::Station => (Color::Green, 0.5),
                };

                ctx.draw(&Circle { x, y, radius, color });

                // Label for planets
                if loc.loc_type == MapLocationType::Planet {
                    ctx.print(
                        x + 2.0,
                        y + 0.5,
                        Span::styled(loc.name.clone(), Style::default().fg(Color::White)),
                    );
                }
            }

            // Draw hotspots at their actual positions in space
            // This shows the real location of route intersections
            for (i, hotspot) in hotspots.iter().enumerate() {
                let color = if i == map_selected {
                    Color::LightRed
                } else {
                    Color::Red
                };

                // Use actual x,y position (ignore z for 2D map)
                // Positions are in Mkm from the star
                let x = hotspot.position.x;
                let y = hotspot.position.y;

                ctx.draw(&Circle {
                    x,
                    y,
                    radius: if i == map_selected { 1.5 } else { 1.0 },
                    color,
                });

                // Draw crosshairs for selected
                if i == map_selected {
                    ctx.draw(&CanvasLine {
                        x1: x - 3.0,
                        y1: y,
                        x2: x + 3.0,
                        y2: y,
                        color: Color::LightRed,
                    });
                    ctx.draw(&CanvasLine {
                        x1: x,
                        y1: y - 3.0,
                        x2: x,
                        y2: y + 3.0,
                        color: Color::LightRed,
                    });
                }
            }
        });

    frame.render_widget(canvas, area);
}

fn render_hotspot_details(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(12), Constraint::Min(0)])
        .split(area);

    // Hotspot info panel
    if let Some(hotspot) = app.hotspots.get(app.map_selected) {
        let info_text = vec![
            Line::from(vec![
                Span::styled("Zone: ", Style::default().fg(Color::Yellow)),
                Span::raw(&hotspot.name),
            ]),
            Line::from(vec![
                Span::styled("Position: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!(
                    "({:.1}, {:.1}) Mkm",
                    hotspot.position.x, hotspot.position.y
                )),
            ]),
            Line::from(vec![
                Span::styled("Routes: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{}", hotspot.route_pair_count)),
            ]),
            Line::from(vec![
                Span::styled("Value: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    format_value(hotspot.interdiction_value),
                    Style::default().fg(Color::Green),
                ),
                Span::raw("/catch"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Jump to: ", Style::default().fg(Color::Cyan)),
                Span::raw(&hotspot.jump_to.destination),
            ]),
            Line::from(vec![
                Span::styled("Exit at: ", Style::default().fg(Color::Cyan)),
                Span::raw(format!("{} Mm", hotspot.jump_to.exit_at_mm)),
            ]),
            Line::from(vec![
                Span::styled("Lateral: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{:.0} km", hotspot.jump_to.lateral_offset_km),
                    if hotspot.jump_to.lateral_offset_km < 20.0 {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::Yellow)
                    },
                ),
                if hotspot.jump_to.lateral_offset_km < 20.0 {
                    Span::styled(" (direct hit!)", Style::default().fg(Color::Green))
                } else {
                    Span::raw("")
                },
            ]),
        ];

        let visible_count = app.visible_hotspot_count();
        let info = Paragraph::new(info_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    " Hotspot {}/{} (h/l to navigate) ",
                    app.map_selected + 1,
                    visible_count
                ))
                .title_style(Style::default().fg(Color::Cyan)),
        );
        frame.render_widget(info, chunks[0]);
    } else {
        let empty = Paragraph::new("No hotspots found").block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Hotspot Details ")
                .title_style(Style::default().fg(Color::Cyan)),
        );
        frame.render_widget(empty, chunks[0]);
    }

    // Route list
    if let Some(hotspot) = app.hotspots.get(app.map_selected) {
        let rows: Vec<Row> = hotspot
            .intersecting_routes
            .iter()
            .take(10)
            .map(|route| {
                let threat_color = match route.threat_level {
                    0..=2 => Color::Green,
                    3..=5 => Color::Yellow,
                    _ => Color::Red,
                };
                Row::new(vec![
                    Cell::from(truncate(&route.commodity, 15)),
                    Cell::from(format_value(route.cargo_value)).style(Style::default().fg(Color::Green)),
                    Cell::from(format!("{}", route.threat_level)).style(Style::default().fg(threat_color)),
                ])
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Min(10),
                Constraint::Length(10),
                Constraint::Length(6),
            ],
        )
        .header(
            Row::new(vec!["Cargo", "Value", "Threat"])
                .style(Style::default().fg(Color::Yellow))
                .bottom_margin(1),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Intersecting Routes "),
        );

        frame.render_widget(table, chunks[1]);
    }
}

fn render_help(frame: &mut Frame, area: Rect) {
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

fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let status = if let Some(ref error) = app.error {
        Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
    } else {
        Paragraph::new(app.status.as_str())
            .style(Style::default().fg(Color::Gray))
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
fn format_value(value: f64) -> String {
    if value >= 1_000_000.0 {
        format!("{:.2}M", value / 1_000_000.0)
    } else if value >= 1_000.0 {
        format!("{:.1}K", value / 1_000.0)
    } else {
        format!("{:.0}", value)
    }
}

/// Create a centered rectangle for popups.
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::text::ScrollState;
    use insta::assert_snapshot;
    use ratatui::{backend::TestBackend, Terminal};

    /// Create a test app with sample data for snapshot testing.
    fn test_app() -> App {
        use intel::{CargoShip, HotRoute, TargetPrediction, TrafficDirection};

        let ship = CargoShip {
            name: "Caterpillar",
            manufacturer: "Drake",
            cargo_scu: 576,
            crew_size: 4,
            threat_level: 3,
            ship_value_uec: 600_000,
            requires_freight_elevator: false,
        };

        let targets = vec![
            TargetPrediction {
                commodity: "Quantanium".to_string(),
                destination: "Area18 TDD".to_string(),
                likely_ship: ship.clone(),
                direction: TrafficDirection::Departing,
                estimated_cargo_value: 1_250_000.0,
            },
            TargetPrediction {
                commodity: "Laranite".to_string(),
                destination: "Port Olisar".to_string(),
                likely_ship: CargoShip {
                    name: "C2 Hercules",
                    manufacturer: "Crusader",
                    cargo_scu: 696,
                    crew_size: 3,
                    threat_level: 8,
                    ship_value_uec: 4_800_000,
                    requires_freight_elevator: true,
                },
                direction: TrafficDirection::Arriving,
                estimated_cargo_value: 980_000.0,
            },
        ];

        let routes = vec![
            HotRoute {
                origin: "HDMS-Bezdek".to_string(),
                origin_system: "Stanton".to_string(),
                destination: "Area18 TDD".to_string(),
                destination_system: "Stanton".to_string(),
                commodity: "Quantanium".to_string(),
                commodity_code: "QUAN".to_string(),
                profit_per_scu: 88.5,
                available_scu: 576.0,
                estimated_haul_value: 51_000.0,
                likely_ship: ship.clone(),
                risk_score: 75.0,
            },
        ];

        App {
            view: View::Targets,
            location: "Crusader".to_string(),
            targets,
            routes,
            hotspots: Vec::new(),
            map_locations: Vec::new(),
            map_system: "Stanton".to_string(),
            map_selected: 0,
            map_zoom: 1.0,
            hotspot_limit: 5,
            selected: 0,
            filter_inbound: false,
            filter_outbound: false,
            min_threat: 0,
            target_sort: TargetSort::Value,
            route_sort: RouteSort::Profit,
            sort_asc: false,
            loading: false,
            error: None,
            status: "Ready".to_string(),
            scroll: ScrollState::new(),
        }
    }

    #[test]
    fn test_render_targets_view() {
        let mut app = test_app();
        app.view = View::Targets;

        let mut terminal = Terminal::new(TestBackend::new(100, 25)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_routes_view() {
        let mut app = test_app();
        app.view = View::Routes;

        let mut terminal = Terminal::new(TestBackend::new(100, 25)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_help_view() {
        let mut app = test_app();
        app.view = View::Help;

        let mut terminal = Terminal::new(TestBackend::new(100, 25)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_with_selection() {
        let mut app = test_app();
        app.view = View::Targets;
        app.selected = 1; // Select second row

        let mut terminal = Terminal::new(TestBackend::new(100, 25)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_with_error() {
        let mut app = test_app();
        app.error = Some("Connection failed: timeout".to_string());

        let mut terminal = Terminal::new(TestBackend::new(100, 25)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_loading_state() {
        let mut app = test_app();
        app.loading = true;
        app.status = "Loading data...".to_string();

        let mut terminal = Terminal::new(TestBackend::new(100, 25)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_snapshot!(terminal.backend());
    }
}
