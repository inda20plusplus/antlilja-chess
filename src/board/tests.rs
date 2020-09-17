use super::*;

#[test]
fn new_board() {
    let board = Board::new();

    let exists_at_both_sides = |offset, r#type| {
        assert_eq!(
            board.at_index(offset),
            TaggedPiece::original(r#type, Color::White)
        );
        assert_eq!(
            board.at_index(8 * 7 + offset),
            TaggedPiece::original(r#type, Color::Black)
        );
    };

    let exists_matching_at_both_sides = |offset, r#type| {
        exists_at_both_sides(offset, r#type);
        exists_at_both_sides(7 - offset, r#type);
    };

    for i in 0..8 {
        assert_eq!(
            board.at_index(8 + i),
            TaggedPiece::original(PieceType::Pawn, Color::White)
        );
        assert_eq!(
            board.at_index(8 * 6 + i),
            TaggedPiece::original(PieceType::Pawn, Color::Black)
        );
    }

    for i in 8 * 2..8 * 6 {
        assert_eq!(board.at_index(i), TaggedPiece::empty());
    }

    exists_matching_at_both_sides(0, PieceType::Rook);
    exists_matching_at_both_sides(1, PieceType::Knight);
    exists_matching_at_both_sides(2, PieceType::Bishop);
    exists_at_both_sides(3, PieceType::Queen);
    exists_at_both_sides(4, PieceType::King);
}
