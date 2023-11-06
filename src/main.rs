pub mod app;
pub mod game;
pub mod event;
pub mod ui;
pub mod tui;
pub mod handler;

use anyhow::Result;
use app::App;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use handler::*;

fn main() -> Result<()> {
  // Create an application.
  let mut app = App::new();

  // Initialize the terminal user interface.
  let backend = CrosstermBackend::new(std::io::stderr());
  let terminal = Terminal::new(backend)?;
  let events = EventHandler::new(250);
  let mut tui = Tui::new(terminal, events);
  tui.enter()?;

  // Start the main loop.
  while !app.should_quit {
    // Render the user interface.
    tui.draw(&mut app)?;
    // Handle events.
    match tui.events.next()? {
      Event::Tick => {},
      Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
      Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app)?,
      Event::Resize(_, _) => {},
    };
  }

  // Exit the user interface.
  tui.exit()?;
  Ok(())
}
