#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_02;

#[test]
fn ribb_len_example_01() {
    assert_eq!(day_02::calc_ribb_len((2, 3, 4)), 34)
}

#[test]
fn ribb_len_example_02() {
    assert_eq!(day_02::calc_ribb_len((1, 1, 10)), 14)
}
