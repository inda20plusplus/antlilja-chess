use crate::{Color, Game, Move, PieceType, Pos};

#[derive(Debug)]
enum Token {
    Piece(PieceType),
    File(u8),
    Rank(u8),
    Promotion,
}

fn lex_pgn(pmove: &str) -> (PieceType, Vec<Token>) {
    let mut chars = pmove.chars();
    let piece = {
        let mut peekable = pmove.chars().peekable();
        let peek = peekable.peek().unwrap();
        if peek.is_uppercase() {
            chars.next();
            match peek {
                'P' => PieceType::Pawn,
                'R' => PieceType::Rook,
                'N' => PieceType::Knight,
                'B' => PieceType::Bishop,
                'Q' => PieceType::Queen,
                'K' => PieceType::King,
                _ => PieceType::None,
            }
        } else {
            PieceType::Pawn
        }
    };

    let mut buffer = Vec::<Token>::with_capacity(pmove.len());
    for c in chars {
        if c.is_digit(10) {
            buffer.push(Token::Rank(c as u8 - 49));
            continue;
        }

        if c.is_uppercase() {
            buffer.push(Token::Piece(match c {
                'P' => PieceType::Pawn,
                'R' => PieceType::Rook,
                'N' => PieceType::Knight,
                'B' => PieceType::Bishop,
                'Q' => PieceType::Queen,
                'K' => PieceType::King,
                _ => PieceType::None,
            }));
            continue;
        }

        if c == '=' {
            buffer.push(Token::Promotion);
            continue;
        }

        if c == 'x' {
            continue;
        }

        if c.is_lowercase() && c.is_alphabetic() {
            buffer.push(Token::File(c as u8 - 97));
            continue;
        }
    }

    return (piece, buffer);
}

impl Game {
    pub fn parse_pgn_move(&self, pmove: &str) -> (Pos, Move) {
        const INVALID: (Pos, Move) = (Pos::invalid(), Move::None);

        if pmove.is_empty() {
            return INVALID;
        }

        if pmove == "O-O" {
            return (
                Pos::new_index(if self.color == Color::White { 4 } else { 60 }),
                Move::KingSideCastling,
            );
        }

        if pmove == "O-O-O" {
            return (
                Pos::new_index(if self.color == Color::White { 4 } else { 60 }),
                Move::QueenSideCastling,
            );
        }

        let (piece, tokens) = lex_pgn(pmove);

        match tokens[..] {
            [Token::File(x), Token::Rank(y)] => {
                let pos = Pos::new_xy(x, y);
                let move_move = Move::Move(pos);
                let en_passant_move = Move::EnPassant(pos);
                for i in 0..64 {
                    let space = self.at_index(i);
                    if !space.is_empty() && space.color() == self.color && space.get_type() == piece
                    {
                        let from = Pos::new_index(i as u8);
                        if let Some(moves) = self.get_moves_for(from) {
                            if moves.contains(&move_move) {
                                return (from, move_move);
                            }

                            if piece == PieceType::Pawn && moves.contains(&en_passant_move) {
                                return (from, en_passant_move);
                            }
                        }
                    }
                }
                return INVALID;
            }
            [Token::File(from_x), Token::File(x), Token::Rank(y)] => {
                let pos = Pos::new_xy(x, y);
                let move_move = Move::Move(pos);
                let en_passant_move = Move::EnPassant(pos);
                for from_y in 0..8 {
                    let from = Pos::new_xy(from_x, from_y);
                    let space = self.at_pos(from);
                    if !space.is_empty() && space.color() == self.color && space.get_type() == piece
                    {
                        if let Some(moves) = self.get_moves_for(from) {
                            if moves.contains(&move_move) {
                                return (from, move_move);
                            }

                            if piece == PieceType::Pawn && moves.contains(&en_passant_move) {
                                return (from, en_passant_move);
                            }
                        }
                    }
                }
                return INVALID;
            }
            [Token::Rank(from_y), Token::File(x), Token::Rank(y)] => {
                let pos = Pos::new_xy(x, y);
                let move_move = Move::Move(pos);
                let en_passant_move = Move::EnPassant(pos);
                for from_x in 0..8 {
                    let from = Pos::new_xy(from_x, from_y);
                    let space = self.at_pos(from);
                    if !space.is_empty() && space.color() == self.color && space.get_type() == piece
                    {
                        if let Some(moves) = self.get_moves_for(from) {
                            if moves.contains(&move_move) {
                                return (from, move_move);
                            }

                            if piece == PieceType::Pawn && moves.contains(&en_passant_move) {
                                return (from, en_passant_move);
                            }
                        }
                    }
                }
                return INVALID;
            }
            [Token::File(x), Token::Rank(y), Token::Promotion, Token::Piece(piece)] => {
                let dir: i8 = if self.color == Color::White { -1 } else { 1 };
                let from = Pos::new_xy(x, y).move_y_non_fail(dir);
                return (from, Move::PawnPromotion(piece, Pos::new_xy(x, y)));
            }
            [Token::File(from_x), Token::Rank(from_y), Token::File(x), Token::Rank(y)] => {
                let pos = Pos::new_xy(x, y);
                let from = Pos::new_xy(from_x, from_y);
                if piece == PieceType::Pawn && from_x != x && self.at_pos(pos).is_empty() {
                    return (from, Move::EnPassant(pos));
                }
                return (from, Move::Move(pos));
            }
            _ => {}
        }

        return INVALID;
    }
}
