#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_02;

#[test]
fn area_example_01() {
    assert_eq!(day_02::calc_wrap_area((2, 3, 4)), 58)
}

#[test]
fn area_example_02() {
    assert_eq!(day_02::calc_wrap_area((1, 1, 10)), 43)
}
