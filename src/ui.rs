use std::{io::{self, Error, ErrorKind}, time::Duration};

use crossterm::{event::{self, Event, KeyCode, poll}, terminal::enable_raw_mode};
use tui::{Terminal, backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, widgets::{Block, BorderType, Borders}};

use crate::shared_state::SharedState;

pub fn start_up() -> Result<SharedState, Error> {
    let backend = CrosstermBackend::new(io::stdout());
    enable_raw_mode().unwrap();
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    loop {
        terminal.draw(|f| {
            let widget = Block::default().title("Connection").borders(Borders::ALL).border_type(BorderType::Rounded);
            let area = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Length(3)].as_ref()).split(f.size())[0];
            f.render_widget(widget, area);
        })?;
        if poll(Duration::from_millis(1_000)).unwrap() {
            let event = event::read().unwrap();
            match event {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Esc => break,
                        _ => {}
                    }
                }
                _ => {} // Non key events
            }
        }
    }
    Err(Error::new(ErrorKind::Other, "Reached end of ui execution path"))
}