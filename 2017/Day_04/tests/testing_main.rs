#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_4;
use day_4::{count_valid_passwords, parse_passwords, Password};
use std::collections::HashMap;

#[test]
fn read_passwords_exp01() {
    assert_eq!(
        parse_passwords("./data/example_01.txt"),
        vec![
            Password {
                word_counts: vec![
                    HashMap::from([('a', 2)]),
                    HashMap::from([('b', 2)]),
                    HashMap::from([('c', 2)]),
                    HashMap::from([('d', 2)]),
                    HashMap::from([('e', 2)]),
                ],
                words: vec![
                    "aa".to_string(),
                    "bb".to_string(),
                    "cc".to_string(),
                    "dd".to_string(),
                    "ee".to_string()
                ]
            },
            Password {
                word_counts: vec![
                    HashMap::from([('a', 2)]),
                    HashMap::from([('b', 2)]),
                    HashMap::from([('c', 2)]),
                    HashMap::from([('d', 2)]),
                    HashMap::from([('a', 2)]),
                ],
                words: vec![
                    "aa".to_string(),
                    "bb".to_string(),
                    "cc".to_string(),
                    "dd".to_string(),
                    "aa".to_string()
                ]
            },
            Password {
                word_counts: vec![
                    HashMap::from([('a', 2)]),
                    HashMap::from([('b', 2)]),
                    HashMap::from([('c', 2)]),
                    HashMap::from([('d', 2)]),
                    HashMap::from([('a', 3)]),
                ],
                words: vec![
                    "aa".to_string(),
                    "bb".to_string(),
                    "cc".to_string(),
                    "dd".to_string(),
                    "aaa".to_string()
                ]
            },
        ]
    )
}

#[test]
fn read_passwords_exp02() {
    assert_eq!(
        parse_passwords("./data/example_02.txt"),
        vec![
            Password {
                word_counts: vec![
                    HashMap::from([('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1)]),
                    HashMap::from([('f', 1), ('g', 1), ('h', 1), ('i', 1), ('j', 1)]),
                ],
                words: vec!["abcde".to_string(), "fghij".to_string(),]
            },
            Password {
                word_counts: vec![
                    HashMap::from([('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1)]),
                    HashMap::from([('x', 1), ('y', 1), ('z', 1)]),
                    HashMap::from([('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1)]),
                ],
                words: vec!["abcde".to_string(), "xyz".to_string(), "ecdab".to_string()]
            },
            Password {
                word_counts: vec![
                    HashMap::from([('a', 1)]),
                    HashMap::from([('a', 1), ('b', 1)]),
                    HashMap::from([('a', 1), ('b', 1), ('c', 1)]),
                    HashMap::from([('a', 1), ('b', 1), ('d', 1)]),
                    HashMap::from([('a', 1), ('b', 1), ('f', 1)]),
                    HashMap::from([('a', 1), ('b', 1), ('j', 1)]),
                ],
                words: vec![
                    "a".to_string(),
                    "ab".to_string(),
                    "abc".to_string(),
                    "abd".to_string(),
                    "abf".to_string(),
                    "abj".to_string(),
                ]
            },
            Password {
                word_counts: vec![
                    HashMap::from([('i', 4)]),
                    HashMap::from([('i', 3), ('o', 1)]),
                    HashMap::from([('i', 2), ('o', 2)]),
                    HashMap::from([('i', 1), ('o', 3)]),
                    HashMap::from([('o', 4)]),
                ],
                words: vec![
                    "iiii".to_string(),
                    "oiii".to_string(),
                    "ooii".to_string(),
                    "oooi".to_string(),
                    "oooo".to_string(),
                ]
            },
            Password {
                word_counts: vec![
                    HashMap::from([('i', 3), ('o', 1)]),
                    HashMap::from([('i', 3), ('o', 1)]),
                    HashMap::from([('i', 3), ('o', 1)]),
                    HashMap::from([('i', 3), ('o', 1)]),
                ],
                words: vec![
                    "oiii".to_string(),
                    "ioii".to_string(),
                    "iioi".to_string(),
                    "iiio".to_string()
                ]
            },
        ]
    )
}

#[test]
fn parse_password_chars_exp1() {
    assert_eq!(
        Password::new("aa bb cc dd ee").word_counts,
        vec![
            HashMap::from([('a', 2)]),
            HashMap::from([('b', 2)]),
            HashMap::from([('c', 2)]),
            HashMap::from([('d', 2)]),
            HashMap::from([('e', 2)]),
        ]
    );
}

#[test]
fn parse_password_chars_exp2() {
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
fn parse_password_chars_exp3() {
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
fn parse_password_chars_exp4() {
    assert_eq!(
        Password::new("abcde fghij").word_counts,
        vec![
            HashMap::from([('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1)]),
            HashMap::from([('f', 1), ('g', 1), ('h', 1), ('i', 1), ('j', 1)]),
        ]
    );
}

#[test]
fn parse_password_chars_exp5() {
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
fn parse_password_chars_exp6() {
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
fn parse_password_chars_exp7() {
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
fn parse_password_chars_exp8() {
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
fn parse_password_words_exp1() {
    assert_eq!(
        Password::new("aa bb cc dd ee").words,
        vec![
            "aa".to_string(),
            "bb".to_string(),
            "cc".to_string(),
            "dd".to_string(),
            "ee".to_string()
        ]
    );
}

#[test]
fn parse_password_words_exp2() {
    assert_eq!(
        Password::new("aa bb cc dd aa").words,
        vec![
            "aa".to_string(),
            "bb".to_string(),
            "cc".to_string(),
            "dd".to_string(),
            "aa".to_string()
        ]
    );
}

#[test]
fn parse_password_words_exp3() {
    assert_eq!(
        Password::new("aa bb cc dd aaa").words,
        vec![
            "aa".to_string(),
            "bb".to_string(),
            "cc".to_string(),
            "dd".to_string(),
            "aaa".to_string()
        ]
    );
}

#[test]
fn parse_password_words_exp4() {
    assert_eq!(
        Password::new("abcde fghij").words,
        vec!["abcde".to_string(), "fghij".to_string(),]
    );
}

#[test]
fn parse_password_words_exp5() {
    assert_eq!(
        Password::new("abcde xyz ecdab").words,
        vec!["abcde".to_string(), "xyz".to_string(), "ecdab".to_string()]
    );
}

#[test]
fn parse_password_words_exp6() {
    assert_eq!(
        Password::new("a ab abc abd abf abj").words,
        vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "abd".to_string(),
            "abf".to_string(),
            "abj".to_string(),
        ]
    );
}

#[test]
fn parse_password_words_exp7() {
    assert_eq!(
        Password::new("iiii oiii ooii oooi oooo").words,
        vec![
            "iiii".to_string(),
            "oiii".to_string(),
            "ooii".to_string(),
            "oooi".to_string(),
            "oooo".to_string(),
        ]
    );
}

#[test]
fn parse_password_words_exp8() {
    assert_eq!(
        Password::new("oiii ioii iioi iiio").words,
        vec![
            "oiii".to_string(),
            "ioii".to_string(),
            "iioi".to_string(),
            "iiio".to_string()
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
                ],
                words: vec![
                    "aa".to_string(),
                    "bb".to_string(),
                    "cc".to_string(),
                    "dd".to_string(),
                    "ee".to_string()
                ]
            },
            Password {
                word_counts: vec![
                    HashMap::from([('a', 2)]),
                    HashMap::from([('b', 2)]),
                    HashMap::from([('c', 2)]),
                    HashMap::from([('d', 2)]),
                    HashMap::from([('a', 2)]),
                ],
                words: vec![
                    "aa".to_string(),
                    "bb".to_string(),
                    "cc".to_string(),
                    "dd".to_string(),
                    "aa".to_string()
                ]
            },
            Password {
                word_counts: vec![
                    HashMap::from([('a', 2)]),
                    HashMap::from([('b', 2)]),
                    HashMap::from([('c', 2)]),
                    HashMap::from([('d', 2)]),
                    HashMap::from([('a', 3)]),
                ],
                words: vec![
                    "aa".to_string(),
                    "bb".to_string(),
                    "cc".to_string(),
                    "dd".to_string(),
                    "aaa".to_string()
                ]
            },
        ]),
        2
    );
}
