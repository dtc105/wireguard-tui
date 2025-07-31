use crate::dto::ClientForm;

use ratatui::{
    buffer::Buffer,
    layout::{
        Alignment,
        Constraint,
        Direction,
        Layout,
        Rect
    },
    style::Style,
    text::Text,
    widgets::{
        Block,
        Cell,
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
    }
};

pub struct Form {
    title: String,
    data: ClientForm
}

impl Form {
    pub fn new(title: String) -> Self {
        Self {
            title,
            data: ClientForm {
                name: String::new(),
                address: String::new(),
                public_key: String::new()
            }
        }
    }
    // One of these is not needed?
    pub fn from(
        title: String,
        name: String,
        address: String,
        public_key: String
    ) -> Self {
        Self {
            title,
            data: ClientForm {
                name,
                address,
                public_key
            }
        }
    }

    fn field_block<'a>(title: &'a str, area: Rect) -> Block<'a> {
        Block::bordered().title(title)
    }

    fn render_header(title: String, area: Rect, buffer: &mut Buffer) {
        let block = Block::bordered();

        Paragraph::new(Text::styled(
            title,
            Style::default()
        ))
            .alignment(Alignment::Center)
            .block(block)
            .render(area, buffer);
    }

    fn render_footer(area: Rect, buffer: &mut Buffer) {
        let border_block = Block::bordered();

        Paragraph::new(Text::styled(
            "(Esc) quit | (Tab) next field | (Shift + Tab) prev field | (Enter) submit",
            Style::default()
        ))
            .alignment(Alignment::Center)
            .block(border_block)
            .render(area, buffer);
    }

    fn render_form(form: &Form, area: Rect, buffer: &mut Buffer) {
        todo!();
        let block = Block::bordered().title(form.title);

        let fields = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3)
            ])
            .split(area);

        let name_block = Form::field_block("Name", fields[0]);
    }
}

impl Widget for &mut Form {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3)
            ])
            .split(area);

        Form::render_header(self.title.to_string(), chunks[0], buffer);
        Form::render_form(self, chunks[1], buffer);
        Form::render_footer(chunks[2], buffer);
    }
}