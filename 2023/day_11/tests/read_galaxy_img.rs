#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_11;

#[test]
fn read_example_01() {
    assert_eq!(
        vec![
            (3, 0),
            (7, 1),
            (0, 2),
            (6, 4),
            (1, 5),
            (9, 6),
            (7, 8),
            (0, 9),
            (4, 9),
        ],
        day_11::read_galaxy_img("./data/example_01.txt")
    )
}

#[test]
fn read_example_02() {
    assert_eq!(
        vec![
            (4, 0),
            (9, 1),
            (0, 2),
            (8, 5),
            (1, 6),
            (12, 7),
            (9, 10),
            (0, 11),
            (5, 11),
        ],
        day_11::read_galaxy_img("./data/example_02.txt")
    )
}
