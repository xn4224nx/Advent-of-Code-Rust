#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_10;

#[test]
fn loopup_pnt_maze_00() {
    let pmaze = day_10::Maze::new("./data/example_00.txt");
    assert_eq!(pmaze.pnt_2_pipe((3, 1)), 'L');
}

#[test]
fn loopup_pnt_maze_01() {
    let pmaze = day_10::Maze::new("./data/example_01.txt");
    assert_eq!(pmaze.pnt_2_pipe((3, 4)), '|');
}

#[test]
fn loopup_pnt_maze_02() {
    let pmaze = day_10::Maze::new("./data/example_02.txt");
    assert_eq!(pmaze.pnt_2_pipe((4, 3)), '.');
}

#[test]
fn loopup_pnt_maze_03() {
    let pmaze = day_10::Maze::new("./data/example_03.txt");
    assert_eq!(pmaze.pnt_2_pipe((0, 2)), 'F');
}

#[test]
fn loopup_pnt_maze_input() {
    let pmaze = day_10::Maze::new("./data/input.txt");
    assert_eq!(pmaze.pnt_2_pipe((20, 80)), '|');
}
