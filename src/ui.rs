use crate::{
    app::App,
    dto::{Clients, Logs},
    modal::Modal,
};

use ratatui::{
    Frame, Terminal,
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::Text,
    widgets::{Block, BorderType, Cell, HighlightSpacing, List, ListItem, Paragraph, Row, Table},
};

fn create_header<'a>() -> Paragraph<'a> {
    let block = Block::bordered().border_type(BorderType::Rounded);

    Paragraph::new(Text::styled("Wireguard TUI", Style::default()))
        .alignment(Alignment::Center)
        .block(block)
}

fn create_footer<'a>() -> Paragraph<'a> {
    let block = Block::bordered().border_type(BorderType::Rounded);

    Paragraph::new(Text::styled(
        "(Esc) quit | (Tab) switch focus | (g) top | (G) bottom | (j) down | (k) up",
        Style::default(),
    ))
    .alignment(Alignment::Center)
    .block(block)
}

fn create_clients<'a>(clients: &Clients) -> Table<'a> {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title("Clients");

    let header = ["Name", "Address", "Public Key"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(Style::default())
        .height(1);

    let rows = clients.data.iter().map(|client| {
        client
            .ref_array()
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
            Constraint::Percentage(40),
        ],
    )
    .block(block)
    .header(header)
    .highlight_symbol(">")
    .highlight_spacing(HighlightSpacing::Always)
}

fn create_logs<'a>(logs: &Logs) -> List<'a> {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title("Logs");

    let rows = logs
        .data
        .iter()
        .map(|log| ListItem::from(log.to_string()))
        .collect::<Vec<ListItem>>();

    List::new(rows)
        .block(block)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always)
}

pub fn draw(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let main_sections = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[1]);

    let header = create_header();
    let footer = create_footer();
    let clients = create_clients(&app.clients);
    let logs = create_logs(&app.logs);

    frame.render_widget(header, chunks[0]);
    frame.render_widget(footer, chunks[2]);
    frame.render_stateful_widget(clients, main_sections[0], &mut app.clients.state);
    frame.render_stateful_widget(logs, main_sections[1], &mut app.logs.state);

    if let Some(modal) = &app.modal {
        modal.draw(frame);
    }
}
