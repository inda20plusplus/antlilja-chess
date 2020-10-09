use std::collections::VecDeque;
use std::io::prelude::*;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoveType {
    Standard(u8, u8),
    EnPassant(u8, u8),
    Promotion(u8, u8, u8),
    KingsideCastle,
    QueensideCastle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Message {
    Decline,
    Move(MoveType),
    Undo,
    Accept,
    Checkmate,
    Draw,
    Resign,
}

pub struct ConnectionHandler {
    is_host: bool,
    stream: Arc<Mutex<TcpStream>>,
    recieved_messages: Arc<Mutex<VecDeque<[u8; 5]>>>,
    read_handle: Option<thread::JoinHandle<()>>,
}

impl ConnectionHandler {
    fn spawn_read_thread(&mut self) -> thread::JoinHandle<()> {
        let stream = Arc::clone(&self.stream);
        let queue = Arc::clone(&self.recieved_messages);

        thread::spawn(move || loop {
            let mut buf = [0; 5];

            let mut stream = stream.lock().unwrap();
            let mut queue = queue.lock().unwrap();

            let result = stream.read(&mut buf).unwrap();

            if result == 0 {
                break;
            };

            queue.push_front(buf);

            drop(stream);
            drop(queue);
        })
    }
}
