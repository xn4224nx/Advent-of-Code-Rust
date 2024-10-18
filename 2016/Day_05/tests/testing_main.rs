#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_05;
use day_05::{decipher_password, is_char_hash, md5_idx_hash};

#[test]
fn md5_idx_hash_exp1() {
    assert_eq!(&md5_idx_hash(&"abc".to_string(), 3231929)[..5], "00000")
}

#[test]
fn md5_idx_hash_exp2() {
    assert_eq!(&md5_idx_hash(&"abc".to_string(), 5017308)[..9], "000008f82")
}

#[test]
fn md5_idx_hash_exp3() {
    assert_eq!(&md5_idx_hash(&"abc".to_string(), 5278568)[..6], "00000f")
}

#[test]
fn is_char_hash_exp1() {
    assert_eq!(
        is_char_hash(&md5_idx_hash(&"abc".to_string(), 3231929)),
        true
    )
}

#[test]
fn is_char_hash_exp2() {
    assert_eq!(
        is_char_hash(&md5_idx_hash(&"abc".to_string(), 5017308)),
        true
    )
}

#[test]
fn is_char_hash_exp3() {
    assert_eq!(
        is_char_hash(&md5_idx_hash(&"abc".to_string(), 5278568)),
        true
    )
}

#[test]
fn is_char_hash_exp4() {
    assert_eq!(is_char_hash(&md5_idx_hash(&"abc".to_string(), 32)), false)
}

#[test]
fn is_char_hash_exp5() {
    assert_eq!(is_char_hash(&md5_idx_hash(&"abc".to_string(), 7308)), false)
}

#[test]
fn is_char_hash_exp6() {
    assert_eq!(
        is_char_hash(&md5_idx_hash(&"abc".to_string(), 5278567)),
        false
    )
}

#[test]
fn decipher_password_exp1() {
    assert_eq!(
        decipher_password(&"abc".to_string(), 8, false),
        "18f47a30".to_string()
    )
}

#[test]
fn decipher_password_exp2() {
    assert_eq!(
        decipher_password(&"abc".to_string(), 8, true),
        "05ace8e3".to_string()
    )
}
