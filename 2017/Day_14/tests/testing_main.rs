#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_14;
use day_14::DiskDefrag;
use std::collections::HashSet;

#[test]
fn parse_data_exp01() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(0, 0)));
}

#[test]
fn parse_data_exp02() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(1, 0)));
}

#[test]
fn parse_data_exp03() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(3, 0)));
}

#[test]
fn parse_data_exp04() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(5, 0)));
}

#[test]
fn parse_data_exp05() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(1, 1)));
}

#[test]
fn parse_data_exp06() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(3, 1)));
}

#[test]
fn parse_data_exp07() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(5, 1)));
}

#[test]
fn parse_data_exp08() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(7, 1)));
}

#[test]
fn parse_data_exp09() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(4, 2)));
}

#[test]
fn parse_data_exp10() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(6, 2)));
}

#[test]
fn parse_data_exp11() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(0, 3)));
}

#[test]
fn parse_data_exp12() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(2, 3)));
}

#[test]
fn parse_data_exp13() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(4, 3)));
}

#[test]
fn parse_data_exp14() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(5, 3)));
}

#[test]
fn parse_data_exp15() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(7, 3)));
}

#[test]
fn parse_data_exp16() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(1, 4)));
}

#[test]
fn parse_data_exp17() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(2, 4)));
}

#[test]
fn parse_data_exp18() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(4, 4)));
}

#[test]
fn parse_data_exp19() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(0, 5)));
}
#[test]
fn parse_data_exp20() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(1, 5)));
}
#[test]
fn parse_data_exp21() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(4, 5)));
}
#[test]
fn parse_data_exp22() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(7, 5)));
}

#[test]
fn parse_data_exp23() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(1, 6)));
}
#[test]
fn parse_data_exp24() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(5, 6)));
}

#[test]
fn parse_data_exp25() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(0, 7)));
}

#[test]
fn parse_data_exp26() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(1, 7)));
}

#[test]
fn parse_data_exp27() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(3, 7)));
}

#[test]
fn parse_data_exp28() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(5, 7)));
}

#[test]
fn parse_data_exp29() {
    assert!(DiskDefrag::new("flqrgnkx").used.contains(&(6, 7)));
}

#[test]
fn count_used_squares_exp01() {
    assert_eq!(DiskDefrag::new("flqrgnkx").used.len(), 8108);
}
