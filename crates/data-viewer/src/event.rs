//! Event handling for the TUI.

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use eyre::Result;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

/// Terminal events.
#[derive(Debug)]
pub enum Event {
    /// Terminal tick (for animations/updates).
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse event (ignored).
    Mouse,
    /// Terminal resize.
    Resize,
}

/// Handles terminal events in a separate thread.
pub struct EventHandler {
    rx: mpsc::Receiver<Event>,
}

impl EventHandler {
    /// Create a new event handler with the given tick rate in milliseconds.
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (tx, rx) = mpsc::channel();
        let sender = tx.clone();

        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(tick_rate);

                let Ok(has_event) = event::poll(timeout) else {
                    return;
                };

                if has_event {
                    let Ok(event) = event::read() else {
                        return;
                    };

                    match event {
                        CrosstermEvent::Key(e) => {
                            if sender.send(Event::Key(e)).is_err() {
                                return;
                            }
                        }
                        CrosstermEvent::Mouse(_) => {
                            if sender.send(Event::Mouse).is_err() {
                                return;
                            }
                        }
                        CrosstermEvent::Resize(_, _) => {
                            if sender.send(Event::Resize).is_err() {
                                return;
                            }
                        }
                        _ => {}
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if sender.send(Event::Tick).is_err() {
                        return;
                    }
                    last_tick = Instant::now();
                }
            }
        });

        Self { rx }
    }

    /// Get the next event, blocking until one is available.
    pub fn next(&self) -> Result<Event> {
        Ok(self.rx.recv()?)
    }
}
