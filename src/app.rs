use crate::{
    dto::{ClientForm, Clients, Logs},
    modal::Modal,
};

use ratatui::{
    backend::Backend,
    buffer::Buffer,
    crossterm::event::{
        read,
        Event,
        KeyCode,
        KeyEvent,
        KeyEventKind,
        KeyModifiers
    },
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
        BorderType,
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
    },
    Frame,
    Terminal
};
use std::io::Result;

enum Focus {
    Client,
    Log
}

pub struct App<'a> {
    pub focus: Focus,
    pub modal: Option<Modal<'a>>,
    pub clients: Clients,
    pub logs: Logs
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self {
            focus: Focus::Client,
            modal: None,
            clients: Clients::dummy(),
            logs: Logs::dummy()
        }
    }

    fn create_header() -> Paragraph<'a> {
        let block = Block::bordered().border_type(BorderType::Rounded);

        Paragraph::new(Text::styled(
            "Wireguard TUI",
            Style::default()
        ))
            .alignment(Alignment::Center)
            .block(block)
    }

    fn create_footer() -> Paragraph<'a> {
        let block = Block::bordered().border_type(BorderType::Rounded);

        Paragraph::new(Text::styled(
            "(ESC) quit | (TAB) switch focus | (g) top | (G) bottom | (j) down | (k) up",
            Style::default()
        ))
            .alignment(Alignment::Center)
            .block(block)
    }

    fn create_clients(&self) -> Table<'a> {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Clients");

        let header = ["Name", "Address", "Public Key"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(Style::default())
            .height(1);

        let rows = self.clients.data.iter().map(|client| {
            client.ref_array()
                .into_iter()
                .map(|content| Cell::from(Text::from(content.clone())))
                .collect::<Row>()
                .style(Style::default())
                .height(1)
        });

        Table::new(
            rows,
            [
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(40)
            ]
        )
            .block(block)
            .header(header)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always)
    }

    fn create_logs(&self) -> List<'a> {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Logs");

        let rows = self.logs.data.iter()
            .map(|log| ListItem::from(log.to_string()))
            .collect::<Vec<ListItem>>();

        List::new(rows)
            .block(block)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always)
    }

    fn draw(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3)
            ])
            .split(frame.area());

        let main_sections = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(60),
                Constraint::Percentage(40)
            ])
            .split(chunks[1]);

        let header = App::create_header();
        let footer = App::create_footer();
        let clients = self.create_clients();
        let logs = self.create_logs();

        frame.render_widget(header, chunks[0]);
        frame.render_widget(footer, chunks[2]);
        frame.render_stateful_widget(clients, main_sections[0], &mut self.clients.state);
        frame.render_stateful_widget(logs, main_sections[1], &mut self.logs.state);

        if let Some(modal) = &self.modal {
            modal.draw(frame);
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Option<Result<()>> {
        if self.modal.is_some() {
            return Modal::handle_key(self, key);
        }

        match key.code {
            KeyCode::Esc => return Some(Ok(())),
            KeyCode::Tab => {
                match &self.focus {
                    Focus::Client => {
                        self.clients.state.select(None);
                        self.logs.state.select_first();
                        self.focus = Focus::Log;
                    },
                    Focus::Log => {
                        self.clients.state.select_first();
                        self.logs.state.select(None);
                        self.focus = Focus::Client;
                    }
                }
            },
            KeyCode::Char('g') => {
                match &self.focus {
                    Focus::Client => self.clients.state.select_first(),
                    Focus::Log => self.logs.state.select_first()
                }
            },
            KeyCode::Char('G') => {
                match &self.focus {
                    Focus::Client => self.clients.state.select_last(),
                    Focus::Log => self.logs.state.select_last()
                }
            },
            KeyCode::Char('j') | KeyCode::Down => {
                match &self.focus {
                    Focus::Client => self.clients.state.select_next(),
                    Focus::Log => self.logs.state.select_next()
                }
            },
            KeyCode::Char('k') | KeyCode::Up => {
                match &self.focus {
                    Focus::Client => self.clients.state.select_previous(),
                    Focus::Log => self.logs.state.select_previous()
                }
            },
            KeyCode::Char('a') => {
                match &self.focus {
                    Focus::Client => self.modal = Some(Modal::CreateClient(ClientForm::new("Create"))),
                    _ => ()
                }
            }
            _ => ()
        };

        None
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('c') &&
                       key.modifiers.contains(KeyModifiers::CONTROL) {
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

// impl<'a> Widget for App<'a> {
//     fn render(self, area: Rect) {
//         let chunks = Layout::default()
//             .direction(Direction::Vertical)
//             .constraints([
//                 Constraint::Length(3),
//                 Constraint::Min(1),
//                 Constraint::Length(3)
//             ])
//             .split(area);

//         let main_sections = Layout::default()
//             .direction(Direction::Horizontal)
//             .constraints([
//                 Constraint::Percentage(60),
//                 Constraint::Percentage(40)
//             ])
//             .split(chunks[1]);

//         App::render_header(chunks[0], buffer);
//         App::render_footer(chunks[2], buffer);
//         App::render_clients(&mut self.clients, main_sections[0], buffer);
//         App::render_logs(&mut self.logs, main_sections[1], buffer);
//     }
// }