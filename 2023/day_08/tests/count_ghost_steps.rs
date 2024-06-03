#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_08;

#[test]
fn ghost_map_ex3() {
    let (turns, raw_map) = day_08::read_map_data("./data/example_03.txt");
    let desert_map = day_08::parse_map(raw_map);
    assert_eq!(6, day_08::count_ghost_steps(&turns, &desert_map));
}

#[test]
fn ghost_map_input() {
    let (turns, raw_map) = day_08::read_map_data("./data/input.txt");
    let desert_map = day_08::parse_map(raw_map);
    assert_eq!(
        9858474970153,
        day_08::count_ghost_steps(&turns, &desert_map)
    );
}
