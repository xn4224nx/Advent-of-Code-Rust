#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_08;

#[test]
fn loop_ghost_path_input() {
    let (turns, raw_map) = day_08::read_map_data("./data/input.txt");
    let desert_map = day_08::parse_map(raw_map);

    assert_eq!(
        21251,
        day_08::loop_ghost_path(&String::from("BFA"), &turns, &desert_map)
    );
}

#[test]
fn loop_ghost_path_ex3() {
    let (turns, raw_map) = day_08::read_map_data("./data/example_03.txt");
    let desert_map = day_08::parse_map(raw_map);

    assert_eq!(
        2,
        day_08::loop_ghost_path(&String::from("11A"), &turns, &desert_map)
    );
}
