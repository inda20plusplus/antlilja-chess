use chess::{Move, PieceType, Pos};
use std::collections::VecDeque;
use std::io::prelude::*;
use std::net::{IpAddr, TcpListener, TcpStream};
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

    pub fn from_bytes(bytes: [u8; 4]) -> Result<MoveType, &'static str> {
        match bytes[0] {
            0x0 => Ok(MoveType::Standard(bytes[1], bytes[2])),
            0x1 => Ok(MoveType::EnPassant(bytes[1], bytes[2])),
            0x2 => Ok(MoveType::Promotion(bytes[1], bytes[2], bytes[3])),
            0x3 => Ok(MoveType::KingsideCastle),
            0x4 => Ok(MoveType::QueensideCastle),
            _ => Err("Byte is not valid move type"),
        }
    }

    pub fn to_chess_move(&self) -> (Option<Pos>, Move) {
        match self {
            MoveType::Standard(origin, target) => {
                let pos = Pos::new_index(*origin);
                let r#move = Move::Move(Pos::new_index(*target));

                (Some(pos), r#move)
            }

            MoveType::EnPassant(origin, target) => {
                let pos = Pos::new_index(*origin);
                let r#move = Move::EnPassant(Pos::new_index(*target));

                (Some(pos), r#move)
            }

            MoveType::Promotion(origin, target, piece_type) => {
                let pos = Pos::new_index(*origin);
                let r#type = match piece_type {
                    0x0 => PieceType::Knight,
                    0x1 => PieceType::Bishop,
                    0x2 => PieceType::Rook,
                    0x3 => PieceType::Queen,
                    _ => panic!("Invalid PieceType when converting to chess::Move"),
                };
                let r#move = Move::PawnPromotion(r#type, Pos::new_index(*target));

                (Some(pos), r#move)
            }

            MoveType::KingsideCastle => (None, Move::KingSideCastling),
            MoveType::QueensideCastle => (None, Move::QueenSideCastling),
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

    pub fn from_bytes(bytes: [u8; 5]) -> Result<Self, &'static str> {
        match bytes[0] {
            0x0 => Ok(Message::Decline),
            0x1 => {
                let move_bytes: [u8; 4] = [bytes[1], bytes[2], bytes[3], bytes[4]];
                Ok(Message::Move(MoveType::from_bytes(move_bytes).unwrap()))
            }
            0x2 => Ok(Message::Undo),
            0x3 => Ok(Message::Accept),
            0x4 => Ok(Message::Checkmate),
            0x5 => Ok(Message::Draw),
            0x6 => Ok(Message::Resign),
            _ => Err("Byte is not valid message type"),
        }
    }
}

pub struct ConnectionHandler {
    pub is_host: bool,
    stream: Arc<Mutex<TcpStream>>,
    recieved_messages: Arc<Mutex<VecDeque<[u8; 5]>>>,
    read_handle: Option<thread::JoinHandle<()>>,
    pub last_sent: Option<Message>,
}

impl ConnectionHandler {
    pub fn new(stream: TcpStream, is_host: bool) -> Self {
        let mut handler = Self {
            is_host,
            stream: Arc::new(Mutex::new(stream)),
            recieved_messages: Arc::new(Mutex::new(VecDeque::with_capacity(10))),
            last_sent: None,
            read_handle: None,
        };

        handler.read_handle = Some(handler.spawn_read_thread());

        handler
    }

    pub fn connect(ip: IpAddr, port: u16) -> Self {
        let stream = TcpStream::connect(format!("{}:{}", ip, port)).unwrap();

        Self::new(stream, false)
    }

    pub fn host(port: u16) -> Self {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
        let (stream, _addr) = listener.accept().unwrap();

        Self::new(stream, true)
    }

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

    fn write_message(&mut self, message: Message) -> std::io::Result<()> {
        let stream_mutex = Arc::clone(&self.stream);
        let mut stream = stream_mutex.lock().unwrap();

        stream.write_all(&message.to_bytes())
    }
}
