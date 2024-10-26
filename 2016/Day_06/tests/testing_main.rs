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
            vec!['e', 'e', 'd', 'a', 'd', 'n'],
            vec!['d', 'r', 'v', 't', 'e', 'e'],
            vec!['e', 'a', 'n', 'd', 's', 'r'],
            vec!['r', 'a', 'a', 'v', 'r', 'd'],
            vec!['a', 't', 'e', 'v', 'r', 's'],
            vec!['t', 's', 'r', 'n', 'e', 'v'],
            vec!['s', 'd', 't', 't', 's', 'a'],
            vec!['r', 'a', 's', 'r', 't', 'v'],
            vec!['n', 's', 's', 'd', 't', 's'],
            vec!['n', 't', 'n', 'a', 'd', 'a'],
            vec!['s', 'v', 'e', 't', 'v', 'e'],
            vec!['t', 'e', 's', 'n', 'v', 't'],
            vec!['v', 'n', 't', 's', 'n', 'd'],
            vec!['v', 'r', 'd', 'e', 'a', 'r'],
            vec!['d', 'v', 'r', 's', 'e', 'n'],
            vec!['e', 'n', 'a', 'r', 'a', 'r'],
        ]
    )
}

#[test]
fn find_freq_msg_exp1() {
    assert_eq!(
        find_freq_msg(&read_signal_data("./data/example_01.txt")),
        String::from("easter")
    )
}
