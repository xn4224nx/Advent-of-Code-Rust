#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_09;

#[test]
fn read_example_data() {
    assert_eq!(
        day_09::read_seq_data("./data/example_01.txt"),
        vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45]
        ]
    )
}
