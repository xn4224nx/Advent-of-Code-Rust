#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_02;
use day_02::{Direc, KeyPad};

#[test]
fn test_read_directions_01() {
    let mut test_pad = KeyPad::new(vec![], (0, 0));
    test_pad.read_keypad_commands("./data/example_01.txt");

    assert_eq!(
        test_pad.directs,
        vec![
            vec![Direc::Up, Direc::Left, Direc::Left],
            vec![
                Direc::Right,
                Direc::Right,
                Direc::Down,
                Direc::Down,
                Direc::Down,
            ],
            vec![
                Direc::Left,
                Direc::Up,
                Direc::Right,
                Direc::Down,
                Direc::Left,
            ],
            vec![Direc::Up, Direc::Up, Direc::Up, Direc::Up, Direc::Down,],
        ]
    );
}

#[test]
fn test_move_up() {
    let mut test_pad = KeyPad::new(
        vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ],
        (1, 1),
    );

    test_pad.move_position(Direc::Up);
    assert_eq!(test_pad.pos, (0, 1));
}

#[test]
fn test_move_down() {
    let mut test_pad = KeyPad::new(
        vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ],
        (1, 1),
    );

    test_pad.move_position(Direc::Down);
    assert_eq!(test_pad.pos, (2, 1));
}

#[test]
fn test_move_left() {
    let mut test_pad = KeyPad::new(
        vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ],
        (1, 1),
    );

    test_pad.move_position(Direc::Left);
    assert_eq!(test_pad.pos, (1, 0));
}

#[test]
fn test_move_right() {
    let mut test_pad = KeyPad::new(
        vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ],
        (1, 1),
    );

    test_pad.move_position(Direc::Right);
    assert_eq!(test_pad.pos, (1, 2));
}

#[test]
fn test_move_invalid_up() {
    let mut test_pad = KeyPad::new(
        vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ],
        (0, 1),
    );

    test_pad.move_position(Direc::Up);
    assert_eq!(test_pad.pos, (0, 1));

    test_pad.move_position(Direc::Down);
    assert_eq!(test_pad.pos, (1, 1));
}

#[test]
fn test_move_invalid_down() {
    let mut test_pad = KeyPad::new(
        vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ],
        (2, 1),
    );

    test_pad.move_position(Direc::Down);
    assert_eq!(test_pad.pos, (2, 1));

    test_pad.move_position(Direc::Up);
    assert_eq!(test_pad.pos, (1, 1));
}

#[test]
fn test_move_invalid_left() {
    let mut test_pad = KeyPad::new(
        vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ],
        (1, 0),
    );

    test_pad.move_position(Direc::Left);
    assert_eq!(test_pad.pos, (1, 0));

    test_pad.move_position(Direc::Right);
    assert_eq!(test_pad.pos, (1, 1));
}

#[test]
fn test_move_invalid_right() {
    let mut test_pad = KeyPad::new(
        vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ],
        (1, 2),
    );
    test_pad.move_position(Direc::Right);
    assert_eq!(test_pad.pos, (1, 2));

    test_pad.move_position(Direc::Left);
    assert_eq!(test_pad.pos, (1, 1));
}

#[test]
fn test_find_access_code_01() {
    let mut test_pad = KeyPad::new(
        vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ],
        (1, 1),
    );

    assert_eq!(test_pad.find_access_code(), String::from("1985"));
}