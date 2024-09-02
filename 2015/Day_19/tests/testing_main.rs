#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_19;
use day_19::{cnt_distinct_chems, read_molc_replacements};

#[test]
fn read_molc_replacements_ex01() {
    let (reps, chem) = read_molc_replacements("./data/example_01.txt");

    assert_eq!(chem, String::from("HOH"));
    assert_eq!(
        reps,
        vec![
            (String::from("H"), String::from("HO")),
            (String::from("H"), String::from("OH")),
            (String::from("O"), String::from("HH"))
        ]
    );
}

#[test]
fn read_molc_replacements_ex02() {
    let (reps, chem) = read_molc_replacements("./data/example_02.txt");

    assert_eq!(chem, String::from("HOHOHO"));
    assert_eq!(
        reps,
        vec![
            (String::from("e"), String::from("H")),
            (String::from("e"), String::from("O")),
            (String::from("H"), String::from("HO")),
            (String::from("H"), String::from("OH")),
            (String::from("O"), String::from("HH"))
        ]
    );
}

#[test]
fn read_molc_replacements_ex03() {
    let (reps, chem) = read_molc_replacements("./data/example_03.txt");

    assert_eq!(chem, String::from("HOH"));
    assert_eq!(
        reps,
        vec![
            (String::from("e"), String::from("H")),
            (String::from("e"), String::from("O")),
            (String::from("H"), String::from("HO")),
            (String::from("H"), String::from("OH")),
            (String::from("O"), String::from("HH"))
        ]
    );
}
