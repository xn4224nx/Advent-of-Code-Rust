#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/hasher.rs"]
mod day_14;
use day_14::KnotHash;

#[test]
fn read_twist_as_ascii_exp01() {
    assert_eq!(
        KnotHash::new("1,2,3").twists,
        vec![49, 44, 50, 44, 51, 17, 31, 73, 47, 23]
    );
}

#[test]
fn read_twist_as_ascii_exp02() {
    assert_eq!(KnotHash::new("").twists, vec![17, 31, 73, 47, 23]);
}

#[test]
fn digest_exp03() {
    assert_eq!(
        KnotHash::new("").digest(),
        String::from("a2582a3a0e66e6e86e3812dcb672a272")
    );
}

#[test]
fn digest_exp04() {
    assert_eq!(
        KnotHash::new("AoC 2017").digest(),
        String::from("33efeb34ea91902bb2f59c9920caa6cd")
    );
}

#[test]
fn digest_exp05() {
    assert_eq!(
        KnotHash::new("1,2,3").digest(),
        String::from("3efbe78a8d82f29979031a4aa0b16a9d")
    );
}

#[test]
fn digest_exp06() {
    assert_eq!(
        KnotHash::new("1,2,4").digest(),
        String::from("63960835bcdc130f0b66d7ff4f6a5a8e")
    );
}
