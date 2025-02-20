#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_4;
use day_4::{count_valid_passwords, parse_passwords, Password};
use std::collections::HashMap;

#[test]
fn read_passwords_exp01() {
    assert_eq!(
        parse_passwords("./data/example_01/txt"),
        vec![
            Password {
                word_counts: vec![
                    HashMap::from([('a', 2)]),
                    HashMap::from([('b', 2)]),
                    HashMap::from([('c', 2)]),
                    HashMap::from([('d', 2)]),
                    HashMap::from([('e', 2)]),
                ]
            },
            Password {
                word_counts: vec![
                    HashMap::from([('a', 2)]),
                    HashMap::from([('b', 2)]),
                    HashMap::from([('c', 2)]),
                    HashMap::from([('d', 2)]),
                    HashMap::from([('a', 2)]),
                ]
            },
            Password {
                word_counts: vec![
                    HashMap::from([('a', 2)]),
                    HashMap::from([('b', 2)]),
                    HashMap::from([('c', 2)]),
                    HashMap::from([('d', 2)]),
                    HashMap::from([('a', 3)]),
                ]
            },
        ]
    )
}

#[test]
fn read_passwords_exp02() {
    assert_eq!(
        parse_passwords("./data/example_02/txt"),
        vec![
            Password {
                word_counts: vec![
                    HashMap::from([('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1)]),
                    HashMap::from([('f', 1), ('g', 1), ('h', 1), ('i', 1), ('j', 1)]),
                ]
            },
            Password {
                word_counts: vec![
                    HashMap::from([('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1)]),
                    HashMap::from([('x', 1), ('y', 1), ('z', 1)]),
                    HashMap::from([('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1)]),
                ]
            },
            Password {
                word_counts: vec![
                    HashMap::from([('a', 1)]),
                    HashMap::from([('a', 1), ('b', 1)]),
                    HashMap::from([('a', 1), ('b', 1), ('c', 1)]),
                    HashMap::from([('a', 1), ('b', 1), ('d', 1)]),
                    HashMap::from([('a', 1), ('b', 1), ('f', 1)]),
                    HashMap::from([('a', 1), ('b', 1), ('j', 1)]),
                ]
            },
            Password {
                word_counts: vec![
                    HashMap::from([('i', 4)]),
                    HashMap::from([('i', 3), ('o', 1)]),
                    HashMap::from([('i', 2), ('o', 2)]),
                    HashMap::from([('i', 1), ('o', 3)]),
                    HashMap::from([('o', 4)]),
                ]
            },
            Password {
                word_counts: vec![
                    HashMap::from([('i', 3), ('o', 1)]),
                    HashMap::from([('i', 3), ('o', 1)]),
                    HashMap::from([('i', 3), ('o', 1)]),
                    HashMap::from([('i', 3), ('o', 1)]),
                ]
            },
        ]
    )
}

#[test]
fn parse_password_exp1() {
    assert_eq!(
        Password::new("aa bb cc dd ee").word_counts,
        vec![
            HashMap::from([('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1)]),
            HashMap::from([('f', 1), ('g', 1), ('h', 1), ('i', 1), ('j', 1)]),
        ]
    );
}

#[test]
fn parse_password_exp2() {
    assert_eq!(
        Password::new("aa bb cc dd aa").word_counts,
        vec![
            HashMap::from([('a', 2)]),
            HashMap::from([('b', 2)]),
            HashMap::from([('c', 2)]),
            HashMap::from([('d', 2)]),
            HashMap::from([('a', 2)]),
        ]
    );
}

#[test]
fn parse_password_exp3() {
    assert_eq!(
        Password::new("aa bb cc dd aaa").word_counts,
        vec![
            HashMap::from([('a', 2)]),
            HashMap::from([('b', 2)]),
            HashMap::from([('c', 2)]),
            HashMap::from([('d', 2)]),
            HashMap::from([('a', 3)]),
        ]
    );
}

#[test]
fn parse_password_exp4() {
    assert_eq!(
        Password::new("abcde fghij").word_counts,
        vec![
            HashMap::from([('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1)]),
            HashMap::from([('f', 1), ('g', 1), ('h', 1), ('i', 1), ('j', 1)]),
        ]
    );
}

#[test]
fn parse_password_exp5() {
    assert_eq!(
        Password::new("abcde xyz ecdab").word_counts,
        vec![
            HashMap::from([('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1)]),
            HashMap::from([('x', 1), ('y', 1), ('z', 1)]),
            HashMap::from([('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1)]),
        ]
    );
}

#[test]
fn parse_password_exp6() {
    assert_eq!(
        Password::new("a ab abc abd abf abj").word_counts,
        vec![
            HashMap::from([('a', 1)]),
            HashMap::from([('a', 1), ('b', 1)]),
            HashMap::from([('a', 1), ('b', 1), ('c', 1)]),
            HashMap::from([('a', 1), ('b', 1), ('d', 1)]),
            HashMap::from([('a', 1), ('b', 1), ('f', 1)]),
            HashMap::from([('a', 1), ('b', 1), ('j', 1)]),
        ]
    );
}

#[test]
fn parse_password_exp7() {
    assert_eq!(
        Password::new("iiii oiii ooii oooi oooo").word_counts,
        vec![
            HashMap::from([('i', 4)]),
            HashMap::from([('i', 3), ('o', 1)]),
            HashMap::from([('i', 2), ('o', 2)]),
            HashMap::from([('i', 1), ('o', 3)]),
            HashMap::from([('o', 4)]),
        ]
    );
}

#[test]
fn parse_password_exp8() {
    assert_eq!(
        Password::new("oiii ioii iioi iiio").word_counts,
        vec![
            HashMap::from([('i', 3), ('o', 1)]),
            HashMap::from([('i', 3), ('o', 1)]),
            HashMap::from([('i', 3), ('o', 1)]),
            HashMap::from([('i', 3), ('o', 1)]),
        ]
    );
}

#[test]
fn detect_duplicates_exp1() {
    assert_eq!(Password::new("aa bb cc dd ee").duplicate_words(), false);
}

#[test]
fn detect_duplicates_exp2() {
    assert_eq!(Password::new("aa bb cc dd aa").duplicate_words(), true);
}

#[test]
fn detect_duplicates_exp3() {
    assert_eq!(Password::new("aa bb cc dd aaa").duplicate_words(), false);
}

#[test]
fn count_the_non_duplicate_pwds_exp1() {
    assert_eq!(
        count_valid_passwords(&vec![
            Password {
                word_counts: vec![
                    HashMap::from([('a', 2)]),
                    HashMap::from([('b', 2)]),
                    HashMap::from([('c', 2)]),
                    HashMap::from([('d', 2)]),
                    HashMap::from([('e', 2)]),
                ]
            },
            Password {
                word_counts: vec![
                    HashMap::from([('a', 2)]),
                    HashMap::from([('b', 2)]),
                    HashMap::from([('c', 2)]),
                    HashMap::from([('d', 2)]),
                    HashMap::from([('a', 2)]),
                ]
            },
            Password {
                word_counts: vec![
                    HashMap::from([('a', 2)]),
                    HashMap::from([('b', 2)]),
                    HashMap::from([('c', 2)]),
                    HashMap::from([('d', 2)]),
                    HashMap::from([('a', 3)]),
                ]
            },
        ]),
        2
    );
}
