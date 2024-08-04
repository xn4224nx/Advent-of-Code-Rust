#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_04;

#[test]
fn valid_md5_hash_example_01_true() {
    assert_eq!(
        day_04::valid_md5_hash(&String::from("abcdef"), 609043, 5),
        true
    )
}

#[test]
fn valid_md5_hash_example_01_false() {
    assert_eq!(
        day_04::valid_md5_hash(&String::from("abcdef"), 609043, 6),
        false
    )
}

#[test]
fn valid_md5_hash_example_02_true() {
    assert_eq!(
        day_04::valid_md5_hash(&String::from("pqrstuv"), 1048970, 5),
        true
    )
}

#[test]
fn valid_md5_hash_example_02_false() {
    assert_eq!(
        day_04::valid_md5_hash(&String::from("pqrstuv"), 1048970, 6),
        false
    )
}
