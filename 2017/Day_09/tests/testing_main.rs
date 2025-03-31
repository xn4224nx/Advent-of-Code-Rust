#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_9;
use day_9::GarbageStream;

#[test]
fn read_stream_exp01() {
    assert_eq!(
        GarbageStream::new("./data/example_01.txt").schars,
        vec!['{', '}']
    )
}

#[test]
fn read_stream_exp02() {
    assert_eq!(
        GarbageStream::new("./data/example_02.txt").schars,
        vec!['{', '{', '{', '}', '}', '}']
    )
}

#[test]
fn read_stream_exp03() {
    assert_eq!(
        GarbageStream::new("./data/example_03.txt").schars,
        vec!['{', '{', '}', ',', '{', '}', '}']
    )
}

#[test]
fn read_stream_exp04() {
    assert_eq!(
        GarbageStream::new("./data/example_04.txt").schars,
        vec![
            '{', '{', '{', '}', ',', '{', '}', ',', '{', '{', '}', '}', '}', '}'
        ]
    )
}

#[test]
fn read_stream_exp05() {
    assert_eq!(
        GarbageStream::new("./data/example_05.txt").schars,
        vec![
            '{', '<', 'a', '>', ',', '<', 'a', '>', ',', '<', 'a', '>', ',', '<', 'a', '>', '}'
        ]
    )
}

#[test]
fn read_stream_exp06() {
    assert_eq!(
        GarbageStream::new("./data/example_06.txt").schars,
        vec![
            '{', '{', '<', 'a', 'b', '>', '}', ',', '{', '<', 'a', 'b', '>', '}', ',', '{', '<',
            'a', 'b', '>', '}', ',', '{', '<', 'a', 'b', '>', '}', '}'
        ]
    )
}

#[test]
fn read_stream_exp07() {
    assert_eq!(
        GarbageStream::new("./data/example_07.txt").schars,
        vec![
            '{', '{', '<', '!', '!', '>', '}', ',', '{', '<', '!', '!', '>', '}', ',', '{', '<',
            '!', '!', '>', '}', ',', '{', '<', '!', '!', '>', '}', '}'
        ]
    )
}

#[test]
fn read_stream_exp08() {
    assert_eq!(
        GarbageStream::new("./data/example_08.txt").schars,
        vec![
            '{', '{', '<', 'a', '!', '>', '}', ',', '{', '<', 'a', '!', '>', '}', ',', '{', '<',
            'a', '!', '>', '}', ',', '{', '<', 'a', 'b', '>', '}', '}'
        ]
    )
}

#[test]
fn score_stream_exp01() {
    assert_eq!(GarbageStream::new("./data/example_01.txt").score().0, 1)
}

#[test]
fn score_stream_exp02() {
    assert_eq!(GarbageStream::new("./data/example_02.txt").score().0, 6)
}

#[test]
fn score_stream_exp03() {
    assert_eq!(GarbageStream::new("./data/example_03.txt").score().0, 5)
}

#[test]
fn score_stream_exp04() {
    assert_eq!(GarbageStream::new("./data/example_04.txt").score().0, 16)
}

#[test]
fn score_stream_exp05() {
    assert_eq!(GarbageStream::new("./data/example_05.txt").score().0, 1)
}

#[test]
fn score_stream_exp06() {
    assert_eq!(GarbageStream::new("./data/example_06.txt").score().0, 9)
}

#[test]
fn score_stream_exp07() {
    assert_eq!(GarbageStream::new("./data/example_07.txt").score().0, 9)
}

#[test]
fn score_stream_exp08() {
    assert_eq!(GarbageStream::new("./data/example_08.txt").score().0, 3)
}

#[test]
fn count_chars_exp09() {
    assert_eq!(GarbageStream::new("./data/example_09.txt").score().1, 0)
}

#[test]
fn count_chars_exp10() {
    assert_eq!(GarbageStream::new("./data/example_10.txt").score().1, 16)
}

#[test]
fn count_chars_exp11() {
    assert_eq!(GarbageStream::new("./data/example_11.txt").score().1, 3)
}

#[test]
fn count_chars_exp12() {
    assert_eq!(GarbageStream::new("./data/example_12.txt").score().1, 2)
}

#[test]
fn count_chars_exp13() {
    assert_eq!(GarbageStream::new("./data/example_13.txt").score().1, 0)
}

#[test]
fn count_chars_exp14() {
    assert_eq!(GarbageStream::new("./data/example_14.txt").score().1, 0)
}

#[test]
fn count_chars_exp15() {
    assert_eq!(GarbageStream::new("./data/example_15.txt").score().1, 10)
}
