use super::*;

#[test]
fn pawn_moves_new_board() {
    let mut game = Game::new();

    let check_side = |game: &mut Game, y_start, y_dir: i8| {
        for x in 0..8 {
            let move_to = |to_y| {
                let to = Pos::from_xy(x, to_y);
                Move::Move(to)
            };

            let correct_moves = MoveArray::from_slice(&[
                move_to((y_start as i8 + y_dir) as u8),
                move_to((y_start as i8 + y_dir * 2) as u8),
            ]);

            assert_eq!(game.get_moves_for(x, y_start).unwrap(), &correct_moves);
        }
    };

    check_side(&mut game, 1, 1);
    game.switch_side();
    check_side(&mut game, 6, -1);
}

#[test]
fn rook_moves_new_board() {
    let mut game = Game::new();

    assert!(game.get_moves_for(0, 0).unwrap().is_empty());
    assert!(game.get_moves_for(7, 0).unwrap().is_empty());

    game.switch_side();
    assert!(game.get_moves_for(0, 7).unwrap().is_empty());
    assert!(game.get_moves_for(7, 7).unwrap().is_empty());
}

#[test]
fn knight_moves_new_board() {
    let mut game = Game::new();

    let check = |game: &Game, x, y, end_y| {
        let correct_moves = MoveArray::from_slice(&[
            Move::Move(Pos::from_xy(x + 1, end_y)),
            Move::Move(Pos::from_xy(x - 1, end_y)),
        ]);

        assert_eq!(game.get_moves_for(x, y).unwrap(), &correct_moves);
    };

    check(&game, 1, 0, 2);
    check(&game, 6, 0, 2);

    game.switch_side();

    check(&game, 1, 7, 5);
    check(&game, 6, 7, 5);
}

#[test]
fn bishop_moves_new_board() {
    let mut game = Game::new();

    assert!(game.get_moves_for(2, 0).unwrap().is_empty());
    assert!(game.get_moves_for(5, 0).unwrap().is_empty());

    game.switch_side();
    assert!(game.get_moves_for(2, 7).unwrap().is_empty());
    assert!(game.get_moves_for(5, 7).unwrap().is_empty());
}

#[test]
fn queen_moves_new_board() {
    let mut game = Game::new();

    assert!(game.get_moves_for(3, 0).unwrap().is_empty());

    game.switch_side();
    assert!(game.get_moves_for(3, 7).unwrap().is_empty());
}

#[test]
fn king_moves_new_board() {
    let mut game = Game::new();

    assert!(game.get_moves_for(4, 0).unwrap().is_empty());

    game.switch_side();
    assert!(game.get_moves_for(4, 7).unwrap().is_empty());
}
