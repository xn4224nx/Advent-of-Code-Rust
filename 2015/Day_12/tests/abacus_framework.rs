#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_12;
use day_12::{read_account_data, sum_all_nums};

use serde_json::json;

#[test]
fn read_account_data_ex_01() {
    assert_eq!(
        read_account_data("./data/example_01.txt"),
        String::from("[1,2,3]")
    );
}

#[test]
fn read_account_data_ex_02() {
    assert_eq!(
        read_account_data("./data/example_02.txt"),
        String::from("{\"a\":2,\"b\":4}")
    );
}

#[test]
fn read_account_data_ex_03() {
    assert_eq!(
        read_account_data("./data/example_03.txt"),
        String::from("[[[3]]]")
    );
}

#[test]
fn read_account_data_ex_04() {
    assert_eq!(
        read_account_data("./data/example_04.txt"),
        String::from("{\"a\":{\"b\":4},\"c\":-1}")
    );
}

#[test]
fn read_account_data_ex_05() {
    assert_eq!(
        read_account_data("./data/example_05.txt"),
        String::from("{\"a\":[-1,1]}")
    );
}

#[test]
fn read_account_data_ex_06() {
    assert_eq!(
        read_account_data("./data/example_06.txt"),
        String::from("[-1,{\"a\":1}]")
    );
}

#[test]
fn read_account_data_ex_07() {
    assert_eq!(
        read_account_data("./data/example_07.txt"),
        String::from("[]")
    );
}

#[test]
fn read_account_data_ex_08() {
    assert_eq!(
        read_account_data("./data/example_08.txt"),
        String::from("{}")
    );
}

#[test]
fn read_account_data_ex_09() {
    assert_eq!(
        read_account_data("./data/example_09.txt"),
        String::from("[1,{\"c\":\"red\",\"b\":2},3]")
    );
}

#[test]
fn read_account_data_ex_10() {
    assert_eq!(
        read_account_data("./data/example_10.txt"),
        String::from("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5")
    );
}

#[test]
fn read_account_data_ex_11() {
    assert_eq!(
        read_account_data("./data/example_11.txt"),
        String::from("[1,\"red\",5]")
    );
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
