pub enum Focus {
    Home(HomeSection),
    CreateForm(ClientFormField),
    EditForm(ClientFormField)
}

impl Focus {
    pub fn new() -> Self {
        Self::Home(HomeSection::Clients)
    }
}

pub enum HomeSection {
    Clients,
    Logs
}

pub enum ClientFormField {
    Name,
    Address,
    PublicKey
}
