#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_9;
use day_9::GarbageStream;
use std::collections::HashMap;

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
    assert_eq!(GarbageStream::new("./data/example_01.txt").score(), 1)
}

#[test]
fn score_stream_exp02() {
    assert_eq!(GarbageStream::new("./data/example_02.txt").score(), 6)
}

#[test]
fn score_stream_exp03() {
    assert_eq!(GarbageStream::new("./data/example_03.txt").score(), 5)
}

#[test]
fn score_stream_exp04() {
    assert_eq!(GarbageStream::new("./data/example_04.txt").score(), 16)
}

#[test]
fn score_stream_exp05() {
    assert_eq!(GarbageStream::new("./data/example_05.txt").score(), 1)
}

#[test]
fn score_stream_exp06() {
    assert_eq!(GarbageStream::new("./data/example_06.txt").score(), 9)
}

#[test]
fn score_stream_exp07() {
    assert_eq!(GarbageStream::new("./data/example_07.txt").score(), 9)
}

#[test]
fn score_stream_exp08() {
    assert_eq!(GarbageStream::new("./data/example_08.txt").score(), 3)
}
