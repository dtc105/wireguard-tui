use crate::{
    dto::{Client, ClientForm, Clients, Logs},
    modal::Modal,
    ui,
};

use ratatui::{
    Frame, Terminal,
    backend::Backend,
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read},
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::Text,
    widgets::{Block, BorderType, Cell, HighlightSpacing, List, ListItem, Paragraph, Row, Table},
};
use std::io::Result;

enum Focus {
    Client,
    Log,
}

pub struct App {
    focus: Focus,
    pub modal: Option<Modal>,
    pub clients: Clients,
    pub logs: Logs,
}

impl App {
    pub fn new() -> Self {
        Self {
            focus: Focus::Client,
            modal: None,
            clients: Clients::init(),
            logs: Logs::dummy(),
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> Option<Result<()>> {
        if self.modal.is_some() {
            return Modal::handle_key(self, key);
        }

        match &self.focus {
            Focus::Client => {
                let client = if let Some(index) = self.clients.state.selected() {
                    Some(self.clients.data[index].clone())
                } else {
                    None
                };

                match key.code {
                    KeyCode::Esc => return Some(Ok(())),
                    KeyCode::Tab => {
                        self.clients.state.select(None);
                        self.logs.state.select_first();
                        self.focus = Focus::Log;
                    }
                    KeyCode::Char('g') => self.clients.state.select_first(),
                    KeyCode::Char('G') => self.clients.state.select_last(),
                    KeyCode::Char('j') | KeyCode::Down => self.clients.state.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.clients.state.select_previous(),
                    KeyCode::Char('a') => self.modal = Some(Modal::CreateClient(ClientForm::new())),
                    KeyCode::Char('e') => {
                        if client.is_some() {
                            self.modal =
                                Some(Modal::EditClient(ClientForm::from_client(client.unwrap())))
                        }
                    }
                    KeyCode::Char('d') => self.modal = Some(Modal::DeleteClient),
                    _ => (),
                };
            }
            Focus::Log => {
                match key.code {
                    KeyCode::Esc => return Some(Ok(())),
                    KeyCode::Tab => {
                        self.logs.state.select(None);
                        self.clients.state.select_first();
                        self.focus = Focus::Log;
                    }
                    KeyCode::Char('g') => self.logs.state.select_first(),
                    KeyCode::Char('G') => self.logs.state.select_last(),
                    KeyCode::Char('j') | KeyCode::Down => self.logs.state.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.logs.state.select_previous(),
                    _ => (),
                };
            }
        };

        None
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|frame| ui::draw(&mut *self, frame))?;

            if let Event::Key(key) = read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        return Ok(());
                    }

                    if let Some(exit) = self.handle_key(key) {
                        return exit;
                    }
                }
            }
        }
    }
}
