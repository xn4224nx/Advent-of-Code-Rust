#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_24;
use day_24::AirDucts;

use std::collections::HashSet;

#[test]
fn reading_data_exp1() {
    let test = AirDucts::new("./data/example_01.txt");
    assert_eq!(
        test.clear_tiles,
        HashSet::from([
            (1, 1),
            (1, 2),
            (1, 3),
            (1, 4),
            (1, 5),
            (1, 6),
            (1, 7),
            (1, 8),
            (1, 9),
            (2, 1),
            (2, 9),
            (3, 1),
            (3, 2),
            (3, 3),
            (3, 4),
            (3, 5),
            (3, 6),
            (3, 7),
            (3, 8),
            (3, 9)
        ])
    );
    assert_eq!(
        test.numbr_tiles,
        vec![(1, 1), (3, 1), (9, 1), (9, 3), (1, 3)]
    );
}

#[test]
fn next_steps() {
    let test = AirDucts::new("./data/example_01.txt");
    assert_eq!(test.next_steps((1, 1)), vec![(1, 2), (2, 1)]);
    assert_eq!(test.next_steps((1, 2)), vec![(1, 1), (1, 3)]);
    assert_eq!(test.next_steps((6, 1)), vec![(5, 1), (7, 1)]);
    assert_eq!(test.next_steps((7, 3)), vec![(6, 3), (8, 3)]);
    assert_eq!(test.next_steps((9, 3)), vec![(9, 2), (8, 3)]);
}

#[test]
fn min_numbr_dist() {
    let test = AirDucts::new("./data/example_01.txt");
    assert_eq!(test.min_numbr_dist(0, 4), 2);
    assert_eq!(test.min_numbr_dist(4, 1), 4);
    assert_eq!(test.min_numbr_dist(1, 2), 6);
    assert_eq!(test.min_numbr_dist(2, 3), 2);
    assert_eq!(test.min_numbr_dist(4, 0), 2);
    assert_eq!(test.min_numbr_dist(1, 4), 4);
    assert_eq!(test.min_numbr_dist(2, 1), 6);
    assert_eq!(test.min_numbr_dist(3, 2), 2);
}

#[test]
fn min_traversal() {
    assert_eq!(AirDucts::new("./data/example_01.txt").min_traversal(), 14);
}
