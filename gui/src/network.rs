
pub enum Hosting {
    Remote { is_host: bool },
    Local,
}

pub struct ConnectionHandler {
    is_host: bool,
}
