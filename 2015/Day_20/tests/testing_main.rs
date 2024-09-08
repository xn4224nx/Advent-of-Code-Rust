#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_20;
use day_20::{deliver_presents, find_lowest_house};

#[test]
fn deliver_presents_ex01() {
    let houses = deliver_presents(100, false);
    assert_eq!(houses[1..10], vec![10, 30, 40, 70, 60, 120, 80, 150, 130]);
}

#[test]
fn find_lowest_house_ex01() {
    let houses = deliver_presents(100, false);
    assert_eq!(find_lowest_house(&houses, 100), 6);
}
