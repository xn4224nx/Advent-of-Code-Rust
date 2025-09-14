#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod main;
use crate::main::MarbleGame;
use std::collections::VecDeque;

#[test]
fn initial_game_exp0() {
    let test = MarbleGame::new(9, 25);

    assert_eq!(test.marbs, VecDeque::from([0]));
    assert_eq!(test.players, 9);
    assert_eq!(test.player_scores, vec![0; 9]);
    assert_eq!(test.last_marb_pnts, 25);
    assert_eq!(test.curr_marb, 1);
    assert_eq!(test.curr_player, 0);
}

#[test]
fn place_marb_exp00() {
    let mut test = MarbleGame::new(9, 25);
    test.place_marb();

    assert_eq!(test.curr_marb, 2);
    assert_eq!(test.curr_player, 1);
    assert_eq!(test.marbs, VecDeque::from([1, 0]));
}

#[test]
fn place_marb_exp01() {
    let mut test = MarbleGame::new(9, 25);
    test.curr_marb = 2;
    test.marbs = VecDeque::from([1, 0]);
    test.curr_player = 1;

    test.place_marb();

    assert_eq!(test.curr_marb, 3);
    assert_eq!(test.curr_player, 2);
    assert_eq!(test.marbs, VecDeque::from([2, 1, 0]));
}

#[test]
fn place_marb_exp02() {
    let mut test = MarbleGame::new(9, 25);
    test.curr_marb = 3;
    test.marbs = VecDeque::from([2, 1, 0]);
    test.curr_player = 2;

    test.place_marb();

    assert_eq!(test.curr_marb, 4);
    assert_eq!(test.curr_player, 3);
    assert_eq!(test.marbs, VecDeque::from([3, 0, 2, 1]));
}

#[test]
fn place_marb_exp03() {
    let mut test = MarbleGame::new(9, 25);
    test.curr_marb = 4;
    test.marbs = VecDeque::from([3, 0, 2, 1]);
    test.curr_player = 3;

    test.place_marb();

    assert_eq!(test.curr_marb, 5);
    assert_eq!(test.curr_player, 4);
    assert_eq!(test.marbs, VecDeque::from([4, 2, 1, 3, 0]));
}

#[test]
fn place_marb_exp04() {
    let mut test = MarbleGame::new(9, 25);
    test.curr_marb = 5;
    test.marbs = VecDeque::from([4, 2, 1, 3, 0]);
    test.curr_player = 4;

    test.place_marb();

    assert_eq!(test.curr_marb, 6);
    assert_eq!(test.curr_player, 5);
    assert_eq!(test.marbs, VecDeque::from([5, 1, 3, 0, 4, 2]));
}

#[test]
fn place_marb_exp05() {
    let mut test = MarbleGame::new(9, 25);
    test.curr_marb = 6;
    test.marbs = VecDeque::from([5, 1, 3, 0, 4, 2]);
    test.curr_player = 5;

    test.place_marb();

    assert_eq!(test.curr_marb, 7);
    assert_eq!(test.curr_player, 6);
    assert_eq!(test.marbs, VecDeque::from([6, 3, 0, 4, 2, 5, 1]));
}

#[test]
fn place_marb_exp06() {
    let mut test = MarbleGame::new(9, 25);
    test.curr_marb = 7;
    test.marbs = VecDeque::from([6, 3, 0, 4, 2, 5, 1]);
    test.curr_player = 6;

    test.place_marb();

    assert_eq!(test.curr_marb, 8);
    assert_eq!(test.curr_player, 7);
    assert_eq!(test.marbs, VecDeque::from([7, 0, 4, 2, 5, 1, 6, 3]));
}

#[test]
fn place_marb_exp07() {
    let mut test = MarbleGame::new(9, 25);
    test.curr_marb = 8;
    test.marbs = VecDeque::from([7, 0, 4, 2, 5, 1, 6, 3]);
    test.curr_player = 7;

    test.place_marb();

    assert_eq!(test.curr_marb, 9);
    assert_eq!(test.curr_player, 8);
    assert_eq!(test.marbs, VecDeque::from([8, 4, 2, 5, 1, 6, 3, 7, 0]));
}

#[test]
fn place_marb_exp08() {
    let mut test = MarbleGame::new(9, 25);
    test.curr_marb = 9;
    test.marbs = VecDeque::from([8, 4, 2, 5, 1, 6, 3, 7, 0]);
    test.curr_player = 8;

    test.place_marb();

    assert_eq!(test.curr_marb, 10);
    assert_eq!(test.curr_player, 0);
    assert_eq!(test.marbs, VecDeque::from([9, 2, 5, 1, 6, 3, 7, 0, 8, 4]));
}

#[test]
fn place_marb_exp09() {
    let mut test = MarbleGame::new(9, 25);
    test.curr_marb = 10;
    test.marbs = VecDeque::from([9, 2, 5, 1, 6, 3, 7, 0, 8, 4]);
    test.curr_player = 0;

    test.place_marb();

    assert_eq!(test.curr_marb, 11);
    assert_eq!(test.curr_player, 1);
    assert_eq!(
        test.marbs,
        VecDeque::from([10, 5, 1, 6, 3, 7, 0, 8, 4, 9, 2])
    );
}

#[test]
fn place_marb_exp10() {
    let mut test = MarbleGame::new(9, 25);
    test.curr_marb = 11;
    test.marbs = VecDeque::from([10, 5, 1, 6, 3, 7, 0, 8, 4, 9, 2]);
    test.curr_player = 1;

    test.place_marb();

    assert_eq!(test.curr_marb, 12);
    assert_eq!(test.curr_player, 2);
    assert_eq!(
        test.marbs,
        VecDeque::from([11, 1, 6, 3, 7, 0, 8, 4, 9, 2, 10, 5])
    );
}

#[test]
fn place_marb_exp11() {
    let mut test = MarbleGame::new(9, 25);
    test.curr_marb = 12;
    test.marbs = VecDeque::from([11, 1, 6, 3, 7, 0, 8, 4, 9, 2, 10, 5]);
    test.curr_player = 2;

    test.place_marb();

    assert_eq!(test.curr_marb, 13);
    assert_eq!(test.curr_player, 3);
    assert_eq!(
        test.marbs,
        VecDeque::from([12, 6, 3, 7, 0, 8, 4, 9, 2, 10, 5, 11, 1])
    );
}

#[test]
fn place_marb_exp12() {
    let mut test = MarbleGame::new(9, 25);
    test.curr_marb = 13;
    test.marbs = VecDeque::from([12, 6, 3, 7, 0, 8, 4, 9, 2, 10, 5, 11, 1]);
    test.curr_player = 3;

    test.place_marb();

    assert_eq!(test.curr_marb, 14);
    assert_eq!(test.curr_player, 4);
    assert_eq!(
        test.marbs,
        VecDeque::from([13, 3, 7, 0, 8, 4, 9, 2, 10, 5, 11, 1, 12, 6])
    );
}

#[test]
fn place_marb_exp13() {
    let mut test = MarbleGame::new(9, 25);
    test.curr_marb = 23;
    test.marbs = VecDeque::from([
        22, 11, 1, 12, 6, 13, 3, 14, 7, 15, 0, 16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 21, 5,
    ]);
    test.curr_player = 4;

    test.place_marb();

    assert_eq!(test.curr_marb, 24);
    assert_eq!(test.curr_player, 5);
    assert_eq!(test.player_scores[4], 32);
    assert_eq!(
        test.marbs,
        VecDeque::from([
            19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15, 0, 16, 8, 17, 4, 18
        ])
    );
}

#[test]
fn highest_score_exp0() {
    assert_eq!(MarbleGame::new(10, 1618).highest_score(), 8317);
}

#[test]
fn highest_score_exp1() {
    assert_eq!(MarbleGame::new(13, 7999).highest_score(), 146373);
}

#[test]
fn highest_score_exp2() {
    assert_eq!(MarbleGame::new(17, 1104).highest_score(), 2764);
}

#[test]
fn highest_score_exp3() {
    assert_eq!(MarbleGame::new(21, 6111).highest_score(), 54718);
}

#[test]
fn highest_score_exp4() {
    assert_eq!(MarbleGame::new(30, 5807).highest_score(), 37305);
}

#[test]
fn highest_score_exp5() {
    assert_eq!(MarbleGame::new(9, 25).highest_score(), 32);
}
