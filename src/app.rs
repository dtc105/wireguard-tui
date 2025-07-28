use crate::{
    dto::{Client, Log},
    focus::Focus,
};

pub struct AppState {
    pub focus: Focus,
}

pub struct Components {
    pub clients: Vec<Client>,
    pub logs: Vec<Log>,
}

pub struct App {
    pub state: AppState,
    pub components: Components,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState {
                focus: Focus::default(),
            },
            components: Components {
                clients: Vec::new(),
                logs: Vec::new(),
            },
        }
    }
}
