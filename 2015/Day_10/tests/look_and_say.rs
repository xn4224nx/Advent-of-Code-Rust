#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_10;
use day_10::look_and_say;

#[test]
fn look_and_say_ex_01() {
    assert_eq!(look_and_say("1".to_string()), "11".to_string());
}

#[test]
fn look_and_say_ex_02() {
    assert_eq!(look_and_say("11".to_string()), "21".to_string());
}

#[test]
fn look_and_say_ex_03() {
    assert_eq!(look_and_say("21".to_string()), "1211".to_string());
}

#[test]
fn look_and_say_ex_04() {
    assert_eq!(look_and_say("1211".to_string()), "111221".to_string());
}

#[test]
fn look_and_say_ex_05() {
    assert_eq!(look_and_say("111221".to_string()), "312211".to_string());
}
