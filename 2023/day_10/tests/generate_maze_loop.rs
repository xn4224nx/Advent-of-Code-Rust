#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_10;

fn read_all_mazes() -> Vec<day_10::Maze> {
    return vec![
        day_10::Maze::new("./data/example_00.txt"),
        day_10::Maze::new("./data/example_01.txt"),
        day_10::Maze::new("./data/example_02.txt"),
        day_10::Maze::new("./data/example_03.txt"),
    ];
}

#[test]
fn generate_maze_00() {
    let pmaze = day_10::Maze::new("./data/example_00.txt");
    assert_eq!(pmaze.generate_maze_loop().len() / 2, 4);
}

#[test]
fn generate_maze_01() {
    let pmaze = day_10::Maze::new("./data/example_01.txt");
    assert_eq!(pmaze.generate_maze_loop().len() / 2, 4);
}

#[test]
fn generate_maze_02() {
    let pmaze = day_10::Maze::new("./data/example_02.txt");
    assert_eq!(pmaze.generate_maze_loop().len() / 2, 8);
}

#[test]
fn generate_maze_03() {
    let pmaze = day_10::Maze::new("./data/example_03.txt");
    assert_eq!(pmaze.generate_maze_loop().len() / 2, 8);
}
