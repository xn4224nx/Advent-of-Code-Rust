#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_06;
use day_06::{find_freq_msg, read_signal_data};

#[test]
fn read_signal_data_exp1() {
    assert_eq!(
        read_signal_data("./data/example_01.txt"),
        vec![
            vec![101, 101, 100, 97, 100, 110],
            vec![100, 114, 118, 116, 101, 101],
            vec![101, 97, 110, 100, 115, 114],
            vec![114, 97, 97, 118, 114, 100],
            vec![97, 116, 101, 118, 114, 115],
            vec![116, 115, 114, 110, 101, 118],
            vec![115, 100, 116, 116, 115, 97],
            vec![114, 97, 115, 114, 116, 118],
            vec![110, 115, 115, 100, 116, 115],
            vec![110, 116, 110, 97, 100, 97],
            vec![115, 118, 101, 116, 118, 101],
            vec![116, 101, 115, 110, 118, 116],
            vec![118, 110, 116, 115, 110, 100],
            vec![118, 114, 100, 101, 97, 114],
            vec![100, 118, 114, 115, 101, 110],
            vec![101, 110, 97, 114, 97, 114],
        ]
    )
}

#[test]
fn find_freq_msg_exp1() {
    assert_eq!(
        find_freq_msg(&read_signal_data("./data/example_01.txt"), false),
        String::from("easter")
    )
}

#[test]
fn find_freq_msg_exp2() {
    assert_eq!(
        find_freq_msg(&read_signal_data("./data/example_01.txt"), true),
        String::from("advent")
    )
}
