#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_14;
use day_14::KeyGen;

#[test]
fn class_initi() {
    let test = KeyGen::new("abc");
    assert_eq!(test.salt, vec![97, 98, 99])
}

#[test]
fn hashing_with_salt() {
    let test = KeyGen::new("abc");

    assert!(String::from_utf8(test.stream(18))
        .unwrap()
        .contains("cc38887a5"));
    assert!(String::from_utf8(test.stream(39)).unwrap().contains("eee"));
    assert!(String::from_utf8(test.stream(816))
        .unwrap()
        .contains("eeeee"));
    assert!(String::from_utf8(test.stream(92)).unwrap().contains("999"));
    assert!(String::from_utf8(test.stream(200))
        .unwrap()
        .contains("99999"));
}

#[test]
fn extacting_multiples() {
    let test = KeyGen::new("abc");

    assert_eq!(
        test.find_multiples(vec![0, 0, 0, 2, 5, 45, 2, 2, 4]),
        (vec![0], Vec::new())
    );
    assert_eq!(
        test.find_multiples(vec![9, 4, 2, 7, 8, 8, 8, 8, 8, 8, 4, 67, 24, 65]),
        (vec![8], vec![8])
    );
    assert_eq!(
        test.find_multiples(vec![5, 32, 65, 34, 5, 68, 0, 9, 9, 8, 9, 9]),
        (Vec::new(), Vec::new())
    );
}

#[test]
fn key_generation_exp1() {
    let test = KeyGen::new("abc");
    let results = test.generate(64);

    assert_eq!(results[0], 39);
    assert_eq!(results[63], 22728);
    assert!(!results.contains(&18));
}

#[test]
fn key_generation_exp2() {
    let test = KeyGen::new("zpqevtbw");
    let results = test.generate(64);

    assert_eq!(results[63], 16106);
}
