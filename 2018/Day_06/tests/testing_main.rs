#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod main;
use crate::main::MineField;

#[test]
fn read_minefield_exp0() {
    let test = MineField::new("./data/example_0.txt");
    assert_eq!(
        test.mine_coords,
        vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9),]
    );
    assert_eq!(test.top_left_corner, (1, 1));
    assert_eq!(test.bottom_right_corner, (8, 9));
}

#[test]
fn largest_connected_area_exp0() {
    assert_eq!(
        MineField::new("./data/example_0.txt").largest_connected_area(),
        17
    )
}

#[test]
fn close_region_size_exp0() {
    assert_eq!(
        MineField::new("./data/example_0.txt").close_region_size(32),
        16
    )
}
