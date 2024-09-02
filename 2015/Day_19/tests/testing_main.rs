#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_19;
use day_19::{cnt_distinct_chems, count_chem_build, read_molc_replacements};

#[test]
fn read_molc_replacements_ex01() {
    let (reps, chem) = read_molc_replacements("./data/example_01.txt");

    assert_eq!(chem, "HOH".as_bytes().to_vec());
    assert_eq!(
        reps,
        vec![
            ("H".as_bytes().to_vec(), "HO".as_bytes().to_vec()),
            ("H".as_bytes().to_vec(), "OH".as_bytes().to_vec()),
            ("O".as_bytes().to_vec(), "HH".as_bytes().to_vec())
        ]
    );
}

#[test]
fn read_molc_replacements_ex02() {
    let (reps, chem) = read_molc_replacements("./data/example_02.txt");

    assert_eq!(chem, "HOHOHO".as_bytes().to_vec());
    assert_eq!(
        reps,
        vec![
            ("e".as_bytes().to_vec(), "H".as_bytes().to_vec()),
            ("e".as_bytes().to_vec(), "O".as_bytes().to_vec()),
            ("H".as_bytes().to_vec(), "HO".as_bytes().to_vec()),
            ("H".as_bytes().to_vec(), "OH".as_bytes().to_vec()),
            ("O".as_bytes().to_vec(), "HH".as_bytes().to_vec())
        ]
    );
}

#[test]
fn read_molc_replacements_ex03() {
    let (reps, chem) = read_molc_replacements("./data/example_03.txt");

    assert_eq!(chem, "HOH".as_bytes().to_vec());
    assert_eq!(
        reps,
        vec![
            ("e".as_bytes().to_vec(), "H".as_bytes().to_vec()),
            ("e".as_bytes().to_vec(), "O".as_bytes().to_vec()),
            ("H".as_bytes().to_vec(), "HO".as_bytes().to_vec()),
            ("H".as_bytes().to_vec(), "OH".as_bytes().to_vec()),
            ("O".as_bytes().to_vec(), "HH".as_bytes().to_vec())
        ]
    );
}

#[test]
fn cnt_distinct_chems_ex1() {
    let (reps, chem) = read_molc_replacements("./data/example_01.txt");
    assert_eq!(cnt_distinct_chems(&reps, &chem), 4);
}

#[test]
fn cnt_distinct_chems_ex2() {
    let (reps, _) = read_molc_replacements("./data/example_01.txt");
    assert_eq!(cnt_distinct_chems(&reps, &"HOHOHO".as_bytes().to_vec()), 7);
}

#[test]
fn count_chem_build_ex1() {
    let (reps, chem) = read_molc_replacements("./data/example_02.txt");
    assert_eq!(count_chem_build(&reps, &chem), 6);
}

#[test]
fn count_chem_build_ex2() {
    let (reps, chem) = read_molc_replacements("./data/example_03.txt");
    assert_eq!(count_chem_build(&reps, &chem), 3);
}
