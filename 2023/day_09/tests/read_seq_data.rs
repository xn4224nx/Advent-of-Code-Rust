#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_09;

#[test]
fn read_example_data() {
    assert_eq!(
        day_09::read_seq_data("./data/example_01.txt"),
        vec![
            vec![0, 3, 6, 9, 1, 2, 1, 5],
            vec![1, 3, 6, 1, 0, 1, 5, 2, 1],
            vec![1, 0, 1, 3, 1, 6, 2, 1, 3, 0, 4, 5]
        ]
    )
}
