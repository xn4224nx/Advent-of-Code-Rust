#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_21;
use day_21::{Command, SecretHasher};
use std::collections::VecDeque;

#[test]
fn reading_data() {
    let test = SecretHasher::new("abcde", "./data/example_01.txt");

    assert_eq!(test.initial_letters, vec!['a', 'b', 'c', 'd', 'e']);
    assert_eq!(test.curr_letters, VecDeque::from(['a', 'b', 'c', 'd', 'e']));
    assert_eq!(
        test.instructs,
        vec![
            Command::SwapIndex(4, 0),
            Command::SwapLetter('d', 'b'),
            Command::Reverse(0, 4),
            Command::Rotate(-1),
            Command::Move(1, 4),
            Command::Move(3, 0),
            Command::RotateLetter('b'),
            Command::RotateLetter('d')
        ]
    );
}

#[test]
fn show_current_letters() {
    let test = SecretHasher::new("abcde", "./data/example_01.txt");
    assert_eq!(test.show(), String::from("abcde"));
}

#[test]
fn swap_position_index() {
    let mut test = SecretHasher::new("abcde", "./data/example_01.txt");
    test.impl_command(0);
    assert_eq!(test.curr_letters, VecDeque::from(['e', 'b', 'c', 'd', 'a']));
}

#[test]
fn swap_position_letter() {
    let mut test = SecretHasher::new("abcde", "./data/example_01.txt");
    test.curr_letters = VecDeque::from(['e', 'b', 'c', 'd', 'a']);
    test.impl_command(1);
    assert_eq!(test.curr_letters, VecDeque::from(['e', 'd', 'c', 'b', 'a']));
}

#[test]
fn rotate_left_right() {
    let mut test = SecretHasher::new("abcde", "./data/example_01.txt");
    test.curr_letters = VecDeque::from(['a', 'b', 'c', 'd', 'e']);
    test.impl_command(3);
    assert_eq!(test.curr_letters, VecDeque::from(['b', 'c', 'd', 'e', 'a']));
}

#[test]
fn rotate_based_on_letter() {
    let mut test = SecretHasher::new("abcde", "./data/example_01.txt");
    test.curr_letters = VecDeque::from(['a', 'b', 'd', 'e', 'c']);
    test.impl_command(6);
    assert_eq!(test.curr_letters, VecDeque::from(['e', 'c', 'a', 'b', 'd']));
    test.impl_command(7);
    assert_eq!(test.curr_letters, VecDeque::from(['d', 'e', 'c', 'a', 'b']));
}

#[test]
fn reverse_positions() {
    let mut test = SecretHasher::new("abcde", "./data/example_01.txt");
    test.curr_letters = VecDeque::from(['e', 'd', 'c', 'b', 'a']);
    test.impl_command(2);
    assert_eq!(test.curr_letters, VecDeque::from(['a', 'b', 'c', 'd', 'e']));
}

#[test]
fn move_position_indexes() {
    let mut test = SecretHasher::new("abcde", "./data/example_01.txt");
    test.curr_letters = VecDeque::from(['b', 'c', 'd', 'e', 'a']);
    test.impl_command(4);
    assert_eq!(test.curr_letters, VecDeque::from(['b', 'd', 'e', 'a', 'c']));
    test.impl_command(5);
    assert_eq!(test.curr_letters, VecDeque::from(['a', 'b', 'd', 'e', 'c']));
}

#[test]
fn find_final_state() {
    let mut test = SecretHasher::new("abcde", "./data/example_01.txt");
    assert_eq!(test.final_state(), String::from("decab"));
}
