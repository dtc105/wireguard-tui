use chrono::{DateTime, Utc};
use ratatui::widgets::{ListState, TableState};

pub struct Data {
    pub create_form: ClientForm,
    pub edit_form: ClientForm,
    pub clients: Clients,
    pub logs: Logs
}

impl Data {
    pub fn new() -> Self {
        Self {
            create_form: ClientForm::default(),
            edit_form: ClientForm::default(),
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
}

#[derive(Default)]
pub struct ClientForm {
    pub name: String,
    pub address: String,
    pub public_key: String
}

pub struct Clients {
    pub data: Vec<Client>,
    pub state: TableState
}

pub struct Logs {
    pub data: Vec<Log>,
    pub state: ListState
}

pub struct Client {
    pub name: String,
    pub address: String,
    pub public_key: String,
}

impl Client {
    pub fn ref_array(&self) -> [&String; 3] {
        [&self.name, &self.address, &self.public_key]
    }

    pub fn dummy() -> Vec<Client> {
        let mut dummy_clients = Vec::new();

        dummy_clients.push(Client {
            name: "Albert".to_string(),
            address: "10.0.0.1".to_string(),
            public_key: "abc123".to_string()
        });

        dummy_clients.push(Client {
            name: "Bobby".to_string(),
            address: "10.0.0.2".to_string(),
            public_key: "def456".to_string()
        });

        dummy_clients.push(Client {
            name: "Charles".to_string(),
            address: "10.0.0.3".to_string(),
            public_key: "ghi789".to_string()
        });

        dummy_clients
    }
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

    pub fn dummy() -> Vec<Log> {
        let mut dummy_logs = Vec::new();

        dummy_logs.push(Log {
            timestamp: Utc::now(),
            client: Client {
                name: "Albert".to_string(),
                address: "10.0.0.1".to_string(),
                public_key: "abc123".to_string()
            },
            status: LogStatus::Connected
        });

        dummy_logs.push(Log {
            timestamp: Utc::now(),
            client: Client {
                name: "Bobby".to_string(),
                address: "10.0.0.2".to_string(),
                public_key: "def456".to_string()
            },
            status: LogStatus::Disconnected
        });

        dummy_logs.push(Log {
            timestamp: Utc::now(),
            client: Client {
                name: "Charles".to_string(),
                address: "10.0.0.3".to_string(),
                public_key: "ghi789".to_string()
            },
            status: LogStatus::Connected
        });

        dummy_logs
    }
}

pub enum LogStatus {
    Connected,
    Disconnected,
}
