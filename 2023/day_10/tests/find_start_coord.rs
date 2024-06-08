#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_10;

#[test]
fn find_start_in_example_00() {
    let maze = day_10::read_pipe_maze("./data/example_00.txt");
    assert_eq!(day_10::find_start_coord(&maze), (3, 1));
}

#[test]
fn find_start_in_example_01() {
    let maze = day_10::read_pipe_maze("./data/example_01.txt");
    assert_eq!(day_10::find_start_coord(&maze), (0, 1));
}

#[test]
fn find_start_in_example_02() {
    let maze = day_10::read_pipe_maze("./data/example_02.txt");
    assert_eq!(day_10::find_start_coord(&maze), (2, 3));
}

#[test]
fn find_start_in_example_03() {
    let maze = day_10::read_pipe_maze("./data/example_03.txt");
    assert_eq!(day_10::find_start_coord(&maze), (2, 2));
}

#[test]
#[should_panic]
fn find_start_in_empty_maze() {
    day_10::find_start_coord(&vec![vec!['.']]);
}
