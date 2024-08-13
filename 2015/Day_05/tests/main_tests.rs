#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_05;

#[test]
fn naughty_or_nice_ex1() {
    assert_eq!(
        day_05::is_nice(&String::from("ugknbfddgicrmopn").into_bytes()),
        true
    )
}

#[test]
fn naughty_or_nice_ex2() {
    assert_eq!(day_05::is_nice(&String::from("aaa").into_bytes()), true)
}

#[test]
fn naughty_or_nice_ex3() {
    assert_eq!(
        day_05::is_nice(&String::from("jchzalrnumimnmhp").into_bytes()),
        false
    )
}

#[test]
fn naughty_or_nice_ex4() {
    assert_eq!(
        day_05::is_nice(&String::from("haegwjzuvuyypxyu").into_bytes()),
        false
    )
}

#[test]
fn naughty_or_nice_ex5() {
    assert_eq!(
        day_05::is_nice(&String::from("dvszwmarrgswjxmb").into_bytes()),
        false
    )
}
