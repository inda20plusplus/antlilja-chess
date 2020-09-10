use super::*;

#[test]
fn new_board() {
    let board = Board::new();

    let exists_at_both_sides = |offset, r#type| {
        assert_eq!(
            board.at_index(offset),
            ColoredPiece::new(r#type, Color::White)
        );
        assert_eq!(
            board.at_index(8 * 7 + offset),
            ColoredPiece::new(r#type, Color::Black)
        );
    };

    let exists_matching_at_both_sides = |offset, r#type| {
        exists_at_both_sides(offset, r#type);
        exists_at_both_sides(7 - offset, r#type);
    };

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

    exists_matching_at_both_sides(0, PieceType::Rook);
    exists_matching_at_both_sides(1, PieceType::Knight);
    exists_matching_at_both_sides(2, PieceType::Bishop);
    exists_at_both_sides(3, PieceType::Queen);
    exists_at_both_sides(4, PieceType::King);
}

#[test]
fn pawn_moves_new_board() {
    let board = Board::new();

    let mut moves = Vec::<Move>::with_capacity(10);
    for x in 0..8 {
        // Top of board
        let mut check_side = |y_start, y_dir: i8| {
            let move_to = |to_x, to_y| {
                let from = Pos::from_xy(x, y_start);
                let to = Pos::from_xy(to_x, to_y);
                Move::Move(from, to)
            };

            let correct_moves = [
                move_to(x, (y_start as i8 + y_dir) as u8),
                move_to(x, (y_start as i8 + y_dir * 2) as u8),
            ];

            let count = board.get_moves_for(&mut moves, x, y_start);
            assert_eq!(count, correct_moves.len());

            for (i, m) in moves.iter().enumerate() {
                assert_eq!(m, &correct_moves[i]);
            }

            moves.clear();
        };

        check_side(1, 1);
        check_side(6, -1);
    }
}

#[test]
fn rook_moves_new_board() {
    let board = Board::new();

    let mut moves = Vec::<Move>::new();
    assert_eq!(board.get_moves_for(&mut moves, 0, 0), 0);
    assert_eq!(board.get_moves_for(&mut moves, 7, 0), 0);
    assert_eq!(board.get_moves_for(&mut moves, 0, 7), 0);
    assert_eq!(board.get_moves_for(&mut moves, 7, 7), 0);
}
