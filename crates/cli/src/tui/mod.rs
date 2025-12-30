//! Terminal User Interface for SC Interdiction.
//!
//! Provides an interactive dashboard for analyzing trade routes and targets.

mod app;
mod data;
mod event;
mod handlers;
mod text;
mod types;
mod ui;
mod views;
mod widgets;

pub use app::App;
pub use event::{Event, EventHandler};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use eyre::Result;
use ratatui::prelude::*;
use std::io::{self, stdout};

/// Initialize the terminal for TUI mode.
pub fn init() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore terminal to normal mode.
pub fn restore() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

/// Run the TUI application.
pub async fn run(location: Option<String>) -> Result<()> {
    let mut terminal = init()?;
    let mut app = App::new(location).await?;
    let events = EventHandler::new(250);

    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        match events.next()? {
            Event::Tick => app.on_tick(),
            Event::Key(key) => {
                if app.on_key(key) {
                    break;
                }
            }
            Event::Mouse => {}
            Event::Resize => {}
        }
    }

    restore()?;
    Ok(())
}
