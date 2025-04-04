#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_10;
use day_10::KnotHash;

#[test]
fn new_class_exp01() {
    let test = KnotHash::new("./data/example_01.txt", 4);
    assert_eq!(test.skip_size, 0);
    assert_eq!(test.curr_pos, 0);
    assert_eq!(test.nums, vec![0, 1, 2, 3, 4]);
    assert_eq!(test.twists, vec![3, 4, 1, 5]);
}

#[test]
fn new_class_exp02() {
    let test = KnotHash::new("./data/example_02.txt", 4);
    assert_eq!(test.skip_size, 0);
    assert_eq!(test.curr_pos, 0);
    assert_eq!(test.nums, vec![0, 1, 2, 3, 4]);
    assert_eq!(
        test.twists,
        vec![
            63, 144, 180, 149, 1, 255, 167, 84, 125, 65, 188, 0, 2, 254, 229, 24
        ]
    );
}

#[test]
fn reverse_exp01() {
    let mut test = KnotHash::new("./data/example_01.txt", 4);
    test.curr_pos = 0;
    test.skip_size = 0;
    test.nums = vec![0, 1, 2, 3, 4];
    test.reverse(3);
    assert_eq!(test.skip_size, 1);
    assert_eq!(test.curr_pos, 3);
    assert_eq!(test.nums, vec![2, 1, 0, 3, 4]);
}

#[test]
fn reverse_exp02() {
    let mut test = KnotHash::new("./data/example_01.txt", 4);
    test.curr_pos = 3;
    test.skip_size = 1;
    test.nums = vec![2, 1, 0, 3, 4];
    test.reverse(4);
    assert_eq!(test.skip_size, 2);
    assert_eq!(test.curr_pos, 3);
    assert_eq!(test.nums, vec![4, 3, 0, 1, 2]);
}

#[test]
fn reverse_exp03() {
    let mut test = KnotHash::new("./data/example_01.txt", 4);
    test.curr_pos = 3;
    test.skip_size = 2;
    test.nums = vec![4, 3, 0, 1, 2];
    test.reverse(1);
    assert_eq!(test.skip_size, 3);
    assert_eq!(test.curr_pos, 1);
    assert_eq!(test.nums, vec![4, 3, 0, 1, 2]);
}

#[test]
fn reverse_exp04() {
    let mut test = KnotHash::new("./data/example_01.txt", 4);
    test.curr_pos = 1;
    test.skip_size = 3;
    test.nums = vec![4, 3, 0, 1, 2];
    test.reverse(5);
    assert_eq!(test.skip_size, 4);
    assert_eq!(test.curr_pos, 4);
    assert_eq!(test.nums, vec![3, 4, 2, 1, 0]);
}

#[test]
fn run_a_round_exp01() {
    let mut test = KnotHash::new("./data/example_01.txt", 4);
    test.run_round();
    assert_eq!(test.skip_size, 4);
    assert_eq!(test.curr_pos, 4);
    assert_eq!(test.nums, vec![3, 4, 2, 1, 0]);
}
