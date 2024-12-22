#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_14;
use day_14::KeyGen;

#[test]
fn class_initi() {
    let test = KeyGen::new("abc");
    assert_eq!(test.salt, String::from("abc"))
}

#[test]
fn hashing_with_salt() {
    let test = KeyGen::new("abc");

    assert!(String::from_iter(test.stream(18)).contains("cc38887a5"));
    assert!(String::from_iter(test.stream(39)).contains("eee"));
    assert!(String::from_iter(test.stream(816)).contains("eeeee"));
    assert!(String::from_iter(test.stream(92)).contains("999"));
    assert!(String::from_iter(test.stream(200)).contains("99999"));
}

#[test]
fn extacting_multiples() {
    let test = KeyGen::new("abc");

    assert_eq!(
        test.find_multiples(vec!['a', 'a', 'a', 'f', 'g']),
        (vec!['a'], Vec::new())
    );
    assert_eq!(
        test.find_multiples(vec![
            '4', '3', 'f', '8', '8', '8', '8', '8', '8', '4', '7', '2', '4', '6', '5'
        ]),
        (vec!['8'], vec!['8'])
    );
    assert_eq!(
        test.find_multiples(vec![
            '5', '3', '2', '6', '5', '3', '4', '5', '6', '8', '0', '9', '9', '8', '9', '9'
        ]),
        (Vec::new(), Vec::new())
    );
}

#[test]
fn key_generation_exp1() {
    let test = KeyGen::new("abc");
    let results = test.generate(64);

    println!("{:?}", results);

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
