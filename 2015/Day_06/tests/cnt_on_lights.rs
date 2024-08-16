#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_06;
use day_06::{cnt_on_lights, read_instrucs};

#[test]
fn cnt_exp_01_fal() {
    let data = read_instrucs("./data/example_01.txt");
    assert_eq!(cnt_on_lights(&data, false), 1_000_000);
}

#[test]
fn cnt_exp_01_tru() {
    let data = read_instrucs("./data/example_01.txt");
    assert_eq!(cnt_on_lights(&data, true), 1_000_000);
}

#[test]
fn cnt_exp_02_fal() {
    let data = read_instrucs("./data/example_02.txt");
    assert_eq!(cnt_on_lights(&data, false), 1_000);
}

#[test]
fn cnt_exp_02_tru() {
    let data = read_instrucs("./data/example_02.txt");
    assert_eq!(cnt_on_lights(&data, true), 999_000);
}

#[test]
fn cnt_exp_03_fal() {
    let data = read_instrucs("./data/example_03.txt");
    assert_eq!(cnt_on_lights(&data, false), 0);
}

#[test]
fn cnt_exp_03_tru() {
    let data = read_instrucs("./data/example_03.txt");
    assert_eq!(cnt_on_lights(&data, true), 999_996);
}
