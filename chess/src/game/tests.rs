use super::*;

use std::str::FromStr;

fn compare_moves(lhs: &[Move], rhs: &[Move]) {
    assert_eq!(lhs.len(), rhs.len());
    if !lhs.iter().all(|a| rhs.iter().find(|&b| a == b).is_some()) {
        panic!("\nlhs: {:?} does not equal \nrhs: {:?}\n", lhs, rhs);
    }
}

#[test]
fn pawn_moves_new_board() {
    let mut game = Game::default();

    let check_side = |game: &mut Game, y_start: u8, y_dir: i8| {
        for x in 0..8 {
            let from = Pos::new_xy(x, y_start);
            let correct_moves = [
                Move::Move(from.move_y(y_dir).unwrap()),
                Move::Move(from.move_y(y_dir * 2).unwrap()),
            ];

            assert_eq!(game.moves_for_pos(from).unwrap(), &correct_moves);
        }
    };

    check_side(&mut game, 1, 1);
    game.switch_side();
    check_side(&mut game, 6, -1);
}

#[test]
fn rook_moves_new_board() {
    let mut game = Game::default();

    assert!(game.moves_for_pos(Pos::new_xy(0, 0)).unwrap().is_empty());
    assert!(game.moves_for_pos(Pos::new_xy(7, 0)).unwrap().is_empty());

    game.switch_side();
    assert!(game.moves_for_pos(Pos::new_xy(0, 7)).unwrap().is_empty());
    assert!(game.moves_for_pos(Pos::new_xy(7, 7)).unwrap().is_empty());
}

#[test]
fn knight_moves_new_board() {
    let mut game = Game::default();

    let check = |game: &Game, pos: Pos, end_y| {
        let move_pos = Pos::new_xy(pos.x(), end_y);
        let correct_moves = [
            Move::Move(move_pos.add_x(1).unwrap()),
            Move::Move(move_pos.sub_x(1).unwrap()),
        ];

        assert_eq!(game.moves_for_pos(pos).unwrap(), &correct_moves);
    };

    check(&game, Pos::new_xy(1, 0), 2);
    check(&game, Pos::new_xy(6, 0), 2);

    game.switch_side();

    check(&game, Pos::new_xy(1, 7), 5);
    check(&game, Pos::new_xy(6, 7), 5);
}

#[test]
fn bishop_moves_new_board() {
    let mut game = Game::default();

    assert!(game.moves_for_pos(Pos::new_xy(2, 0)).unwrap().is_empty());
    assert!(game.moves_for_pos(Pos::new_xy(5, 0)).unwrap().is_empty());

    game.switch_side();
    assert!(game.moves_for_pos(Pos::new_xy(2, 7)).unwrap().is_empty());
    assert!(game.moves_for_pos(Pos::new_xy(5, 7)).unwrap().is_empty());
}

#[test]
fn queen_moves_new_board() {
    let mut game = Game::default();

    assert!(game.moves_for_pos(Pos::new_xy(3, 0)).unwrap().is_empty());

    game.switch_side();
    assert!(game.moves_for_pos(Pos::new_xy(3, 7)).unwrap().is_empty());
}

#[test]
fn king_moves_new_board() {
    let mut game = Game::default();

    assert!(game.moves_for_pos(Pos::new_xy(4, 0)).unwrap().is_empty());

    game.switch_side();
    assert!(game.moves_for_pos(Pos::new_xy(4, 7)).unwrap().is_empty());
}

#[test]
fn test_rook_moves() {
    const BOARD_STATE: &str = "
        R* N* B* Q* K* B* N* .
        P* P* P* P* P* P* P* .
        .  .  .  .  .  .  .  .
        .  n  .  .  .  .  .  R
        p  .  .  .  .  .  .  P
        r  .  .  .  .  .  .  .
        .  p* p* p* p* p* p* p*
        .  . b* q* k* b* n* r*";

    let board = Board::from_str(BOARD_STATE).unwrap();
    let game = Game::from_board(board, Color::White);

    let correct_moves = [
        // Horizontal
        Move::move_xy(6, 3),
        Move::move_xy(5, 3),
        Move::move_xy(4, 3),
        Move::move_xy(3, 3),
        Move::move_xy(2, 3),
        Move::move_xy(1, 3),
        // Vertical
        Move::move_xy(7, 2),
        Move::move_xy(7, 1),
        Move::move_xy(7, 0),
    ];

    let moves = game.moves_for_pos(Pos::new_xy(7, 3)).unwrap();
    compare_moves(moves, &correct_moves);
}

#[test]
fn test_bishop_moves() {
    const BOARD_STATE: &str = "
        R* N* B* Q* K* B* N* R*
        P* P* P* . P* P* P* P*
        .  .  .  .  .  .  .  .
        .  .  .  P  .  .  .  .
        .  .  .  .  .  .  .  .
        n  .  .  .  .  .  .  .
        p* p* p* p* p* p* p* p*
        r* . b* q* k* b* n* r*";

    let board = Board::from_str(BOARD_STATE).unwrap();
    let game = Game::from_board(board, Color::White);

    let correct_moves = [
        Move::move_xy(3, 1),
        Move::move_xy(4, 2),
        Move::move_xy(5, 3),
        Move::move_xy(6, 4),
        Move::move_xy(7, 5),
    ];

    let moves = game.moves_for_pos(Pos::new_xy(2, 0)).unwrap();
    compare_moves(moves, &correct_moves);
}

#[test]
fn test_queen_moves() {
    const BOARD_STATE: &str = "
        R* N* B* .  K* B* N* R*
        P* P* P* P* . P* P* P*
        .  .  .  .  .  .  .  .
        .  .  .  .  P  .  .  .
        p  .  .  .  .  .  .  Q
        r  .  .  .  .  .  .  .
        .  p* p* p* p* p* p* p*
        .  n* b* q* k* b* n* r*";

    let board = Board::from_str(BOARD_STATE).unwrap();
    let game = Game::from_board(board, Color::White);

    let correct_moves = [
        // Horizontal
        Move::move_xy(6, 4),
        Move::move_xy(5, 4),
        Move::move_xy(4, 4),
        Move::move_xy(3, 4),
        Move::move_xy(2, 4),
        Move::move_xy(1, 4),
        Move::move_xy(0, 4),
        // Vertical
        Move::move_xy(7, 3),
        Move::move_xy(7, 2),
        Move::move_xy(7, 5),
        Move::move_xy(7, 6),
        // Diag
        Move::move_xy(6, 5),
        Move::move_xy(5, 6),
        Move::move_xy(6, 3),
        Move::move_xy(5, 2),
        Move::move_xy(4, 1),
        Move::move_xy(3, 0),
    ];

    let moves = game.moves_for_pos(Pos::new_xy(7, 4)).unwrap();
    compare_moves(moves, &correct_moves)
}

#[test]
fn test_king_moves() {
    const BOARD_STATE: &str = "
        R* N* B* Q* .  B* N* R*
        P* P* P* P* K  P* P* P*
        .  .  .  .  .  .  .  .
        .  .  .  .  P  .  .  .
        .  .  .  .  .  .  .  .
        n  .  .  p  .  .  .  .
        p* p* p* .  p* p* p* p*
        r* .  b* q* k* b* n* r*";

    let board = Board::from_str(BOARD_STATE).unwrap();
    let game = Game::from_board(board, Color::White);

    let correct_moves = [
        Move::move_xy(4, 0),
        Move::move_xy(3, 2),
        Move::move_xy(5, 2),
        Move::move_xy(4, 2),
    ];

    let moves = game.moves_for_pos(Pos::new_xy(4, 1)).unwrap();
    compare_moves(moves, &correct_moves);
}

#[test]
fn test_with_whole_game() {
    let mut game = Game::default();
    let moves = "
            e4 e6 d4 d5 Nd2 Nf6 e5 Nfd7 f4 c5 c3 Nc6 Ndf3 cxd4 cxd4 f6 Bd3 Bb4+ Bd2 
            Qb6 Ne2 fxe5 fxe5 O-O a3 Be7 Qc2 Rxf3 gxf3 Nxd4 Nxd4 Qxd4 O-O-O Nxe5 Bxh7+ 
            Kh8 Kb1 Qh4 Bc3 Bf6 f4 Nc4 Bxf6 Qxf6 Bd3 b5 Qe2 Bd7 Rhg1 Be8 Rde1 Bf7 Rg3 
            Rc8 Reg1 Nd6 Rxg7 Nf5 R7g5 Rc7 Bxf5 exf5 Rh5+ Bxh5 Qxh5+ Qh6 Qxh6+ Rh7 Qf8#";

    let moves = moves.trim().split_whitespace();

    for (i, str_move) in moves.enumerate() {
        let (from, actual_move) = game.parse_pgn_move(str_move);
        assert_ne!(actual_move, Move::None);

        let result = game.play(from, actual_move);
        if result == GameResult::Checkmate {
            println!("{}", str_move);
            assert_eq!(game.current_color(), Color::Black);
            assert_eq!(i, 68);
            break;
        }
    }
}
