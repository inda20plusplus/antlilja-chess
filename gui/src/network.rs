

pub struct ConnectionHandler {
    is_host: bool,
    stream: Arc<Mutex<TcpStream>>,
}
