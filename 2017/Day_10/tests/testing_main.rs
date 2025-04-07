#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_10;
use day_10::KnotHash;

#[test]
fn new_class_exp01() {
    let test = KnotHash::new("./data/example_01.txt", 4, false);
    assert_eq!(test.skip_size, 0);
    assert_eq!(test.curr_pos, 0);
    assert_eq!(test.nums, vec![0, 1, 2, 3, 4]);
    assert_eq!(test.twists, vec![3, 4, 1, 5]);
}

#[test]
fn new_class_exp02() {
    let test = KnotHash::new("./data/example_02.txt", 4, false);
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
    let mut test = KnotHash::new("./data/example_01.txt", 4, false);
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
    let mut test = KnotHash::new("./data/example_01.txt", 4, false);
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
    let mut test = KnotHash::new("./data/example_01.txt", 4, false);
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
    let mut test = KnotHash::new("./data/example_01.txt", 4, false);
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
    let mut test = KnotHash::new("./data/example_01.txt", 4, false);
    test.run_round();
    assert_eq!(test.skip_size, 4);
    assert_eq!(test.curr_pos, 4);
    assert_eq!(test.nums, vec![3, 4, 2, 1, 0]);
}

#[test]
fn read_twist_as_ascii_exp01() {
    assert_eq!(
        KnotHash::new("./data/example_05.txt", 4, true).twists,
        vec![49, 44, 50, 44, 51, 17, 31, 73, 47, 23]
    );
}

#[test]
fn read_twist_as_ascii_exp02() {
    assert_eq!(
        KnotHash::new("./data/example_03.txt", 4, true).twists,
        vec![17, 31, 73, 47, 23]
    );
}

#[test]
fn digest_exp03() {
    assert_eq!(
        KnotHash::new("./data/example_03.txt", 255, true).digest(),
        String::from("a2582a3a0e66e6e86e3812dcb672a272")
    );
}

#[test]
fn digest_exp04() {
    assert_eq!(
        KnotHash::new("./data/example_04.txt", 255, true).digest(),
        String::from("33efeb34ea91902bb2f59c9920caa6cd")
    );
}

#[test]
fn digest_exp05() {
    assert_eq!(
        KnotHash::new("./data/example_05.txt", 255, true).digest(),
        String::from("3efbe78a8d82f29979031a4aa0b16a9d")
    );
}

#[test]
fn digest_exp06() {
    assert_eq!(
        KnotHash::new("./data/example_06.txt", 255, true).digest(),
        String::from("63960835bcdc130f0b66d7ff4f6a5a8e")
    );
}
