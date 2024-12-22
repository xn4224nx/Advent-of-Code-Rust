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
    let results = test.generate(64, false);

    assert_eq!(results[0], 39);
    assert_eq!(results[63], 22728);
    assert!(!results.contains(&18));
}

#[test]
fn key_generation_exp2() {
    let test = KeyGen::new("zpqevtbw");
    let results = test.generate(64, false);
    assert_eq!(results[63], 16106);
}

#[test]
fn key_generation_exp1_stretch() {
    let test = KeyGen::new("abc");
    let results = test.generate(64, true);
    assert_eq!(results[63], 22551);
}

#[test]
fn key_generation_exp2_stretch() {
    let test = KeyGen::new("zpqevtbw");
    let results = test.generate(64, true);
    assert_eq!(results[63], 22423);
}

#[test]
fn key_stretching() {
    let test = KeyGen::new("abc");
    /*assert_eq!(
        test.stretch(&vec!['a', 'b', 'c', '0'], 1),
        vec![
            '5', '7', '7', '5', '7', '1', 'b', 'e', '4', 'd', 'e', '9', 'd', 'c', 'c', 'e', '8',
            '5', 'a', '0', '4', '1', 'b', 'a', '0', '4', '1', '0', 'f', '2', '9', 'f'
        ]
    );*/
    assert_eq!(
        test.stretch(&vec!['a', 'b', 'c', '0'], 2),
        vec![
            'e', 'e', 'c', '8', '0', 'a', '0', 'c', '9', '2', 'd', 'c', '8', 'a', '0', '7', '7',
            '7', 'c', '6', '1', '9', 'd', '9', 'b', 'b', '5', '1', 'e', '9', '1', '0'
        ]
    );
    assert_eq!(
        test.stretch(&vec!['a', 'b', 'c', '0'], 3),
        vec![
            '1', '6', '0', '6', '2', 'c', 'e', '7', '6', '8', '7', '8', '7', '3', '8', '4', 'c',
            '8', '1', 'f', 'e', '1', '7', 'a', '7', 'a', '6', '0', 'c', '7', 'e', '3'
        ]
    );
    assert_eq!(
        test.stretch(&vec!['a', 'b', 'c', '0'], 2017),
        vec![
            'a', '1', '0', '7', 'f', 'f', '6', '3', '4', '8', '5', '6', 'b', 'b', '3', '0', '0',
            '1', '3', '8', 'c', 'a', 'c', '6', '5', '6', '8', 'c', '0', 'f', '2', '4'
        ]
    );
}
