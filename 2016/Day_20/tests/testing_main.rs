#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_20;
use day_20::BlackList;

#[test]
fn reading_data() {
    assert_eq!(
        BlackList::new("./data/example_01.txt").ranges,
        vec![(5, 8), (0, 2), (4, 7),]
    );
}

#[test]
fn range_compression() {
    let mut test = BlackList::new("./data/example_01.txt");
    test.ranges = vec![(5, 8), (0, 2), (4, 7)];
    test.compress_ranges();
    assert_eq!(test.ranges, vec![(0, 2), (4, 8)]);
}

#[test]
fn find_min_ip_address() {
    assert_eq!(
        BlackList::new("./data/example_01.txt").lowest_allowed_ip(),
        3
    );
}
