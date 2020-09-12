use super::*;

#[test]
fn new_board() {
    let game = Game::new();

    let exists_at_both_sides = |offset, r#type| {
        assert_eq!(
            game.at_index(offset),
            TaggedPiece::original(r#type, Color::White)
        );
        assert_eq!(
            game.at_index(8 * 7 + offset),
            TaggedPiece::original(r#type, Color::Black)
        );
    };

    let exists_matching_at_both_sides = |offset, r#type| {
        exists_at_both_sides(offset, r#type);
        exists_at_both_sides(7 - offset, r#type);
    };

    for i in 0..8 {
        assert_eq!(
            game.at_index(8 + i),
            TaggedPiece::original(PieceType::Pawn, Color::White)
        );
        assert_eq!(
            game.at_index(8 * 6 + i),
            TaggedPiece::original(PieceType::Pawn, Color::Black)
        );
    }

    for i in 8 * 2..8 * 6 {
        assert_eq!(game.at_index(i), TaggedPiece::empty());
    }

    exists_matching_at_both_sides(0, PieceType::Rook);
    exists_matching_at_both_sides(1, PieceType::Knight);
    exists_matching_at_both_sides(2, PieceType::Bishop);
    exists_at_both_sides(3, PieceType::Queen);
    exists_at_both_sides(4, PieceType::King);
}

#[test]
fn pawn_moves_new_board() {
    let mut game = Game::new();

    for x in 0..8 {
        let check_side = |board: &mut Game, y_start, y_dir: i8| {
            let move_to = |to_y| {
                let from = Pos::from_xy(x, y_start);
                let to = Pos::from_xy(x, to_y);
                Move::Move(from, to)
            };

            let correct_moves = MoveArray::from_slice(&[
                move_to((y_start as i8 + y_dir) as u8),
                move_to((y_start as i8 + y_dir * 2) as u8),
            ]);

            assert_eq!(board.get_moves_for(x, y_start).unwrap(), &correct_moves);
        };

        check_side(&mut game, 1, 1);
        game.flip_color();
        check_side(&mut game, 6, -1);
        game.flip_color();
    }
}

#[test]
fn rook_moves_new_board() {
    let mut game = Game::new();

    assert!(game.get_moves_for(0, 0).unwrap().is_empty());
    assert!(game.get_moves_for(7, 0).unwrap().is_empty());

    game.flip_color();
    assert!(game.get_moves_for(0, 7).unwrap().is_empty());
    assert!(game.get_moves_for(7, 7).unwrap().is_empty());
}

#[test]
fn knight_moves_new_board() {
    let mut game = Game::new();

    let check = |board: &mut Game, x, y, end_y| {
        let from = Pos::from_xy(x, y);
        let correct_moves = MoveArray::from_slice(&[
            Move::Move(from, Pos::from_xy(x + 1, end_y)),
            Move::Move(from, Pos::from_xy(x - 1, end_y)),
        ]);

        assert_eq!(board.get_moves_for(x, y).unwrap(), &correct_moves);
    };

    check(&mut game, 1, 0, 2);
    check(&mut game, 6, 0, 2);

    game.flip_color();
    check(&mut game, 1, 7, 5);
    check(&mut game, 6, 7, 5);
}

#[test]
fn bishop_moves_new_board() {
    let mut game = Game::new();

    assert!(game.get_moves_for(2, 0).unwrap().is_empty());
    assert!(game.get_moves_for(5, 0).unwrap().is_empty());

    game.flip_color();
    assert!(game.get_moves_for(2, 7).unwrap().is_empty());
    assert!(game.get_moves_for(5, 7).unwrap().is_empty());
}

#[test]
fn queen_moves_new_board() {
    let mut game = Game::new();

    assert!(game.get_moves_for(3, 0).unwrap().is_empty());

    game.flip_color();
    assert!(game.get_moves_for(3, 7).unwrap().is_empty());
}

#[test]
fn king_moves_new_board() {
    let mut game = Game::new();

    assert!(game.get_moves_for(4, 0).unwrap().is_empty());

    game.flip_color();
    assert!(game.get_moves_for(4, 7).unwrap().is_empty());
}
