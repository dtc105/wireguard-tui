use crate::{
    app::App,
    dto::{Client, ClientForm},
};

use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    widgets::{Block, BorderType, Clear, Paragraph},
};
use std::io::Result;

pub enum Modal {
    CreateClient(ClientForm),
    EditClient(ClientForm),
    DeleteClient,
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

impl Modal {
    pub fn create_field<'a>(title: &'static str, value: &String) -> Paragraph<'a> {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title);

        Paragraph::new(value.clone()).block(block)
    }

    fn draw_user_form(title: &str, frame: &mut Frame, form: &ClientForm) {
        let area = center(frame.area(), Constraint::Length(64), Constraint::Length(12));

        let footer = Paragraph::new("(Esc) exit | (Tab) next | (Shift + Tab) previous")
            .alignment(Alignment::Center);

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title);

        let inner_sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(block.inner(area));

        frame.render_widget(Clear, area);
        frame.render_widget(block, area);
        frame.render_widget(footer, inner_sections[3]);

        let name_field = Modal::create_field("Name", &form.values.name);
        let address_field = Modal::create_field("Address", &form.values.address);
        let public_key_field = Modal::create_field("Public Key", &form.values.public_key);

        frame.render_widget(name_field, inner_sections[0]);
        frame.render_widget(address_field, inner_sections[1]);
        frame.render_widget(public_key_field, inner_sections[2]);
    }

    pub fn draw_delete(frame: &mut Frame) {
        let area = center(frame.area(), Constraint::Length(16), Constraint::Length(4));

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Delete Client");

        let message = Paragraph::new("Confirm? y/N")
            .alignment(Alignment::Center)
            .block(block);

        frame.render_widget(Clear, area);
        frame.render_widget(message, area);
    }

    pub fn draw(&self, frame: &mut Frame) {
        match self {
            Modal::CreateClient(form) => Modal::draw_user_form("Add Client", frame, form),
            Modal::EditClient(form) => Modal::draw_user_form("Edit Client", frame, form),
            Modal::DeleteClient => Modal::draw_delete(frame),
        };
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) -> Option<Result<()>> {
        if let Some(modal) = &mut app.modal {
            match modal {
                Modal::CreateClient(form) | Modal::EditClient(form) => match key.code {
                    KeyCode::Esc => app.modal = None,
                    KeyCode::Enter => (),
                    KeyCode::Backspace => form.backspace(),
                    KeyCode::Tab => form.next_field(),
                    KeyCode::BackTab => form.prev_field(),
                    KeyCode::Char(chr) => form.add_char(chr),
                    _ => (),
                },
                Modal::DeleteClient => match key.code {
                    KeyCode::Esc | KeyCode::Enter | KeyCode::Char('n') => app.modal = None,
                    KeyCode::Char('y') => (),
                    _ => (),
                },
            }
        }

        None
    }
}
