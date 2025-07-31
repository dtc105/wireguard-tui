use crate::{
    app::App,
    dto::{
        Client,
        Clients,
        Log,
        Logs
    }
};

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

pub struct Home {
    clients: Clients,
    logs: Logs
}

impl Home {
    pub fn new() -> Self {
        Self {
            clients: Clients {
                data: Client::dummy(),
                state: TableState::default().with_selected(0)
            },
            logs: Logs {
                data: Log::dummy(),
                state: ListState::default().with_selected(None)
            }
        }
    }

    fn render_header(area: Rect, buffer: &mut Buffer) {
        let block = Block::bordered();

        Paragraph::new(Text::styled(
            "Wireguard TUI",
            Style::default()
        ))
            .alignment(Alignment::Center)
            .block(block)
            .render(area, buffer);
    }

    fn render_footer(area: Rect, buffer: &mut Buffer) {
        let block = Block::bordered();

        Paragraph::new(Text::styled(
            "(ESC) quit | (TAB) switch focus | (g) top | (G) bottom | (k) up | (j) down", 
            Style::default()
        ))
            .alignment(Alignment::Center)
            .block(block)
            .render(area, buffer);
    }

    fn render_clients(clients: &mut Clients, area: Rect, buffer: &mut Buffer) {
        let header = ["Name", "Address", "Public Key"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(Style::default())
            .height(1);

        let rows = clients.data.iter().map(|client| {
            client.ref_array()
                .into_iter()
                .map(|content| Cell::from(Text::from(content.clone())))
                .collect::<Row>()
                .style(Style::default())
                .height(1)
        });

        let table = Table::new(
            rows,
            [
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(40)
            ]
        )
            .header(header)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(table, area, buffer, &mut clients.state);
    }

    fn render_logs(logs: &mut Logs, area: Rect, buffer: &mut Buffer) {
        let block = Block::bordered()
            .title("Logs");

        let rows = logs.data.iter()
            .map(|log| ListItem::from(log.to_string()))
            .collect::<Vec<ListItem>>();

        let list = List::new(rows)
            .block(block)
            .highlight_symbol(">");

        StatefulWidget::render(list, area, buffer, &mut logs.state);
    }
}

impl Widget for &mut Home {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3)
            ])
            .split(area);

        let main_sections = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(60),
                Constraint::Percentage(40)
            ])
            .split(chunks[1]);

        Home::render_header(chunks[0], buffer);
        Home::render_footer(chunks[2], buffer);
        Home::render_clients(&mut self.clients, main_sections[0], buffer);
        Home::render_logs(&mut self.logs, main_sections[1], buffer);
    }
}