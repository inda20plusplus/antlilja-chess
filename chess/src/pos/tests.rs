use super::*;

macro_rules! assert_panic {
    ($expression:expr) => {
        if !std::panic::catch_unwind(|| $expression).is_err() {
            panic!(
                "Assertion didn't panic on expression: {}",
                stringify!($expression)
            );
        }
    };
}

#[test]
fn equal_to_index() {
    for x in 0..8 {
        for y in 0..8 {
            let index = (y * 8 + x) as usize;
            let pos_index = Pos::new_xy(x, y).index();

            assert_eq!(index, pos_index);
        }
    }
}

#[test]
fn new_xy_invalid() {
    for x in 8..=255 {
        for y in 0..8 {
            assert_panic!(Pos::new_xy(x, y));
        }
    }

    for x in 0..8 {
        for y in 8..=255 {
            assert_panic!(Pos::new_xy(x, y));
        }
    }
}

#[test]
fn new_index_invalid() {
    for i in 64..=255 {
        assert_panic!(Pos::new_index(i));
    }
}

#[test]
fn x_and_y_correct() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            assert_eq!(x, pos.x());
            assert_eq!(y, pos.y());
        }
    }
}

#[test]
fn xy_correct() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            assert_eq!((x, y), pos.xy());
        }
    }
}

#[test]
fn add_x() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            for x2 in 0..8 {
                let new_pos = pos.add_x(x2);
                if x + x2 > 7 {
                    assert_eq!(new_pos, None);
                } else {
                    let new_pos = new_pos.unwrap();
                    assert_eq!(new_pos.x(), x + x2);
                    assert_eq!(new_pos.y(), y);
                }
            }
        }
    }
}

#[test]
fn sub_x() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            for x2 in 0..8 {
                let new_pos = pos.sub_x(x2);
                if (x as i8 - x2 as i8) < 0 {
                    assert_eq!(new_pos, None);
                } else {
                    let new_pos = new_pos.unwrap();
                    assert_eq!(new_pos.x(), x - x2);
                    assert_eq!(new_pos.y(), y);
                }
            }
        }
    }
}

#[test]
fn move_x() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            for x2 in -8..8 {
                let new_pos = pos.move_x(x2);
                let new_x = x as i8 + x2;
                if !(0..8).contains(&new_x) {
                    assert_eq!(new_pos, None);
                } else {
                    let new_pos = new_pos.unwrap();
                    assert_eq!(new_pos.x(), (x as i8 + x2) as u8);
                    assert_eq!(new_pos.y(), y);
                }
            }
        }
    }
}

#[test]
fn add_y() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            for y2 in 0..8 {
                let new_pos = pos.add_y(y2);
                if y + y2 > 7 {
                    assert_eq!(new_pos, None);
                } else {
                    let new_pos = new_pos.unwrap();
                    assert_eq!(new_pos.x(), x);
                    assert_eq!(new_pos.y(), y + y2);
                }
            }
        }
    }
}

#[test]
fn sub_y() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            for y2 in 0..8 {
                let new_pos = pos.sub_y(y2);
                if (y as i8 - y2 as i8) < 0 {
                    assert_eq!(new_pos, None);
                } else {
                    let new_pos = new_pos.unwrap();
                    assert_eq!(new_pos.x(), x);
                    assert_eq!(new_pos.y(), y - y2);
                }
            }
        }
    }
}

#[test]
fn move_y() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            for y2 in -8..8 {
                let new_pos = pos.move_y(y2);
                let new_y = y as i8 + y2;
                if !(0..8).contains(&new_y) {
                    assert_eq!(new_pos, None);
                } else {
                    let new_pos = new_pos.unwrap();
                    assert_eq!(new_pos.x(), x);
                    assert_eq!(new_pos.y(), (y as i8 + y2) as u8);
                }
            }
        }
    }
}

#[test]
fn move_xy() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            for x2 in -8..8 {
                for y2 in -8..8 {
                    let new_pos = pos.move_xy(x2, y2);
                    let new_x = x as i8 + x2;
                    let new_y = y as i8 + y2;
                    if !(0..8).contains(&new_x) || !(0..8).contains(&new_y) {
                        assert_eq!(new_pos, None);
                    } else {
                        let new_pos = new_pos.unwrap();
                        assert_eq!(new_pos.x(), (x as i8 + x2) as u8);
                        assert_eq!(new_pos.y(), (y as i8 + y2) as u8);
                    }
                }
            }
        }
    }
}

#[test]
fn at_left_edge() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            assert_eq!(x == 0, pos.at_left_edge());
        }
    }
}

#[test]
fn at_right_edge() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            assert_eq!(x == 7, pos.at_right_edge());
        }
    }
}

#[test]
fn at_x_edge() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            assert_eq!(x == 0 || x == 7, pos.at_x_edge());
        }
    }
}

#[test]
fn at_white_edge() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            assert_eq!(y == 0, pos.at_white_edge());
        }
    }
}

#[test]
fn at_black_edge() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            assert_eq!(y == 7, pos.at_black_edge());
        }
    }
}

#[test]
fn at_y_edge() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            assert_eq!(y == 0 || y == 7, pos.at_y_edge());
        }
    }
}

#[test]
fn at_pawn_rank() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            assert_eq!(y == 1, pos.at_pawn_rank(Color::White));
            assert_eq!(y == 6, pos.at_pawn_rank(Color::Black));
        }
    }
}

#[test]
fn distance() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            for x2 in 0..8 {
                for y2 in 0..8 {
                    let pos2 = Pos::new_xy(x2, y2);
                    let dist_x = pos.distance_x(&pos2);
                    let dist_y = pos.distance_y(&pos2);
                    let rdist_x = (x as i8 - x2 as i8).abs() as u8;
                    let rdist_y = (y as i8 - y2 as i8).abs() as u8;
                    assert_eq!(dist_x, rdist_x);
                    assert_eq!(dist_y, rdist_y);
                }
            }
        }
    }
}
