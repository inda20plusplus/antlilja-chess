use super::*;

fn exists_at(board: &Board, offset: u8, r#type: PieceType) {
    assert_eq!(
        board.at_index(offset),
        ColoredPiece::new(r#type, Color::White)
    );
    assert_eq!(
        board.at_index(8 * 7 + offset),
        ColoredPiece::new(r#type, Color::Black)
    );
}

fn exists_matching_at(board: &Board, offset: u8, piece: PieceType) {
    exists_at(board, offset, piece);
    exists_at(board, 7 - offset, piece);
}

#[test]
fn new_board() {
    let board = Board::new();

    for i in 0..8 {
        assert_eq!(
            board.at_index(8 + i),
            ColoredPiece::new(PieceType::Pawn, Color::White)
        );
        assert_eq!(
            board.at_index(8 * 6 + i),
            ColoredPiece::new(PieceType::Pawn, Color::Black)
        );
    }

    for i in 8 * 2..8 * 6 {
        assert_eq!(board.at_index(i), ColoredPiece::empty());
    }

    exists_matching_at(&board, 0, PieceType::Rook);
    exists_matching_at(&board, 1, PieceType::Knight);
    exists_matching_at(&board, 2, PieceType::Bishop);
    exists_at(&board, 3, PieceType::Queen);
    exists_at(&board, 4, PieceType::King);
}
