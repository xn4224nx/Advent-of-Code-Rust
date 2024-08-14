#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_06;
use day_06::{read_instrucs, Command, Instruct};

#[test]
fn read_exp_01() {
    assert_eq!(
        read_instrucs("./data/example_01.txt"),
        vec![Instruct {
            cmd: Command::TurnOn,
            srt_x: 0,
            srt_y: 0,
            end_x: 999,
            end_y: 999,
        }]
    )
}

#[test]
fn read_exp_02() {
    assert_eq!(
        read_instrucs("./data/example_02.txt"),
        vec![Instruct {
            cmd: Command::Toggle,
            srt_x: 0,
            srt_y: 0,
            end_x: 999,
            end_y: 0,
        }]
    )
}

#[test]
fn read_exp_03() {
    assert_eq!(
        read_instrucs("./data/example_03.txt"),
        vec![Instruct {
            cmd: Command::TurnOff,
            srt_x: 499,
            srt_y: 499,
            end_x: 500,
            end_y: 500,
        }]
    )
}

#[test]
fn read_exp_04() {
    assert_eq!(
        read_instrucs("./data/example_04.txt"),
        vec![Instruct {
            cmd: Command::TurnOn,
            srt_x: 0,
            srt_y: 0,
            end_x: 0,
            end_y: 0,
        }]
    )
}

#[test]
fn read_exp_05() {
    assert_eq!(
        read_instrucs("./data/example_05.txt"),
        vec![Instruct {
            cmd: Command::Toggle,
            srt_x: 0,
            srt_y: 0,
            end_x: 999,
            end_y: 999,
        }]
    )
}
