#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_16;
use day_16::{Move, Promenade};

#[test]
fn read_data_exp01() {
    let test = Promenade::new("abcde", "./data/example_01.txt");
    assert_eq!(test.group, vec!['a', 'b', 'c', 'd', 'e']);
    assert_eq!(
        test.instruc,
        vec![Move::Spin(1), Move::Exchange(3, 4), Move::Partner('e', 'b'),]
    );
}

#[test]
fn spin_exp01() {
    let mut test = Promenade::new("abcde", "./data/example_01.txt");
    test.spin(1);
    assert_eq!(test.group, vec!['e', 'a', 'b', 'c', 'd']);
}

#[test]
fn exchange_exp01() {
    let mut test = Promenade::new("eabcd", "./data/example_01.txt");
    test.exchange(3, 4);
    assert_eq!(test.group, vec!['e', 'a', 'b', 'd', 'c']);
}

#[test]
fn partner_exp01() {
    let mut test = Promenade::new("eabdc", "./data/example_01.txt");
    test.partner('e', 'b');
    assert_eq!(test.group, vec!['b', 'a', 'e', 'd', 'c']);
}

#[test]
fn dance_exp01() {
    let mut test = Promenade::new("abcde", "./data/example_01.txt");
    test.dance();
    assert_eq!(test.group, vec!['b', 'a', 'e', 'd', 'c']);
}

#[test]
fn dance_exp02() {
    assert_eq!(
        Promenade::new("abcde", "./data/example_01.txt").one_dance(),
        String::from("baedc")
    );
}
