#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_11;
use day_11::{Dir, HexGrid};

#[test]
fn parse_data_exp01() {
    assert_eq!(
        HexGrid::new("./data/example_01.txt").path,
        vec![Dir::NorthEast, Dir::NorthEast, Dir::NorthEast,]
    );
}

#[test]
fn parse_data_exp02() {
    assert_eq!(
        HexGrid::new("./data/example_02.txt").path,
        vec![
            Dir::NorthEast,
            Dir::NorthEast,
            Dir::SouthWest,
            Dir::SouthWest,
        ]
    );
}

#[test]
fn parse_data_exp03() {
    assert_eq!(
        HexGrid::new("./data/example_03.txt").path,
        vec![Dir::NorthEast, Dir::NorthEast, Dir::South, Dir::South,]
    );
}

#[test]
fn parse_data_exp04() {
    assert_eq!(
        HexGrid::new("./data/example_04.txt").path,
        vec![
            Dir::SouthEast,
            Dir::SouthWest,
            Dir::SouthEast,
            Dir::SouthWest,
            Dir::SouthWest,
        ]
    );
}

#[test]
fn parse_data_exp05() {
    assert_eq!(
        HexGrid::new("./data/example_05.txt").path,
        vec![Dir::SouthEast, Dir::SouthEast,]
    );
}

#[test]
fn parse_data_exp06() {
    assert_eq!(
        HexGrid::new("./data/example_06.txt").path,
        vec![Dir::South, Dir::South, Dir::SouthWest,]
    );
}

#[test]
fn hex_distance_exp01() {
    assert_eq!(HexGrid::new("./data/example_01.txt").distance(), 3);
}

#[test]
fn hex_distance_exp02() {
    assert_eq!(HexGrid::new("./data/example_02.txt").distance(), 0);
}

#[test]
fn hex_distance_exp03() {
    assert_eq!(HexGrid::new("./data/example_03.txt").distance(), 2);
}

#[test]
fn hex_distance_exp04() {
    assert_eq!(HexGrid::new("./data/example_04.txt").distance(), 3);
}

#[test]
fn hex_distance_exp05() {
    assert_eq!(HexGrid::new("./data/example_05.txt").distance(), 2);
}

#[test]
fn hex_distance_exp06() {
    assert_eq!(HexGrid::new("./data/example_06.txt").distance(), 3);
}
