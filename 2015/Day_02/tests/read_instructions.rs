#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_02;

#[test]
fn read_example_01() {
    assert_eq!(
        day_02::read_instructions("./data/example_01.txt"),
        vec![(2, 3, 4)]
    )
}

#[test]
fn read_example_02() {
    assert_eq!(
        day_02::read_instructions("./data/example_02.txt"),
        vec![(1, 1, 10)]
    )
}

#[test]
fn read_example_03() {
    assert_eq!(
        day_02::read_instructions("./data/example_03.txt"),
        vec![(2, 3, 4), (1, 1, 10)]
    )
}
