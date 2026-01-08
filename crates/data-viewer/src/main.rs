//! Data Viewer - TUI for browsing `PostgreSQL` game data with medallion schemas.

mod app;
mod db;
mod event;
mod types;
mod ui;
mod views;

use app::App;
use clap::Parser;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use event::{Event, EventHandler};
use eyre::Result;
use ratatui::prelude::*;
use std::io::{self, stdout};

/// TUI for browsing `PostgreSQL` game data with medallion architecture.
#[derive(Parser, Debug)]
#[command(name = "data-viewer")]
#[command(about = "Browse PostgreSQL game data in a TUI (raw/silver/gold schemas)")]
struct Args {
    /// `PostgreSQL` database URL. If not provided, uses `DATABASE_URL` environment variable.
    #[arg(short, long, env = "DATABASE_URL")]
    url: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize terminal
    let mut terminal = init()?;

    // Create app
    let result = run(&mut terminal, args.url.as_deref());

    // Restore terminal
    restore()?;

    result
}

/// Initialize the terminal for TUI mode.
fn init() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore terminal to normal mode.
fn restore() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

/// Run the main event loop.
fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, db_url: Option<&str>) -> Result<()> {
    let mut app = App::new(db_url)?;
    let events = EventHandler::new(250);

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        match events.next()? {
            Event::Tick => app.on_tick(),
            Event::Key(key) => {
                if app.on_key(key) {
                    break;
                }
            }
            Event::Mouse | Event::Resize => {}
        }
    }

    Ok(())
}
