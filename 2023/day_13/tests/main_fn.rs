#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_13;

#[test]
fn read_example_00() {
    assert_eq!(
        day_13::read_raw_notes("./data/example_00.txt"),
        vec![[
            String::from("#.##..##."),
            String::from("..#.##.#."),
            String::from("##......#"),
            String::from("##......#"),
            String::from("..#.##.#."),
            String::from("..##..##."),
            String::from("#.#.##.#.")
        ]]
    );
}

#[test]
fn read_example_01() {
    assert_eq!(
        day_13::read_raw_notes("./data/example_01.txt"),
        vec![[
            String::from("#...##..#"),
            String::from("#....#..#"),
            String::from("..##..###"),
            String::from("#####.##."),
            String::from("#####.##."),
            String::from("..##..###"),
            String::from("#....#..#")
        ]]
    );
}
