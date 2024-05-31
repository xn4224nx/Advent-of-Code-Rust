#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_08;

#[test]
fn reads_turns_example_01() {
    let result = day_08::read_map_data("./data/example_01.txt");
    assert_eq!(String::from("RL"), result.0);
}

#[test]
fn reads_turns_example_02() {
    let result = day_08::read_map_data("./data/example_02.txt");
    assert_eq!(String::from("LLR"), result.0);
}

#[test]
fn reads_map_example_01() {
    let result = day_08::read_map_data("./data/example_01.txt");
    assert_eq!(String::from("ZZZ"), result.1[6].2);
}

#[test]
fn reads_map_example_02() {
    let result = day_08::read_map_data("./data/example_02.txt");
    assert_eq!(String::from("AAA"), result.1[0].0);
}

#[test]
#[should_panic]
fn panics_on_no_file() {
    let _result = day_08::read_map_data("./data/example_03.txt");
}
