//! Map view rendering.

use crate::tui::app::App;
use crate::tui::text::truncate;
use crate::tui::types::{MapLocation, MapLocationType};
use crate::tui::widgets::format_value;
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Circle, Line as CanvasLine},
        Block, Borders, Cell, Paragraph, Row, Table,
    },
};

/// Render the map view with system visualization and hotspot details.
pub fn render_map(frame: &mut Frame, app: &App, area: Rect) {
    // Split the area into map canvas (left) and hotspot details (right)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(area);

    render_system_canvas(frame, app, chunks[0]);
    render_hotspot_details(frame, app, chunks[1]);
}

/// Convert a location to its display (x, y) coordinates using angle and orbital radius.
fn location_to_xy(
    loc: &MapLocation,
    planet_positions: &std::collections::HashMap<String, (f64, f64)>,
) -> (f64, f64) {
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
#[allow(clippy::too_many_lines)]

fn render_system_canvas(frame: &mut Frame, app: &App, area: Rect) {
    // First pass: calculate planet positions (they orbit the star)
    let mut planet_positions: std::collections::HashMap<String, (f64, f64)> =
        std::collections::HashMap::new();

    for loc in &app.map_locations {
        if loc.loc_type == MapLocationType::Planet {
            let x = loc.orbital_radius * loc.angle.cos();
            let y = loc.orbital_radius * loc.angle.sin();
            planet_positions.insert(loc.name.clone(), (x, y));
        }
    }

    // Second pass: calculate moon positions (they orbit planets)
    let mut moon_positions: std::collections::HashMap<String, (f64, f64)> =
        std::collections::HashMap::new();
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
                .title(format!(
                    " {} System Map | {} | {}% zoom ",
                    app.map_system, hotspot_info, zoom_pct
                ))
                .title_bottom(Line::from(vec![
                    Span::raw(" "),
                    Span::styled("n", Style::default().fg(Color::Yellow)),
                    Span::raw("/"),
                    Span::styled("N", Style::default().fg(Color::Yellow)),
                    Span::raw(":limit │ "),
                    Span::styled("z", Style::default().fg(Color::Yellow)),
                    Span::raw("/"),
                    Span::styled("Z", Style::default().fg(Color::Yellow)),
                    Span::raw(":zoom │ "),
                    Span::styled("a", Style::default().fg(Color::Yellow)),
                    Span::raw(":toggle all │ "),
                    Span::styled("h", Style::default().fg(Color::Yellow)),
                    Span::raw("/"),
                    Span::styled("l", Style::default().fg(Color::Yellow)),
                    Span::raw(":navigate "),
                ]))
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

                ctx.draw(&Circle {
                    x,
                    y,
                    radius,
                    color,
                });

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

#[allow(clippy::too_many_lines)]
fn render_hotspot_details(frame: &mut Frame, app: &App, area: Rect) {
    if app.detail_expanded {
        // Expanded view - show full route list
        render_hotspot_details_expanded(frame, app, area);
    } else {
        // Compact view - show summary info
        render_hotspot_details_compact(frame, app, area);
    }
}

/// Render compact hotspot details (original view)
#[allow(clippy::too_many_lines)]
fn render_hotspot_details_compact(frame: &mut Frame, app: &App, area: Rect) {
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
                // Indicator if this is a cross-system corridor
                if hotspot.is_cross_system {
                    Span::styled(" (cross-system)", Style::default().fg(Color::DarkGray))
                } else {
                    Span::raw("")
                },
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
                // Warning for armistice zones
                if hotspot.jump_to.exit_at_mm < 100 {
                    Span::styled(
                        " ⚠ ARMISTICE ZONE!",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    )
                } else if hotspot.jump_to.exit_at_mm < 500 {
                    Span::styled(" (⚠ UEE monitored)", Style::default().fg(Color::Yellow))
                } else {
                    Span::raw("")
                },
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
                .title_bottom(" ⓘ Exit distances are approximate ")
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
                    Cell::from(format_value(route.cargo_value))
                        .style(Style::default().fg(Color::Green)),
                    Cell::from(format!("{}", route.threat_level))
                        .style(Style::default().fg(threat_color)),
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

/// Render expanded hotspot details with full scrollable route list
#[allow(clippy::too_many_lines)]
fn render_hotspot_details_expanded(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(0)])
        .split(area);

    // Hotspot summary (condensed)
    if let Some(hotspot) = app.hotspots.get(app.map_selected) {
        let info_text = vec![
            Line::from(vec![
                Span::styled("Zone: ", Style::default().fg(Color::Yellow)),
                Span::raw(&hotspot.name),
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
            Line::from(vec![
                Span::styled("Jump: ", Style::default().fg(Color::Cyan)),
                Span::raw(format!(
                    "{} @ {} Mm",
                    hotspot.jump_to.destination, hotspot.jump_to.exit_at_mm
                )),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Press ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    "Enter",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" to collapse", Style::default().fg(Color::DarkGray)),
            ]),
        ];

        let info = Paragraph::new(info_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Hotspot Details (EXPANDED) ")
                .title_style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
        );
        frame.render_widget(info, chunks[0]);

        // Full scrollable route list
        let routes = &hotspot.intersecting_routes;
        let rows: Vec<Row> = routes
            .iter()
            .enumerate()
            .map(|(i, route)| {
                let threat_color = match route.threat_level {
                    0..=2 => Color::Green,
                    3..=5 => Color::Yellow,
                    _ => Color::Red,
                };

                let style = if i == app.detail_selected {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                };

                Row::new(vec![
                    Cell::from(truncate(&route.origin, 20)),
                    Cell::from(truncate(&route.destination, 20)),
                    Cell::from(truncate(&route.commodity, 12)),
                    Cell::from(truncate(&route.ship_name, 15)),
                    Cell::from(format_value(route.cargo_value))
                        .style(Style::default().fg(Color::Green)),
                    Cell::from(format!("{}", route.threat_level))
                        .style(Style::default().fg(threat_color)),
                    Cell::from(format_value(route.interdiction_value))
                        .style(Style::default().fg(Color::Cyan)),
                ])
                .style(style)
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(20), // Origin
                Constraint::Length(20), // Destination
                Constraint::Length(12), // Commodity
                Constraint::Length(15), // Ship
                Constraint::Length(10), // Value
                Constraint::Length(6),  // Threat
                Constraint::Length(10), // Interdict Value
            ],
        )
        .header(
            Row::new(vec![
                "Origin",
                "Destination",
                "Cargo",
                "Ship",
                "Value",
                "Thr",
                "Int Value",
            ])
            .style(Style::default().fg(Color::Yellow))
            .bottom_margin(1),
        )
        .block(Block::default().borders(Borders::ALL).title(format!(
            " All Routes ({}/{}) - j/k to scroll ",
            app.detail_selected + 1,
            routes.len()
        )));

        frame.render_widget(table, chunks[1]);
    } else {
        let empty = Paragraph::new("No hotspot selected").block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Hotspot Details "),
        );
        frame.render_widget(empty, area);
    }
}
