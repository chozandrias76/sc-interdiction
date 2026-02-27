# CLI Crate (sc-interdiction)

Binary crate. Clap CLI + Ratatui TUI dashboard for interdiction planning.

## Entrypoint

- `src/main.rs` -- Clap parser, subcommand dispatch, table printing (744 lines)
- Binary name: `sc-interdiction` (defined in Cargo.toml `[[bin]]`)
- All handlers are async fns in main.rs (not separated into modules)

## Subcommands

| Command       | Description                              |
|---------------|------------------------------------------|
| `serve`       | Start Axum REST API server               |
| `routes`      | Hot trade routes (best targets)          |
| `runs`        | Round-trip trade runs with return cargo  |
| `chokepoints` | Interdiction chokepoints                 |
| `intel`       | Target predictions at a location         |
| `ships`       | Static cargo ship database               |
| `fleet-ships` | FleetYards API ship specs (cached)       |
| `terminals`   | List terminals (filterable by system)    |
| `nearby`      | Nearest hotspots to a location           |
| `distance`    | Distance between two locations           |
| `locations`   | Known locations in a system              |
| `dashboard`   | Launch interactive TUI                   |

Global flags: `--api-key`, `--verbose`, `--json` (per-command).

## TUI Layout (src/tui/)

```
tui/
  mod.rs          -- Terminal init/restore, run loop
  app.rs          -- App state (view, filters, sort, scroll)
  ui.rs           -- Top-level render: header | content | status bar
  types.rs        -- View, TargetSort, RouteSort, MapLocation enums
  event.rs        -- Crossterm event handler (tick, key, mouse)
  text.rs         -- ScrollState for text display
  widgets.rs      -- Header tabs, status bar widgets
  handlers/       -- Key/navigation/sorting input handlers
  views/          -- View renderers: targets, routes, map, help
  data/           -- Data loading (hotspots, map locations)
```

Views: Targets (default), Routes, Map (system visualization), Help.
Navigation: Tab/number keys switch views. Arrow keys scroll/select.

## Snapshot Tests

Uses `insta` for TUI snapshot testing. 13 snapshots in:
`src/tui/snapshots/`

Test module lives in `src/tui/ui.rs` (mod tests). Uses `TestBackend`
to render frames and `assert_snapshot!` to verify output.

Snapshots cover: help view, loading state, map view, routes view,
targets view, detail expansion, wikelo overlays, error display, selection.

Update snapshots: `cargo insta review -p sc-interdiction`

## Commands

```bash
cargo test -p sc-interdiction          # Run all tests (including snapshots)
cargo run -p sc-interdiction -- --help # Show CLI help
cargo run -p sc-interdiction -- dashboard --location Crusader
```

## Dependencies

Workspace crates: `api-client`, `route-graph`, `intel`, `server`,
`sc-data-extractor`. External: `clap`, `ratatui`, `crossterm`,
`tokio`, `tracing`, `serde_json`, `dirs`, `eyre`.

## Notes

- main.rs is 744 lines (over 500-line limit) -- print_* fns could extract to output module
- Handler pattern: build client -> call analyzer -> print table or JSON
