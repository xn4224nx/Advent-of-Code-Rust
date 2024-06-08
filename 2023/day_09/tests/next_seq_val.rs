#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_09;

#[test]
fn next_seq_val_00() {
    assert_eq!(day_09::next_seq_val(&vec![0, 3, 6, 9, 12, 15]), 18)
}

#[test]
fn next_seq_val_01() {
    assert_eq!(day_09::next_seq_val(&vec![1, 3, 6, 10, 15, 21]), 28)
}

#[test]
fn next_seq_val_02() {
    assert_eq!(day_09::next_seq_val(&vec![10, 13, 16, 21, 30, 45]), 68)
}
