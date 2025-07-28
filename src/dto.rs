use chrono::{DateTime, Utc};

pub struct Client {
    pub name: String,
    pub address: String,
    pub public_key: String,
}

pub struct Log {
    pub timestamp: DateTime<Utc>,
    pub client: Client,
    pub status: LogStatus,
}

pub enum LogStatus {
    Connected,
    Disconnected,
}
