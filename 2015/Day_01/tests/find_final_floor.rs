#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_01;

#[test]
fn final_floor_example_01() {
    assert_eq!(day_01::find_final_floor(&String::from("(())"), false), 0)
}

#[test]
fn final_floor_example_02() {
    assert_eq!(day_01::find_final_floor(&String::from("()()"), false), 0)
}

#[test]
fn final_floor_example_03() {
    assert_eq!(day_01::find_final_floor(&String::from("((("), false), 3)
}

#[test]
fn final_floor_example_04() {
    assert_eq!(day_01::find_final_floor(&String::from("(()(()("), false), 3)
}

#[test]
fn final_floor_example_05() {
    assert_eq!(day_01::find_final_floor(&String::from("))((((("), false), 3)
}

#[test]
fn final_floor_example_06() {
    assert_eq!(day_01::find_final_floor(&String::from("())"), false), -1)
}

#[test]
fn final_floor_example_07() {
    assert_eq!(day_01::find_final_floor(&String::from("))("), false), -1)
}

#[test]
fn final_floor_example_08() {
    assert_eq!(day_01::find_final_floor(&String::from(")))"), false), -3)
}

#[test]
fn final_floor_example_09() {
    assert_eq!(
        day_01::find_final_floor(&String::from(")())())"), false),
        -3
    )
}

#[test]
fn final_floor_example_10() {
    assert_eq!(day_01::find_final_floor(&String::from(")"), true), 1)
}

#[test]
fn final_floor_example_11() {
    assert_eq!(day_01::find_final_floor(&String::from("()())"), true), 5)
}
