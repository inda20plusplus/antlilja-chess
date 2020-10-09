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

impl MoveType {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            MoveType::Standard(origin, target) => vec![0x0, *origin, *target],
            MoveType::EnPassant(origin, target) => vec![0x1, *origin, *target],
            MoveType::Promotion(origin, target, piece_type) => {
                vec![0x2, *origin, *target, *piece_type]
            }
            MoveType::KingsideCastle => vec![0x3],
            MoveType::QueensideCastle => vec![0x4],
        }
    }
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

impl Message {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0x0];

        match self {
            Message::Decline => bytes[0] = 0x0,
            Message::Move(move_type) => {
                bytes[0] = 0x1;
                bytes.append(&mut move_type.to_bytes());
            }
            Message::Undo => bytes[0] = 0x2,
            Message::Accept => bytes[0] = 0x3,
            Message::Checkmate => bytes[0] = 0x4,
            Message::Draw => bytes[0] = 0x5,
            Message::Resign => bytes[0] = 0x6,
        };

        bytes
    }
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
