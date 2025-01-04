#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_20;
use day_20::BlackList;

#[test]
fn reading_data() {
    assert_eq!(
        BlackList::new("./data/example_01.txt").ranges,
        vec![(0, 2), (4, 7), (5, 8),]
    );
}

#[test]
fn find_min_ip_address() {
    assert_eq!(
        BlackList::new("./data/example_01.txt").lowest_allowed_ip(),
        3
    );
}
