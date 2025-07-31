use crate::{
    focus::Focus,
    pages::home::Home
};

use ratatui::{
    backend::Backend,
    crossterm::event::{
        read,
        Event,
        KeyCode,
        KeyEvent,
        KeyEventKind,
        KeyModifiers
    },
    Frame,
    Terminal
};
use std::error::Error;

pub struct App {
    pub focus: Focus,
    pub home: Home        // Need to organize this better for multiple pages
}

impl App {
    pub fn new() -> Self {
        Self {
            focus: Focus::new(),
            home: Home::new()
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        todo!();
        match self.focus {
            Focus::Home(_) => frame.render_widget(&mut self.home, frame.area()),
            Focus::CreateForm(_) => {},
            Focus::EditForm(_) => {}
        };
    }

    fn handle_key(&mut self, key: KeyEvent) {
        todo!();
        match self.focus {
            Focus::Home(_) => {},
            Focus::CreateForm(_) => {},
            Focus::EditForm(_) => {}
        };
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('c') &&
                       key.modifiers.contains(KeyModifiers::CONTROL) {
                        return Ok(());
                    }

                    self.handle_key(key);
                }
            }
        }
    }
}
