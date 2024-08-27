#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_13;
use day_13::{find_maximum_happy, read_guest_prefs};

#[test]
fn read_guest_prefs_ex_01() {
    assert_eq!(read_guest_prefs("./data/example_01.txt").len(), 12);
}

#[test]
fn test_find_max_happy_ex_01() {
    assert_eq!(
        find_maximum_happy(&read_guest_prefs("./data/example_01.txt"), false),
        330
    );
}

#[test]
fn test_find_max_happy_with_me_ex_01() {
    assert_eq!(
        find_maximum_happy(&read_guest_prefs("./data/example_01.txt"), true),
        286
    );
}
