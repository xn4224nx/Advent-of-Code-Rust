#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_15;
use day_15::Sculpture;

#[test]
fn reading_data() {
    let test = Sculpture::new("./data/example_01.txt", false);
    assert_eq!(test.dsk_total_pos, vec![5, 2]);
    assert_eq!(test.dsk_start_pos, vec![4, 1]);

    let test = Sculpture::new("./data/input.txt", false);
    assert_eq!(test.dsk_total_pos, vec![5, 13, 17, 3, 19, 7]);
    assert_eq!(test.dsk_start_pos, vec![2, 7, 10, 2, 9, 0]);
}

#[test]
fn can_drops_happen() {
    let test = Sculpture::new("./data/example_01.txt", false);

    assert_eq!(test.can_drop_happen(0), false);
    assert_eq!(test.can_drop_happen(1), false);
    assert_eq!(test.can_drop_happen(2), false);
    assert_eq!(test.can_drop_happen(3), false);
    assert_eq!(test.can_drop_happen(4), false);
    assert_eq!(test.can_drop_happen(5), true);
}

#[test]
fn find_the_first_drop() {
    let test = Sculpture::new("./data/example_01.txt", false);
    assert_eq!(test.find_first_drop_time(), 5)
}
