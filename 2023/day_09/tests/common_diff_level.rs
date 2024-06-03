#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_09;

#[test]
fn common_diff_level_00() {
    assert_eq!(day_09::common_diff_level(vec![0, 3, 6, 9, 12, 15]), (3, 1))
}

#[test]
fn common_diff_level_01() {
    assert_eq!(day_09::common_diff_level(vec![1, 3, 6, 10, 15, 21]), (1, 2))
}

#[test]
fn common_diff_level_02() {
    assert_eq!(
        day_09::common_diff_level(vec![10, 13, 16, 21, 30, 45]),
        (2, 3)
    )
}
