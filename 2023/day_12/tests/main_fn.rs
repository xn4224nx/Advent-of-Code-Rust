#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_12;

#[test]
fn read_example_00() {
    assert_eq!(
        day_12::read_spring_condition_data("./data/example_00.txt"),
        vec![
            (String::from("#.#.###"), vec![1, 1, 3]),
            (String::from(".#...#....###."), vec![1, 1, 3]),
            (String::from(".#.###.#.######"), vec![1, 3, 1, 6]),
            (String::from("####.#...#..."), vec![4, 1, 1]),
            (String::from("#....######..#####."), vec![1, 6, 5]),
            (String::from(".###.##....#"), vec![3, 2, 1])
        ]
    )
}

#[test]
fn read_example_01() {
    assert_eq!(
        day_12::read_spring_condition_data("./data/example_01.txt"),
        vec![
            (String::from("???.###"), vec![1, 1, 3]),
            (String::from(".??..??...?##."), vec![1, 1, 3]),
            (String::from("?#?#?#?#?#?#?#?"), vec![1, 3, 1, 6]),
            (String::from("????.#...#..."), vec![4, 1, 1]),
            (String::from("????.######..#####."), vec![1, 6, 5]),
            (String::from("?###????????"), vec![3, 2, 1])
        ]
    )
}
