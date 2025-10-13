#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod main;
use main::is_valid_num;

#[test]
fn test_is_valid_num_exp0() {
    assert_eq!(is_valid_num(&vec![1, 2, 2, 3, 4, 5], true), true);
}

#[test]
fn test_is_valid_num_exp1() {
    assert_eq!(is_valid_num(&vec![1, 1, 1, 1, 2, 3], true), true);
}

#[test]
fn test_is_valid_num_exp2() {
    assert_eq!(is_valid_num(&vec![1, 3, 5, 6, 7, 9], true), false);
}

#[test]
fn test_is_valid_num_exp3() {
    assert_eq!(is_valid_num(&vec![1, 1, 1, 1, 1, 1], true), true);
}

#[test]
fn test_is_valid_num_exp4() {
    assert_eq!(is_valid_num(&vec![2, 2, 3, 4, 5, 0], true), false);
}

#[test]
fn test_is_valid_num_exp5() {
    assert_eq!(is_valid_num(&vec![1, 2, 3, 7, 8, 9], true), false);
}

#[test]
fn test_is_valid_num_exp6() {
    assert_eq!(is_valid_num(&vec![1, 1, 2, 2, 3, 3], false), true);
}

#[test]
fn test_is_valid_num_exp7() {
    assert_eq!(is_valid_num(&vec![1, 2, 3, 4, 4, 4], false), false);
}

#[test]
fn test_is_valid_num_exp8() {
    assert_eq!(is_valid_num(&vec![1, 1, 1, 1, 2, 2], false), true);
}
