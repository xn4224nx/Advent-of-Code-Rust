#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_2;
use day_2::SpreadSheet;

#[test]
fn reading_data_exp1() {
    assert_eq!(
        SpreadSheet::new("./data/example_01.txt").data,
        vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]]
    );
}

#[test]
fn reading_data_exp2() {
    assert_eq!(
        SpreadSheet::new("./data/example_02.txt").data,
        vec![vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]]
    );
}

#[test]
fn row_range_01() {
    let test = SpreadSheet::new("./data/example_01.txt");
    assert_eq!(test.row_range(0), 8);
}

#[test]
fn row_range_02() {
    let test = SpreadSheet::new("./data/example_01.txt");
    assert_eq!(test.row_range(1), 4);
}

#[test]
fn row_range_03() {
    let test = SpreadSheet::new("./data/example_01.txt");
    assert_eq!(test.row_range(2), 6);
}

#[test]
fn row_range_04() {
    let test = SpreadSheet::new("./data/example_02.txt");
    assert_eq!(test.row_range(0), 7);
}

#[test]
fn row_range_05() {
    let test = SpreadSheet::new("./data/example_02.txt");
    assert_eq!(test.row_range(1), 6);
}

#[test]
fn row_range_06() {
    let test = SpreadSheet::new("./data/example_02.txt");
    assert_eq!(test.row_range(2), 5);
}
