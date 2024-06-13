#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_10;

#[test]
fn maze_area_04() {
    let pmaze = day_10::Maze::new("./data/example_04.txt");
    let m_loop = pmaze.generate_maze_loop();

    assert_eq!(pmaze.calc_maze_area(&m_loop), 4);
}

#[test]
fn maze_area_05() {
    let pmaze = day_10::Maze::new("./data/example_05.txt");
    let m_loop = pmaze.generate_maze_loop();

    assert_eq!(pmaze.calc_maze_area(&m_loop), 8);
}

#[test]
fn maze_area_06() {
    let pmaze = day_10::Maze::new("./data/example_06.txt");
    let m_loop = pmaze.generate_maze_loop();

    assert_eq!(pmaze.calc_maze_area(&m_loop), 10);
}
