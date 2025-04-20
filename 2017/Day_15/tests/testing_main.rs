#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_15;
use day_15::DualGenerator;

#[test]
fn iterate_exp01() {
    let mut test = DualGenerator::new(65, 8921);
    test.iterate();
    assert_eq!(test.a, 1092455);
    assert_eq!(test.b, 430625591);
}

#[test]
fn iterate_exp02() {
    let mut test = DualGenerator::new(1092455, 430625591);
    test.iterate();
    assert_eq!(test.a, 1181022009);
    assert_eq!(test.b, 1233683848);
}

#[test]
fn iterate_exp03() {
    let mut test = DualGenerator::new(1181022009, 1233683848);
    test.iterate();
    assert_eq!(test.a, 245556042);
    assert_eq!(test.b, 1431495498);
}

#[test]
fn iterate_exp04() {
    let mut test = DualGenerator::new(245556042, 1431495498);
    test.iterate();
    assert_eq!(test.a, 1744312007);
    assert_eq!(test.b, 137874439);
}

#[test]
fn iterate_exp05() {
    let mut test = DualGenerator::new(1744312007, 137874439);
    test.iterate();
    assert_eq!(test.a, 1352636452);
    assert_eq!(test.b, 285222916);
}

#[test]
fn compare_bits_exp01() {
    assert_eq!(
        DualGenerator::new(0, 0).compare_low_bits(1092455, 430625591),
        false
    );
}

#[test]
fn compare_bits_exp02() {
    assert_eq!(
        DualGenerator::new(0, 0).compare_low_bits(1181022009, 1233683848),
        false
    );
}

#[test]
fn compare_bits_exp03() {
    assert_eq!(
        DualGenerator::new(0, 0).compare_low_bits(245556042, 1431495498),
        true
    );
}

#[test]
fn compare_bits_exp04() {
    assert_eq!(
        DualGenerator::new(0, 0).compare_low_bits(1744312007, 137874439),
        false
    );
}

#[test]
fn compare_bits_exp05() {
    assert_eq!(
        DualGenerator::new(0, 0).compare_low_bits(1352636452, 285222916),
        false
    );
}

#[test]
fn count_next_matches_exp01() {
    assert_eq!(DualGenerator::new(65, 8921).count_next_matches(5), 1);
}

#[test]
fn count_next_matches_exp02() {
    assert_eq!(
        DualGenerator::new(65, 8921).count_next_matches(40_000_000),
        588
    );
}
