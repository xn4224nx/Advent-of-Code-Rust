#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_11;
use day_11::{increm_str, is_pass_valid, next_valid_pass};

#[test]
fn is_pass_valid_ex_01() {
    assert_eq!(is_pass_valid("hijklmmn".to_string()), false);
}

#[test]
fn is_pass_valid_ex_02() {
    assert_eq!(is_pass_valid("abbceffg".to_string()), false);
}

#[test]
fn is_pass_valid_ex_03() {
    assert_eq!(is_pass_valid("abbcegjk".to_string()), false);
}

#[test]
fn is_pass_valid_ex_04() {
    assert_eq!(is_pass_valid("abcdffaa".to_string()), true);
}

#[test]
fn is_pass_valid_ex_05() {
    assert_eq!(is_pass_valid("ghjaabcc".to_string()), true);
}

#[test]
fn is_pass_valid_ex_06() {
    assert_eq!(is_pass_valid("abcdefgh".to_string()), false);
}

#[test]
fn is_pass_valid_ex_07() {
    assert_eq!(is_pass_valid("ghijklmn".to_string()), false);
}

#[test]
fn next_valid_pass_ex_01() {
    let mut passw = String::from("abcdefgh");
    next_valid_pass(&mut passw);
    assert_eq!(passw, "abcdffaa".to_string());
}

#[test]
fn next_valid_pass_ex_02() {
    let mut passw = String::from("ghijklmn");
    next_valid_pass(&mut passw);
    assert_eq!(passw, "ghjaabcc".to_string());
}

#[test]
fn increm_str_ex_01() {
    let mut passw = String::from("aaaaaa");
    increm_str(&mut passw);
    assert_eq!(passw, "aaaaab".to_string());
}

#[test]
fn increm_str_ex_02() {
    let mut passw = String::from("aaaaaz");
    increm_str(&mut passw);
    assert_eq!(passw, "aaaaba".to_string());
}

#[test]
fn increm_str_ex_03() {
    let mut passw = String::from("aaaabz");
    increm_str(&mut passw);
    assert_eq!(passw, "aaaaca".to_string());
}

#[test]
fn increm_str_ex_04() {
    let mut passw = String::from("aaazzz");
    increm_str(&mut passw);
    assert_eq!(passw, "aabaaa".to_string());
}
