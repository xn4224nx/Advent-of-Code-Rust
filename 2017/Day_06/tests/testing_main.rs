#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_6;
use day_6::MemBank;

#[test]
fn read_data_exp01() {
    assert_eq!(
        MemBank::new("./data/example_01.txt").blocks,
        vec![0, 2, 7, 0]
    );
}

#[test]
fn read_data_exp02() {
    assert_eq!(
        MemBank::new("./data/input.txt").blocks,
        vec![11, 11, 13, 7, 0, 15, 5, 5, 4, 4, 1, 1, 7, 1, 15, 11]
    );
}

#[test]
fn reallocate_exp01() {
    let mut test_bank = MemBank::new("./data/example_01.txt");
    test_bank.blocks = vec![0, 2, 7, 0];
    test_bank.realocate();
    assert_eq!(test_bank.blocks, vec![2, 4, 1, 2])
}

#[test]
fn reallocate_exp02() {
    let mut test_bank = MemBank::new("./data/example_01.txt");
    test_bank.blocks = vec![2, 4, 1, 2];
    test_bank.realocate();
    assert_eq!(test_bank.blocks, vec![3, 1, 2, 3])
}

#[test]
fn reallocate_exp03() {
    let mut test_bank = MemBank::new("./data/example_01.txt");
    test_bank.blocks = vec![3, 1, 2, 3];
    test_bank.realocate();
    assert_eq!(test_bank.blocks, vec![0, 2, 3, 4]);
}

#[test]
fn reallocate_exp04() {
    let mut test_bank = MemBank::new("./data/example_01.txt");
    test_bank.blocks = vec![0, 2, 3, 4];
    test_bank.realocate();
    assert_eq!(test_bank.blocks, vec![1, 3, 4, 1]);
}

#[test]
fn reallocate_exp05() {
    let mut test_bank = MemBank::new("./data/example_01.txt");
    test_bank.blocks = vec![1, 3, 4, 1];
    test_bank.realocate();
    assert_eq!(test_bank.blocks, vec![2, 4, 1, 2]);
}

#[test]
fn idx_of_max_bank_exp01() {
    let mut test_bank = MemBank::new("./data/example_01.txt");
    test_bank.blocks = vec![0, 2, 7, 0];
    assert_eq!(test_bank.idx_of_max_bank(), 2)
}

#[test]
fn idx_of_max_bank_exp02() {
    let mut test_bank = MemBank::new("./data/example_01.txt");
    test_bank.blocks = vec![2, 4, 1, 2];
    assert_eq!(test_bank.idx_of_max_bank(), 1)
}

#[test]
fn idx_of_max_bank_exp03() {
    let mut test_bank = MemBank::new("./data/example_01.txt");
    test_bank.blocks = vec![3, 1, 2, 3];
    assert_eq!(test_bank.idx_of_max_bank(), 0)
}

#[test]
fn idx_of_max_bank_exp04() {
    let mut test_bank = MemBank::new("./data/example_01.txt");
    test_bank.blocks = vec![0, 2, 3, 4];
    assert_eq!(test_bank.idx_of_max_bank(), 3)
}

#[test]
fn idx_of_max_bank_exp05() {
    let mut test_bank = MemBank::new("./data/example_01.txt");
    test_bank.blocks = vec![1, 3, 4, 1];
    assert_eq!(test_bank.idx_of_max_bank(), 2)
}

#[test]
fn cycles_to_duplicate_exp01() {
    assert_eq!(
        MemBank::new("./data/example_01.txt").cycles_to_duplicate(),
        5
    );
}
