#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_06;
use day_06::{read_instrucs, sum_nord_lights};

#[test]
fn sum_exp_04() {
    let data = read_instrucs("./data/example_04.txt");
    assert_eq!(sum_nord_lights(&data, 0), 1);
}

#[test]
fn sum_exp_05() {
    let data = read_instrucs("./data/example_05.txt");
    assert_eq!(sum_nord_lights(&data, 0), 2_000_000);
}
