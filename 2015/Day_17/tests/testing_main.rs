#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_17;
use day_17::{count_cont_combs, does_comb_fit, read_container_sizes};

#[test]
fn read_container_sizes_ex1() {
    assert_eq!(
        read_container_sizes("./data/example_01.txt"),
        vec![20, 15, 10, 5, 5]
    );
}

#[test]
fn does_comb_fit_ex1() {
    assert_eq!(does_comb_fit(&vec![15, 10].iter().collect(), 25), true);
}

#[test]
fn does_comb_fit_ex2() {
    assert_eq!(does_comb_fit(&vec![20, 5].iter().collect(), 25), true);
}

#[test]
fn does_comb_fit_ex3() {
    assert_eq!(does_comb_fit(&vec![15, 5, 5].iter().collect(), 25), true);
}

#[test]
fn does_comb_fit_ex4() {
    assert_eq!(does_comb_fit(&vec![15, 10, 5].iter().collect(), 25), false);
}

#[test]
fn does_comb_fit_ex5() {
    assert_eq!(does_comb_fit(&vec![20].iter().collect(), 25), false);
}

#[test]
fn does_comb_fit_ex6() {
    assert_eq!(does_comb_fit(&vec![20, 5, 5].iter().collect(), 25), false);
}

#[test]
fn count_cont_combs_ex1() {
    assert_eq!(count_cont_combs(&vec![20, 15, 10, 5, 5], 25, false), 4);
}

#[test]
fn count_cont_combs_ex2() {
    assert_eq!(count_cont_combs(&vec![20, 15, 10, 5, 5], 25, true), 3);
}
