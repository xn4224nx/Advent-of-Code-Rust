#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_10;

#[test]
fn reading_pipe_mazes() {
    let pmaze = day_10::Maze::new("./data/example_00.txt");
    assert_eq!(&pmaze.adj_squares((0, 0)), &vec![(1, 0), (0, 1)]);
    assert_eq!(&pmaze.adj_squares((4, 0)), &vec![(3, 0), (4, 1)]);
    assert_eq!(&pmaze.adj_squares((0, 4)), &vec![(1, 4), (0, 3)]);
    assert_eq!(&pmaze.adj_squares((4, 4)), &vec![(3, 4), (4, 3)]);
    assert_eq!(
        &pmaze.adj_squares((2, 3)),
        &vec![(3, 3), (1, 3), (2, 4), (2, 2)]
    );
}

#[test]
#[should_panic]
fn outside_maze_adj_x() {
    let pmaze = day_10::Maze::new("./data/example_00.txt");
    pmaze.adj_squares((10, 0));
}

#[test]
#[should_panic]
fn outside_maze_adj_y() {
    let pmaze = day_10::Maze::new("./data/example_00.txt");
    pmaze.adj_squares((0, 20));
}
