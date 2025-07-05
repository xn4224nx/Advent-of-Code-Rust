#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_1;
use day_1::{IMS, Multiples};

#[test]
fn new_ims_exp0() {
    let test = IMS::new("./data/example_0.txt");
    assert_eq!(
        test.boxes,
        vec![
            vec!['a', 'b', 'c', 'd', 'e', 'f'],
            vec!['b', 'a', 'b', 'a', 'b', 'c'],
            vec!['a', 'b', 'b', 'c', 'd', 'e'],
            vec!['a', 'b', 'c', 'c', 'c', 'd'],
            vec!['a', 'a', 'b', 'c', 'd', 'd'],
            vec!['a', 'b', 'c', 'd', 'e', 'e'],
            vec!['a', 'b', 'a', 'b', 'a', 'b'],
        ]
    );
}

#[test]
fn new_ims_exp1() {
    let test = IMS::new("./data/example_1.txt");
    assert_eq!(
        test.boxes,
        vec![
            vec!['a', 'b', 'c', 'd', 'e'],
            vec!['f', 'g', 'h', 'i', 'j'],
            vec!['k', 'l', 'm', 'n', 'o'],
            vec!['p', 'q', 'r', 's', 't'],
            vec!['f', 'g', 'u', 'i', 'j'],
            vec!['a', 'x', 'c', 'y', 'e'],
            vec!['w', 'v', 'x', 'y', 'z'],
        ]
    );
}

#[test]
fn box_check_exp0() {
    let test = IMS::new("./data/example_0.txt");
    assert_eq!(test.box_check(0), Multiples::Nothing);
}

#[test]
fn box_check_exp1() {
    let test = IMS::new("./data/example_0.txt");
    assert_eq!(test.box_check(1), Multiples::Both);
}

#[test]
fn box_check_exp2() {
    let test = IMS::new("./data/example_0.txt");
    assert_eq!(test.box_check(2), Multiples::Duplicate);
}

#[test]
fn box_check_exp3() {
    let test = IMS::new("./data/example_0.txt");
    assert_eq!(test.box_check(3), Multiples::Triplicate);
}

#[test]
fn box_check_exp4() {
    let test = IMS::new("./data/example_0.txt");
    assert_eq!(test.box_check(4), Multiples::Duplicate);
}

#[test]
fn box_check_exp5() {
    let test = IMS::new("./data/example_0.txt");
    assert_eq!(test.box_check(5), Multiples::Duplicate);
}

#[test]
fn box_check_exp6() {
    let test = IMS::new("./data/example_0.txt");
    assert_eq!(test.box_check(6), Multiples::Triplicate);
}

#[test]
fn checksum_exp0() {
    assert_eq!(IMS::new("./data/example_0.txt").checksum(), 12);
}

#[test]
fn key_box_contents_exp1() {
    assert_eq!(
        IMS::new("./data/example_1.txt").key_box_contents(),
        String::from("fgij")
    );
}
