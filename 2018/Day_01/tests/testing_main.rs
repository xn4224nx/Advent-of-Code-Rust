#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_1;
use day_1::{final_freq, read_frequencies};

#[test]
fn read_frequencies_exp0() {
    assert_eq!(read_frequencies("./data/example_0.txt"), vec![1, -2, 3, 1]);
}

#[test]
fn read_frequencies_exp1() {
    assert_eq!(read_frequencies("./data/example_1.txt"), vec![1, 1, 1]);
}

#[test]
fn read_frequencies_exp2() {
    assert_eq!(read_frequencies("./data/example_2.txt"), vec![1, 1, -2]);
}

#[test]
fn read_frequencies_exp3() {
    assert_eq!(read_frequencies("./data/example_3.txt"), vec![-1, -2, -3]);
}

#[test]
fn read_frequencies_exp4() {
    assert_eq!(read_frequencies("./data/example_4.txt"), vec![1, -1]);
}

#[test]
fn read_frequencies_exp5() {
    assert_eq!(
        read_frequencies("./data/example_5.txt"),
        vec![3, 3, 4, -2, -4]
    );
}

#[test]
fn read_frequencies_exp6() {
    assert_eq!(
        read_frequencies("./data/example_6.txt"),
        vec![-6, 3, 8, 5, -6]
    );
}

#[test]
fn read_frequencies_exp7() {
    assert_eq!(
        read_frequencies("./data/example_7.txt"),
        vec![7, 7, -2, -7, -4]
    );
}

#[test]
fn final_freq_exp1() {
    assert_eq!(final_freq(&vec![1, -2, 3, 1]), 3);
}

#[test]
fn final_freq_exp2() {
    assert_eq!(final_freq(&vec![1, 1, 1]), 3);
}

#[test]
fn final_freq_exp3() {
    assert_eq!(final_freq(&vec![1, 1, -2]), 0);
}

#[test]
fn final_freq_exp4() {
    assert_eq!(final_freq(&vec![-1, -2, -3]), -6);
}
