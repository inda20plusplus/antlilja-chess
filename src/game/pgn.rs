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
    pub fn parse_pgn_move(&self, pmove: &str) -> ((u8, u8), Move) {
        if pmove.is_empty() {
            return ((0, 0), Move::None);
        }

        if pmove == "O-O" {
            return (
                (4, if self.color == Color::White { 0 } else { 7 }),
                Move::KingSideCastling,
            );
        }

        if pmove == "O-O-O" {
            return (
                (4, if self.color == Color::White { 0 } else { 7 }),
                Move::QueenSideCastling,
            );
        }

        let (piece, tokens) = lex_pgn(pmove);

        match tokens[..] {
            [Token::File(x), Token::Rank(y)] => {
                let r#move = Move::move_xy(x, y);
                for i in 0..64 {
                    let space = self.at_index(i);
                    if !space.is_empty()
                        && space.get_color() == self.color
                        && space.get_type() == piece
                    {
                        let (from_x, from_y) = Pos::from_index(i as u8).to_xy();
                        let moves = self.get_moves_for(from_x, from_y);
                        if moves.is_some() && moves.unwrap().exists(r#move) {
                            return ((from_x, from_y), r#move);
                        }
                    }
                }
                return ((0, 0), Move::None);
            }
            [Token::File(from_x), Token::File(x), Token::Rank(y)] => {
                let r#move = Move::move_xy(x, y);
                for from_y in 0..8 {
                    let space = self.at_xy(from_x, from_y);
                    if !space.is_empty()
                        && space.get_color() == self.color
                        && space.get_type() == piece
                    {
                        let moves = self.get_moves_for(from_x, from_y);
                        if moves.is_some() && moves.unwrap().exists(r#move) {
                            return ((from_x, from_y), r#move);
                        }
                    }
                }
                return ((0, 0), Move::None);
            }
            [Token::Rank(from_y), Token::File(x), Token::Rank(y)] => {
                let r#move = Move::move_xy(x, y);
                for from_x in 0..8 {
                    let space = self.at_xy(from_x, from_y);
                    if !space.is_empty()
                        && space.get_color() == self.color
                        && space.get_type() == piece
                    {
                        let moves = self.get_moves_for(from_x, from_y);
                        if moves.is_some() && moves.unwrap().exists(r#move) {
                            return ((from_x, from_y), r#move);
                        }
                    }
                }
                return ((0, 0), Move::None);
            }
            [Token::File(x), Token::Rank(y), Token::Promotion, Token::Piece(piece)] => {
                let dir: i8 = if self.color == Color::White { -1 } else { 1 };
                return (
                    (x, (y as i8 + dir) as u8),
                    Move::PawnPromotion(piece, Pos::from_xy(x, y)),
                );
            }
            [Token::File(from_x), Token::Rank(from_y), Token::File(x), Token::Rank(y)] => {
                return ((from_x, from_y), Move::move_xy(x, y));
            }
            _ => {}
        }

        return ((0, 0), Move::None);
    }
}
