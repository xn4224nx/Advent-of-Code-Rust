#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_01;

#[test]
fn read_example_01() {
    assert_eq!(
        day_01::read_building_directions("./data/example_01.txt"),
        String::from("(())")
    )
}

#[test]
fn read_example_02() {
    assert_eq!(
        day_01::read_building_directions("./data/example_02.txt"),
        String::from("()()")
    )
}

#[test]
fn read_example_03() {
    assert_eq!(
        day_01::read_building_directions("./data/example_03.txt"),
        String::from("(((")
    )
}

#[test]
fn read_example_04() {
    assert_eq!(
        day_01::read_building_directions("./data/example_04.txt"),
        String::from("(()(()(")
    )
}

#[test]
fn read_example_05() {
    assert_eq!(
        day_01::read_building_directions("./data/example_05.txt"),
        String::from("))(((((")
    )
}

#[test]
fn read_example_06() {
    assert_eq!(
        day_01::read_building_directions("./data/example_06.txt"),
        String::from("())")
    )
}

#[test]
fn read_example_07() {
    assert_eq!(
        day_01::read_building_directions("./data/example_07.txt"),
        String::from("))(")
    )
}

#[test]
fn read_example_08() {
    assert_eq!(
        day_01::read_building_directions("./data/example_08.txt"),
        String::from(")))")
    )
}

#[test]
fn read_example_09() {
    assert_eq!(
        day_01::read_building_directions("./data/example_09.txt"),
        String::from(")())())")
    )
}

#[test]
fn read_example_10() {
    assert_eq!(
        day_01::read_building_directions("./data/example_10.txt"),
        String::from(")")
    )
}

#[test]
fn read_example_11() {
    assert_eq!(
        day_01::read_building_directions("./data/example_11.txt"),
        String::from("()())")
    )
}
