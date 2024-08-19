#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_08;
use day_08::{encoded_str_len, parse_data, raw_str_len, str_len};

#[test]
fn str_len_ex_01() {
    assert_eq!(str_len(&r#""""#.to_string()), 0)
}

#[test]
fn str_len_ex_02() {
    assert_eq!(str_len(&r#""abc""#.to_string()), 3)
}

#[test]
fn str_len_ex_03() {
    assert_eq!(str_len(&r#""aaa\"aaa""#.to_string()), 7)
}

#[test]
fn str_len_ex_04() {
    assert_eq!(str_len(&r#""\x27""#.to_string()), 1)
}

#[test]
fn raw_str_len_ex_01() {
    assert_eq!(raw_str_len(&r#""""#.to_string()), 2)
}

#[test]
fn raw_str_len_ex_02() {
    assert_eq!(raw_str_len(&r#""abc""#.to_string()), 5)
}

#[test]
fn raw_str_len_ex_03() {
    assert_eq!(raw_str_len(&r#""aaa\"aaa""#.to_string()), 10)
}

#[test]
fn raw_str_len_ex_04() {
    assert_eq!(raw_str_len(&r#""\x27""#.to_string()), 6)
}

#[test]
fn ecn_str_len_ex_01() {
    assert_eq!(encoded_str_len(&r#""""#.to_string()), 6)
}

#[test]
fn ecn_str_len_ex_02() {
    assert_eq!(encoded_str_len(&r#""abc""#.to_string()), 9)
}

#[test]
fn ecn_str_len_ex_03() {
    assert_eq!(encoded_str_len(&r#""aaa\"aaa""#.to_string()), 16)
}

#[test]
fn ecn_str_len_ex_04() {
    assert_eq!(encoded_str_len(&r#""\x27""#.to_string()), 11)
}

#[test]
fn check_read() {
    assert_eq!(parse_data("./data/example_01.txt"), (12, 19))
}
