#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_25;
use day_25::{find_val_at_coord, next_num, place_in_order};

#[test]
fn test_next_num_01() {
    assert_eq!(next_num(20151125), 31916031);
}
#[test]
fn test_next_num_02() {
    assert_eq!(next_num(31916031), 18749137);
}
#[test]
fn test_next_num_03() {
    assert_eq!(next_num(18749137), 16080970);
}
#[test]
fn test_next_num_04() {
    assert_eq!(next_num(16080970), 21629792);
}
#[test]
fn test_next_num_05() {
    assert_eq!(next_num(21629792), 17289845);
}
#[test]
fn test_place_in_order_01() {
    assert_eq!(place_in_order(6, 1), 15);
}
#[test]
fn test_place_in_order_02() {
    assert_eq!(place_in_order(1, 6), 20);
}
#[test]
fn test_place_in_order_03() {
    assert_eq!(place_in_order(3, 4), 18);
}
#[test]
fn test_place_in_order_04() {
    assert_eq!(place_in_order(5, 2), 16);
}
#[test]
fn test_place_in_order_05() {
    assert_eq!(place_in_order(2, 5), 19);
}
#[test]
fn test_find_val_at_coord_01() {
    assert_eq!(find_val_at_coord(20151125, 6, 6), 27995004);
}
#[test]
fn test_find_val_at_coord_02() {
    assert_eq!(find_val_at_coord(20151125, 6, 1), 33071741);
}
#[test]
fn test_find_val_at_coord_03() {
    assert_eq!(find_val_at_coord(20151125, 3, 4), 7981243);
}
#[test]
fn test_find_val_at_coord_04() {
    assert_eq!(find_val_at_coord(20151125, 5, 2), 17552253);
}
#[test]
fn test_find_val_at_coord_05() {
    assert_eq!(find_val_at_coord(20151125, 1, 6), 33511524);
}
