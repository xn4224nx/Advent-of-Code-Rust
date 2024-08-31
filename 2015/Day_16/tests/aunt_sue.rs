#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_16;
use day_16::{could_aunt_match, find_true_aunt_index, read_aunt_data};

use std::collections::HashMap;

#[test]
fn read_cookie_data_ex_01() {
    assert_eq!(
        read_aunt_data("./data/p1_aunt.txt"),
        vec![HashMap::from([
            (String::from("children"), 3),
            (String::from("cats"), 7),
            (String::from("samoyeds"), 2),
            (String::from("pomeranians"), 3),
            (String::from("akitas"), 0),
            (String::from("vizslas"), 0),
            (String::from("goldfish"), 5),
            (String::from("trees"), 3),
            (String::from("cars"), 2),
            (String::from("perfumes"), 1)
        ])]
    );
}

#[test]
fn read_cookie_data_ex_02() {
    assert_eq!(
        read_aunt_data("./data/p1_example_aunts.txt"),
        vec![
            HashMap::from([
                (String::from("children"), 1),
                (String::from("vizslas"), 7),
                (String::from("cars"), 8),
            ]),
            HashMap::from([
                (String::from("children"), 5),
                (String::from("akitas"), 10),
                (String::from("perfumes"), 10),
            ]),
            HashMap::from([
                (String::from("pomeranians"), 4),
                (String::from("vizslas"), 1),
                (String::from("cars"), 5),
            ]),
            HashMap::from([
                (String::from("children"), 8),
                (String::from("goldfish"), 5),
                (String::from("perfumes"), 3)
            ]),
            HashMap::from([
                (String::from("akitas"), 0),
                (String::from("vizslas"), 0),
                (String::from("perfumes"), 1)
            ]),
            HashMap::from([
                (String::from("akitas"), 1),
                (String::from("vizslas"), 0),
                (String::from("perfumes"), 2)
            ]),
            HashMap::from([
                (String::from("goldfish"), 10),
                (String::from("cars"), 4),
                (String::from("perfumes"), 8)
            ]),
            HashMap::from([
                (String::from("children"), 2),
                (String::from("cats"), 1),
                (String::from("perfumes"), 7)
            ]),
            HashMap::from([
                (String::from("pomeranians"), 3),
                (String::from("goldfish"), 10),
                (String::from("trees"), 10),
            ]),
            HashMap::from([
                (String::from("pomeranians"), 4),
                (String::from("akitas"), 7),
                (String::from("trees"), 8),
            ]),
        ]
    );
}

#[test]
fn could_aunt_match_01() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    let pos_aunt = &read_aunt_data("./data/p1_example_aunts.txt")[0];
    assert_eq!(could_aunt_match(true_aunt, pos_aunt, false), false);
}

#[test]
fn could_aunt_match_02() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    let pos_aunt = &read_aunt_data("./data/p1_example_aunts.txt")[1];
    assert_eq!(could_aunt_match(true_aunt, pos_aunt, false), false);
}

#[test]
fn could_aunt_match_03() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    let pos_aunt = &read_aunt_data("./data/p1_example_aunts.txt")[2];
    assert_eq!(could_aunt_match(true_aunt, pos_aunt, false), false);
}

#[test]
fn could_aunt_match_04() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    let pos_aunt = &read_aunt_data("./data/p1_example_aunts.txt")[3];
    assert_eq!(could_aunt_match(true_aunt, pos_aunt, false), false);
}

#[test]
fn could_aunt_match_05() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    let pos_aunt = &read_aunt_data("./data/p1_example_aunts.txt")[4];
    assert_eq!(could_aunt_match(true_aunt, pos_aunt, false), true);
}

#[test]
fn could_aunt_match_06() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    let pos_aunt = &read_aunt_data("./data/p1_example_aunts.txt")[5];
    assert_eq!(could_aunt_match(true_aunt, pos_aunt, false), false);
}

#[test]
fn could_aunt_match_07() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    let pos_aunt = &read_aunt_data("./data/p1_example_aunts.txt")[6];
    assert_eq!(could_aunt_match(true_aunt, pos_aunt, false), false);
}

#[test]
fn could_aunt_match_08() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    let pos_aunt = &read_aunt_data("./data/p1_example_aunts.txt")[7];
    assert_eq!(could_aunt_match(true_aunt, pos_aunt, false), false);
}

#[test]
fn could_aunt_match_09() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    let pos_aunt = &read_aunt_data("./data/p1_example_aunts.txt")[8];
    assert_eq!(could_aunt_match(true_aunt, pos_aunt, false), false);
}

#[test]
fn could_aunt_match_10() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    let pos_aunt = &read_aunt_data("./data/p1_example_aunts.txt")[9];
    assert_eq!(could_aunt_match(true_aunt, pos_aunt, false), false);
}

#[test]
fn find_true_aunt_index_exp1() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    let all_aunts = &read_aunt_data("./data/p1_example_aunts.txt");
    assert_eq!(find_true_aunt_index(true_aunt, all_aunts, false), 5);
}

#[test]
fn could_aunt_match_retro_ex_1() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    assert_eq!(
        could_aunt_match(
            true_aunt,
            &HashMap::from([
                (String::from("akitas"), 0),
                (String::from("trees"), 3),
                (String::from("pomeranians"), 3),
            ]),
            true
        ),
        false
    );
}

#[test]
fn could_aunt_match_retro_ex_2() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    assert_eq!(
        could_aunt_match(
            true_aunt,
            &HashMap::from([
                (String::from("cats"), 7),
                (String::from("akitas"), 0),
                (String::from("trees"), 3),
                (String::from("pomeranians"), 3),
                (String::from("goldfish"), 5),
            ]),
            true
        ),
        false
    );
}

#[test]
fn could_aunt_match_retro_ex_3() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    assert_eq!(
        could_aunt_match(
            true_aunt,
            &HashMap::from([
                (String::from("cats"), 5),
                (String::from("akitas"), 0),
                (String::from("trees"), 1),
                (String::from("pomeranians"), 5),
                (String::from("goldfish"), 7),
            ]),
            true
        ),
        false
    );
}

#[test]
fn could_aunt_match_retro_ex_4() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    assert_eq!(
        could_aunt_match(
            true_aunt,
            &HashMap::from([
                (String::from("cats"), 9),
                (String::from("akitas"), 0),
                (String::from("trees"), 9),
                (String::from("pomeranians"), 2),
                (String::from("goldfish"), 4),
            ]),
            true
        ),
        true
    );
}

#[test]
fn could_aunt_match_retro_ex_5() {
    let true_aunt = &read_aunt_data("./data/p1_aunt.txt")[0];
    assert_eq!(
        could_aunt_match(
            true_aunt,
            &HashMap::from([(String::from("trees"), 4),]),
            true
        ),
        true
    );
}
