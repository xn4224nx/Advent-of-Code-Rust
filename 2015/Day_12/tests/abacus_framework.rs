#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_12;
use day_12::{read_account_data, sum_all_nums};

use serde_json::json;

#[test]
fn read_account_data_ex_01() {
    assert_eq!(read_account_data("./data/example_01.txt"), json!([1, 2, 3]));
}

#[test]
fn read_account_data_ex_02() {
    assert_eq!(
        read_account_data("./data/example_02.txt"),
        json!({"a":2,"b":4})
    );
}

#[test]
fn read_account_data_ex_03() {
    assert_eq!(read_account_data("./data/example_03.txt"), json!([[[3]]]));
}

#[test]
fn read_account_data_ex_04() {
    assert_eq!(
        read_account_data("./data/example_04.txt"),
        json!({"a":{"b":4},"c":-1})
    );
}

#[test]
fn read_account_data_ex_05() {
    assert_eq!(
        read_account_data("./data/example_05.txt"),
        json!({"a":[-1,1]})
    );
}

#[test]
fn read_account_data_ex_06() {
    assert_eq!(
        read_account_data("./data/example_06.txt"),
        json!([-1,{"a":1}])
    );
}

#[test]
fn read_account_data_ex_07() {
    assert_eq!(read_account_data("./data/example_07.txt"), json!([]));
}

#[test]
fn read_account_data_ex_08() {
    assert_eq!(read_account_data("./data/example_08.txt"), json!({}));
}

#[test]
fn sum_all_nums_ex_01() {
    assert_eq!(sum_all_nums(&read_account_data("./data/example_01.txt")), 6)
}

#[test]
fn sum_all_nums_ex_02() {
    assert_eq!(sum_all_nums(&read_account_data("./data/example_02.txt")), 6)
}

#[test]
fn sum_all_nums_ex_03() {
    assert_eq!(sum_all_nums(&read_account_data("./data/example_03.txt")), 3)
}

#[test]
fn sum_all_nums_ex_04() {
    assert_eq!(sum_all_nums(&read_account_data("./data/example_04.txt")), 3)
}

#[test]
fn sum_all_nums_ex_05() {
    assert_eq!(sum_all_nums(&read_account_data("./data/example_05.txt")), 0)
}

#[test]
fn sum_all_nums_ex_06() {
    assert_eq!(sum_all_nums(&read_account_data("./data/example_06.txt")), 0)
}

#[test]
fn sum_all_nums_ex_07() {
    assert_eq!(sum_all_nums(&read_account_data("./data/example_07.txt")), 0)
}

#[test]
fn sum_all_nums_ex_08() {
    assert_eq!(sum_all_nums(&read_account_data("./data/example_08.txt")), 0)
}
