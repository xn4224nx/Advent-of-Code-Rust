#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_11;
use day_11::{increm_str, is_pass_valid, next_valid_pass};

#[test]
fn is_pass_valid_ex_01() {
    assert_eq!(is_pass_valid(&"hijklmmn".as_bytes().to_vec()), false);
}

#[test]
fn is_pass_valid_ex_02() {
    assert_eq!(is_pass_valid(&"abbceffg".as_bytes().to_vec()), false);
}

#[test]
fn is_pass_valid_ex_03() {
    assert_eq!(is_pass_valid(&"abbcegjk".as_bytes().to_vec()), false);
}

#[test]
fn is_pass_valid_ex_04() {
    assert_eq!(is_pass_valid(&"abcdffaa".as_bytes().to_vec()), true);
}

#[test]
fn is_pass_valid_ex_05() {
    assert_eq!(is_pass_valid(&"ghjaabcc".as_bytes().to_vec()), true);
}

#[test]
fn is_pass_valid_ex_06() {
    assert_eq!(is_pass_valid(&"abcdefgh".as_bytes().to_vec()), false);
}

#[test]
fn is_pass_valid_ex_07() {
    assert_eq!(is_pass_valid(&"ghijklmn".as_bytes().to_vec()), false);
}

#[test]
fn next_valid_pass_ex_01() {
    let mut passw = "abcdefgh".as_bytes().to_vec();
    next_valid_pass(&mut passw);
    assert_eq!(passw, "abcdffaa".as_bytes().to_vec());
}

#[test]
fn next_valid_pass_ex_02() {
    let mut passw = "ghijklmn".as_bytes().to_vec();
    next_valid_pass(&mut passw);
    assert_eq!(passw, "ghjaabcc".as_bytes().to_vec());
}

#[test]
fn increm_str_ex_01() {
    let mut passw = "aaaaaa".as_bytes().to_vec();
    increm_str(&mut passw);
    assert_eq!(passw, "aaaaab".as_bytes().to_vec());
}

#[test]
fn increm_str_ex_02() {
    let mut passw = "aaaaaz".as_bytes().to_vec();
    increm_str(&mut passw);
    assert_eq!(passw, "aaaaba".as_bytes().to_vec());
}

#[test]
fn increm_str_ex_03() {
    let mut passw = "aaaabz".as_bytes().to_vec();
    increm_str(&mut passw);
    assert_eq!(passw, "aaaaca".as_bytes().to_vec());
}

#[test]
fn increm_str_ex_04() {
    let mut passw = "aaazzz".as_bytes().to_vec();
    increm_str(&mut passw);
    assert_eq!(passw, "aabaaa".as_bytes().to_vec());
}
