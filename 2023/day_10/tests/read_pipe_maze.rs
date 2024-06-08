#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_10;

#[test]
fn read_example_00() {
    assert_eq!(
        day_10::read_pipe_maze("./data/example_00.txt"),
        vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'S', '-', '7', '.'],
            vec!['.', '|', '.', '|', '.'],
            vec!['.', 'L', '-', 'J', '.'],
            vec!['.', '.', '.', '.', '.']
        ]
    );
}

#[test]
fn read_example_01() {
    assert_eq!(
        day_10::read_pipe_maze("./data/example_01.txt"),
        vec![
            vec!['-', 'L', '|', 'F', '7'],
            vec!['7', 'S', '-', '7', '|'],
            vec!['L', '|', '7', '|', '|'],
            vec!['-', 'L', '-', 'J', '|'],
            vec!['L', '|', '-', 'J', 'F']
        ]
    );
}

#[test]
fn read_example_02() {
    assert_eq!(
        day_10::read_pipe_maze("./data/example_02.txt"),
        vec![
            vec!['.', '.', 'F', '7', '.'],
            vec!['.', 'F', 'J', '|', '.'],
            vec!['S', 'J', '.', 'L', '7'],
            vec!['|', 'F', '-', '-', 'J'],
            vec!['L', 'J', '.', '.', '.']
        ]
    );
}

#[test]
fn read_example_03() {
    assert_eq!(
        day_10::read_pipe_maze("./data/example_03.txt"),
        vec![
            vec!['7', '-', 'F', '7', '-'],
            vec!['.', 'F', 'J', '|', '7'],
            vec!['S', 'J', 'L', 'L', '7'],
            vec!['|', 'F', '-', '-', 'J'],
            vec!['L', 'J', '.', 'L', 'J']
        ]
    );
}
