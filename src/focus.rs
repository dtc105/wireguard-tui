pub enum Focus {
    Main(MainFocus),
    Add(AddFocus),
}

impl Default for Focus {
    fn default() -> Self {
        Self::Main(MainFocus::default())
    }
}

#[derive(Default)]
pub enum MainFocus {
    #[default]
    Clients,
    Logs,
}

#[derive(Default)]
pub enum AddFocus {
    #[default]
    Name,
    Address,
    PublicKey,
}
