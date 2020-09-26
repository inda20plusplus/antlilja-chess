use super::*;

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
