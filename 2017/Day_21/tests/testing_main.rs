#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_21;
use day_21::{FractalArt, Pixels};
use std::collections::HashMap;

#[test]
fn new_art_exp01() {
    let test = FractalArt::new("./data/example_01.txt");
    assert_eq!(test.transformations.len(), 12);
    assert_eq!(
        test.state,
        Pixels {
            square: vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1]]
        }
    );
}

#[test]
fn rotate_once() {
    let mut test = Pixels {
        square: vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1]],
    };
    test.rotate_right(1);
    assert_eq!(
        test.square,
        vec![vec![1, 0, 0], vec![1, 0, 1], vec![1, 1, 0],]
    );
}

#[test]
fn rotate_two_times() {
    let mut test = Pixels {
        square: vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1]],
    };
    test.rotate_right(2);
    assert_eq!(
        test.square,
        vec![vec![1, 1, 1], vec![1, 0, 0], vec![0, 1, 0],]
    );
}

#[test]
fn rotate_three_times() {
    let mut test = Pixels {
        square: vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1]],
    };
    test.rotate_right(3);
    assert_eq!(
        test.square,
        vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1],]
    );
}

#[test]
fn rotate_four_times() {
    let mut test = Pixels {
        square: vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1]],
    };
    test.rotate_right(4);
    assert_eq!(
        test.square,
        vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1],]
    );
}

#[test]
fn vertically_flip_pixels() {
    let mut test = Pixels {
        square: vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1]],
    };
    test.flip_vertical();
    assert_eq!(
        test.square,
        vec![vec![1, 1, 1], vec![0, 0, 1], vec![0, 1, 0],]
    );
}

#[test]
fn horizontally_flip_pixels() {
    let mut test = Pixels {
        square: vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1]],
    };
    test.flip_horizontal();
    assert_eq!(
        test.square,
        vec![vec![0, 1, 0], vec![1, 0, 0], vec![1, 1, 1],]
    );
}

#[test]
fn transformation_exp01() {
    let mut test = FractalArt::new("./data/example_01.txt");
    test.transform();
    assert_eq!(
        test.state,
        Pixels {
            square: vec![
                vec![1, 0, 0, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![1, 0, 0, 1]
            ]
        }
    );
}

#[test]
fn transformation_exp02() {
    let mut test = FractalArt::new("./data/example_01.txt");
    test.transform();
    test.transform();
    assert_eq!(
        test.state,
        Pixels {
            square: vec![
                vec![1, 1, 0, 1, 1, 0],
                vec![1, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0],
                vec![1, 1, 0, 1, 1, 0],
                vec![1, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0],
            ]
        }
    );
}

#[test]
fn count_on_pixels() {
    assert_eq!(FractalArt::new("./data/example_01.txt").on_pixels(), 5);
}

#[test]
fn pixels_after_trans() {
    assert_eq!(
        FractalArt::new("./data/example_01.txt").final_on_pixels(2),
        12
    );
}
