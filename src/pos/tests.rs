use super::*;

#[test]
fn to_pos_and_back() {
    for x in 0..8 {
        for y in 0..8 {
            let pos = Pos::new_xy(x, y);
            let (pos_x, pos_y) = pos.xy();

            assert_eq!(x, pos_x);
            assert_eq!(y, pos_y);
        }
    }
}

#[test]
fn pos_equal_to_index() {
    for x in 0..8 {
        for y in 0..8 {
            let index = (y * 8 + x) as usize;
            let pos_index = Pos::new_xy(x, y).index();

            assert_eq!(index, pos_index);
        }
    }
}
