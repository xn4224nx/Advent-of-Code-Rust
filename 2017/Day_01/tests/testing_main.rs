#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_1;
use day_1::AntiHumanCaptcha;

#[test]
fn reading_data_exp1() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_01.txt").vals,
        vec![1, 1, 2, 2]
    );
}

#[test]
fn reading_data_exp2() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_02.txt").vals,
        vec![1, 1, 1, 1]
    );
}

#[test]
fn reading_data_exp3() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_03.txt").vals,
        vec![1, 2, 3, 4]
    );
}

#[test]
fn reading_data_exp4() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_04.txt").vals,
        vec![9, 1, 2, 1, 2, 1, 2, 9]
    );
}

#[test]
fn reading_data_exp5() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_05.txt").vals,
        vec![1, 2, 1, 2]
    );
}

#[test]
fn reading_data_exp6() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_06.txt").vals,
        vec![1, 2, 2, 1]
    );
}

#[test]
fn reading_data_exp7() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_07.txt").vals,
        vec![1, 2, 3, 4, 2, 5]
    );
}

#[test]
fn reading_data_exp8() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_08.txt").vals,
        vec![1, 2, 3, 1, 2, 3]
    );
}

#[test]
fn reading_data_exp9() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_09.txt").vals,
        vec![1, 2, 1, 3, 1, 4, 1, 5]
    );
}

#[test]
fn ring_sum_exp1() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_01.txt").ring_sum(true),
        3
    );
}

#[test]
fn ring_sum_exp2() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_02.txt").ring_sum(true),
        4
    );
}

#[test]
fn ring_sum_exp3() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_03.txt").ring_sum(true),
        0
    );
}

#[test]
fn ring_sum_exp4() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_04.txt").ring_sum(true),
        9
    );
}

#[test]
fn ring_sum_exp5() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_05.txt").ring_sum(false),
        6
    );
}

#[test]
fn ring_sum_exp6() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_06.txt").ring_sum(false),
        0
    );
}

#[test]
fn ring_sum_exp7() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_07.txt").ring_sum(false),
        4
    );
}

#[test]
fn ring_sum_exp8() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_08.txt").ring_sum(false),
        12
    );
}

#[test]
fn ring_sum_exp9() {
    assert_eq!(
        AntiHumanCaptcha::new("./data/example_09.txt").ring_sum(false),
        4
    );
}
