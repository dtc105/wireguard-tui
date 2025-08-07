use crate::{modal::Modal, wireguard::Wireguard};
use chrono::{DateTime, Utc};
use ratatui::widgets::{ListState, TableState};

pub struct Client {
    pub name: String,
    pub address: String,
    pub public_key: String,
}

impl Client {
    pub fn ref_array(&self) -> [&String; 3] {
        [&self.name, &self.address, &self.public_key]
    }
}

pub struct Clients {
    pub data: Vec<Client>,
    pub state: TableState,
}

impl Clients {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            state: TableState::default().with_selected(0),
        }
    }

    pub fn init(wireguard_connection: Wireguard) -> Self {
        let clients = wireguard_connection.get_clients();

        Self {
            data: clients,
            state: TableState::default().with_selected(0),
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.data.shrink_to(5);
        self.state.select_first();
    }

    pub fn add_client(&mut self, client: Client) {
        self.data.push(client);
    }

    pub fn set_clients(&mut self, clients: Vec<Client>) {
        self.data = clients;
    }
}

pub enum LogStatus {
    Connected,
    Disconnected,
}

pub struct Log {
    pub timestamp: DateTime<Utc>,
    pub client: Client,
    pub status: LogStatus,
}

impl Log {
    pub fn to_string(&self) -> String {
        format!("TIME CLIENT TYPE")
    }
}

pub struct Logs {
    pub data: Vec<Log>,
    pub state: ListState,
}

impl Logs {
    pub fn dummy() -> Self {
        let mut dummy_logs = Vec::new();

        dummy_logs.push(Log {
            timestamp: Utc::now(),
            client: Client {
                name: "Albert".to_string(),
                address: "10.0.0.1".to_string(),
                public_key: "abc123".to_string(),
            },
            status: LogStatus::Connected,
        });

        dummy_logs.push(Log {
            timestamp: Utc::now(),
            client: Client {
                name: "Bobby".to_string(),
                address: "10.0.0.2".to_string(),
                public_key: "def456".to_string(),
            },
            status: LogStatus::Disconnected,
        });

        dummy_logs.push(Log {
            timestamp: Utc::now(),
            client: Client {
                name: "Charles".to_string(),
                address: "10.0.0.3".to_string(),
                public_key: "ghi789".to_string(),
            },
            status: LogStatus::Connected,
        });

        Self {
            data: dummy_logs,
            state: ListState::default().with_selected(None),
        }
    }
}

pub enum ClientFocus {
    Name,
    Address,
    PublicKey,
}

pub struct ClientForm<'a> {
    pub title: &'a str,
    pub values: Client,
    pub callback: Option<&'a Client>,
    pub focus: ClientFocus,
}

impl<'a> ClientForm<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            values: Client {
                name: String::new(),
                address: String::new(),
                public_key: String::new(),
            },
            callback: None,
            focus: ClientFocus::Name,
        }
    }

    pub fn from_client(title: &'a str, client: &'a Client) -> Self {
        Self {
            title,
            values: Client {
                name: String::new(),
                address: String::new(),
                public_key: String::new(),
            },
            callback: Some(client),
            focus: ClientFocus::Name,
        }
    }

    pub fn next_field(&mut self) {
        match self.focus {
            ClientFocus::Name => self.focus = ClientFocus::Address,
            ClientFocus::Address => self.focus = ClientFocus::PublicKey,
            ClientFocus::PublicKey => self.focus = ClientFocus::Name,
        }
    }

    pub fn prev_field(&mut self) {
        match self.focus {
            ClientFocus::Name => self.focus = ClientFocus::PublicKey,
            ClientFocus::Address => self.focus = ClientFocus::Name,
            ClientFocus::PublicKey => self.focus = ClientFocus::Address,
        }
    }

    pub fn backspace(&mut self) {
        match &self.focus {
            ClientFocus::Name => self.values.name.pop(),
            ClientFocus::Address => self.values.address.pop(),
            ClientFocus::PublicKey => self.values.public_key.pop(),
        };
    }

    pub fn add_char(&mut self, chr: char) {
        match &self.focus {
            ClientFocus::Name => self.values.name.push(chr),
            ClientFocus::Address => self.values.address.push(chr),
            ClientFocus::PublicKey => self.values.public_key.push(chr),
        };
    }
}

