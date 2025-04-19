#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_13;
use day_13::Firewall;
use std::collections::HashMap;

#[test]
fn parse_data_exp01() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").info,
        HashMap::from([(0, 3), (1, 2), (4, 4), (6, 4),])
    );
}

#[test]
fn detect_scanner_at_top_exp01() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").scanner_at_top(0, 0),
        true
    );
}

#[test]
fn detect_scanner_at_top_exp02() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").scanner_at_top(1, 0),
        true
    );
}

#[test]
fn detect_scanner_at_top_exp03() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").scanner_at_top(4, 0),
        true
    );
}

#[test]
fn detect_scanner_at_top_exp04() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").scanner_at_top(6, 0),
        true
    );
}

#[test]
fn detect_scanner_at_top_exp05() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").scanner_at_top(0, 1),
        false
    );
}

#[test]
fn detect_scanner_at_top_exp06() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").scanner_at_top(1, 1),
        false
    );
}

#[test]
fn detect_scanner_at_top_exp07() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").scanner_at_top(4, 1),
        false
    );
}

#[test]
fn detect_scanner_at_top_exp08() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").scanner_at_top(6, 1),
        false
    );
}

#[test]
fn detect_scanner_at_top_exp09() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").scanner_at_top(0, 2),
        false
    );
}

#[test]
fn detect_scanner_at_top_exp10() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").scanner_at_top(1, 2),
        true
    );
}

#[test]
fn detect_scanner_at_top_exp11() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").scanner_at_top(4, 2),
        false
    );
}

#[test]
fn detect_scanner_at_top_exp12() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").scanner_at_top(6, 2),
        false
    );
}

#[test]
fn detect_scanner_at_top_exp13() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").scanner_at_top(0, 3),
        false
    );
}

#[test]
fn detect_scanner_at_top_exp14() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").scanner_at_top(1, 3),
        false
    );
}

#[test]
fn detect_scanner_at_top_exp15() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").scanner_at_top(4, 3),
        false
    );
}

#[test]
fn detect_scanner_at_top_exp16() {
    assert_eq!(
        Firewall::new("./data/example_01.txt").scanner_at_top(6, 3),
        false
    );
}

#[test]
fn trip_severity_exp01() {
    assert_eq!(Firewall::new("./data/example_01.txt").trip_severity(), 24);
}
