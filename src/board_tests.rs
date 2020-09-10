use super::*;

#[test]
fn new_board() {
    let board = Board::new();

    let exists_at_both_sides = |offset, r#type| {
        assert_eq!(
            board.at_index(offset),
            TaggedPiece::new(r#type, Color::White)
        );
        assert_eq!(
            board.at_index(8 * 7 + offset),
            TaggedPiece::new(r#type, Color::Black)
        );
    };

    let exists_matching_at_both_sides = |offset, r#type| {
        exists_at_both_sides(offset, r#type);
        exists_at_both_sides(7 - offset, r#type);
    };

    for i in 0..8 {
        assert_eq!(
            board.at_index(8 + i),
            TaggedPiece::new(PieceType::Pawn, Color::White)
        );
        assert_eq!(
            board.at_index(8 * 6 + i),
            TaggedPiece::new(PieceType::Pawn, Color::Black)
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

#[test]
fn knight_moves_new_board() {
    let board = Board::new();
    let mut moves = Vec::<Move>::with_capacity(2);

    let mut check = |x, y, end_y| {
        let from = Pos::from_xy(x, y);
        let correct_moves = [
            Move::Move(from, Pos::from_xy(x + 1, end_y)),
            Move::Move(from, Pos::from_xy(x - 1, end_y)),
        ];

        assert_eq!(board.get_moves_for(&mut moves, x, y), 2);
        for (i, m) in moves.iter().enumerate() {
            assert_eq!(m, &correct_moves[i]);
        }
        moves.clear();
    };

    check(1, 0, 2);
    check(6, 0, 2);
    check(1, 7, 5);
    check(6, 7, 5);
}

#[test]
fn bishop_moves_new_board() {
    let board = Board::new();

    let mut moves = Vec::<Move>::new();
    assert_eq!(board.get_moves_for(&mut moves, 2, 0), 0);
    assert_eq!(board.get_moves_for(&mut moves, 5, 0), 0);
    assert_eq!(board.get_moves_for(&mut moves, 2, 7), 0);
    assert_eq!(board.get_moves_for(&mut moves, 5, 7), 0);
}

#[test]
fn queen_moves_new_board() {
    let board = Board::new();

    let mut moves = Vec::<Move>::new();
    assert_eq!(board.get_moves_for(&mut moves, 3, 0), 0);
    assert_eq!(board.get_moves_for(&mut moves, 3, 7), 0);
}

#[test]
fn king_moves_new_board() {
    let board = Board::new();

    let mut moves = Vec::<Move>::new();
    assert_eq!(board.get_moves_for(&mut moves, 4, 0), 0);
    assert_eq!(board.get_moves_for(&mut moves, 4, 7), 0);
}
