use crate::{app::App, dto::ClientForm};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{
        Alignment,
        Constraint,
        Direction,
        Flex,
        Layout,
        Rect
    },
    style::Style,
    text::Text,
    widgets::{
        Block,
        BorderType,
        Cell,
        Clear,
        HighlightSpacing,
        List,
        ListItem,
        ListState,
        Paragraph,
        Row,
        StatefulWidget,
        Table,
        TableState,
        Widget
    },
    Frame
};
use std::io::Result;

pub enum Modal<'a> {
    CreateClient(ClientForm<'a>),
    EditClient(ClientForm<'a>),
    DeleteClient
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

impl<'a> Modal<'a> {
    pub fn create_field(title: &'a str, value: &String) -> Paragraph<'a> {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title);

        Paragraph::new(value.clone())
            .block(block)
    }

    pub fn draw(&self, frame: &mut Frame) {
        let area = center(
            frame.area(),
            Constraint::Length(32),
            Constraint::Length(12)
        );

        let title = match self {
            Modal::CreateClient(form) | Modal::EditClient(form) => form.title,
            Modal::DeleteClient => "Delete"
        };

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title);

        let inner_sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3)
            ])
            .split(block.inner(area));

        frame.render_widget(Clear, area);
        frame.render_widget(block, area);

        let form = match &self {
            Modal::CreateClient(form) | Modal::EditClient(form) => form,
            _ => return
        };

        let name_field = Modal::create_field("Name", &form.values.name);
        let address_field = Modal::create_field("Address", &form.values.address);
        let public_key_field = Modal::create_field("Public Key", &form.values.public_key);

        frame.render_widget(name_field, inner_sections[0]);
        frame.render_widget(address_field, inner_sections[1]);
        frame.render_widget(public_key_field, inner_sections[2]);
    }

    pub fn handle_key(app: &mut App, key: KeyEvent) -> Option<Result<()>> {
        if let Some(modal) = &mut app.modal {
            match modal {
                Modal::CreateClient(form) | Modal::EditClient(form) => {
                    match key.code {
                        KeyCode::Esc => app.modal = None,
                        KeyCode::Enter => (),
                        KeyCode::Backspace => form.backspace(),
                        KeyCode::Tab => form.next_field(),
                        KeyCode::BackTab => form.prev_field(),
                        KeyCode::Char(chr) => form.add_char(chr),
                        _ => ()
                    }
                },
                Modal::DeleteClient => {}
            }
        }

        None
    }
}