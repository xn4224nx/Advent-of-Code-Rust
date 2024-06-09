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
fn reading_pipe_mazes() {
    let all_mazes: Vec<Vec<Vec<char>>> = read_all_mazes().iter().map(|x| x.pipes.clone()).collect();
    assert_eq!(
        all_mazes[0],
        vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'S', '-', '7', '.'],
            vec!['.', '|', '.', '|', '.'],
            vec!['.', 'L', '-', 'J', '.'],
            vec!['.', '.', '.', '.', '.']
        ]
    );
    assert_eq!(
        all_mazes[1],
        vec![
            vec!['-', 'L', '|', 'F', '7'],
            vec!['7', 'S', '-', '7', '|'],
            vec!['L', '|', '7', '|', '|'],
            vec!['-', 'L', '-', 'J', '|'],
            vec!['L', '|', '-', 'J', 'F']
        ]
    );
    assert_eq!(
        all_mazes[2],
        vec![
            vec!['.', '.', 'F', '7', '.'],
            vec!['.', 'F', 'J', '|', '.'],
            vec!['S', 'J', '.', 'L', '7'],
            vec!['|', 'F', '-', '-', 'J'],
            vec!['L', 'J', '.', '.', '.']
        ]
    );
    assert_eq!(
        all_mazes[3],
        vec![
            vec!['7', '-', 'F', '7', '-'],
            vec!['.', 'F', 'J', '|', '7'],
            vec!['S', 'J', 'L', 'L', '7'],
            vec!['|', 'F', '-', '-', 'J'],
            vec!['L', 'J', '.', 'L', 'J']
        ]
    );
}

#[test]
fn start_points() {
    let all_pts: Vec<(usize, usize)> = read_all_mazes().iter().map(|x| x.start_pnt).collect();
    assert_eq!(all_pts[0], (1, 1));
    assert_eq!(all_pts[1], (1, 1));
    assert_eq!(all_pts[2], (2, 0));
    assert_eq!(all_pts[3], (2, 0));
}

#[test]
fn maze_sizes() {
    let all_sizes: Vec<(usize, usize)> = read_all_mazes().iter().map(|x| x.maze_size).collect();
    assert_eq!(all_sizes[0], (5, 5));
    assert_eq!(all_sizes[1], (5, 5));
    assert_eq!(all_sizes[2], (5, 5));
    assert_eq!(all_sizes[3], (5, 5));
}

#[test]
#[should_panic]
fn no_start_point() {
    day_10::Maze::new("./data/no_start_point.txt");
}

#[test]
#[should_panic]
fn unequal_rows() {
    day_10::Maze::new("./data/unequal_rows.txt");
}
