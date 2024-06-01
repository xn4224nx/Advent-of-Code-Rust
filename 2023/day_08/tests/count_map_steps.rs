#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_08;

#[test]
fn count_example_01() {
    let (turns, raw_map) = day_08::read_map_data("./data/example_01.txt");
    let desert_map = day_08::parse_map(raw_map);
    let cnt = day_08::count_map_steps(turns, desert_map);
    assert_eq!(cnt, 2)
}

#[test]
fn count_example_02() {
    let (turns, raw_map) = day_08::read_map_data("./data/example_02.txt");
    let desert_map = day_08::parse_map(raw_map);
    let cnt = day_08::count_map_steps(turns, desert_map);
    assert_eq!(cnt, 6)
}

#[test]
fn count_input() {
    let (turns, raw_map) = day_08::read_map_data("./data/input.txt");
    let desert_map = day_08::parse_map(raw_map);
    let cnt = day_08::count_map_steps(turns, desert_map);
    assert_eq!(cnt, 11567)
}
