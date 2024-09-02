#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_18;
use day_18::{find_adj_lights, incre_light_grid, new_light_state, read_light_grid, Point};

use ndarray::arr2;
use std::collections::HashSet;

#[test]
fn read_light_grid_ex01() {
    assert_eq!(
        read_light_grid("./data/example_01.txt"),
        arr2(&[
            [false, true, false, true, false, true],
            [false, false, false, true, true, false],
            [true, false, false, false, false, true],
            [false, false, true, false, false, false],
            [true, false, true, false, false, true],
            [true, true, true, true, false, false],
        ])
    );
}

#[test]
fn read_light_grid_ex02() {
    assert_eq!(
        read_light_grid("./data/example_02.txt"),
        arr2(&[
            [false, false, true, true, false, false],
            [false, false, true, true, false, true],
            [false, false, false, true, true, false],
            [false, false, false, false, false, false],
            [true, false, false, false, false, false],
            [true, false, true, true, false, false],
        ])
    );
}

#[test]
fn read_light_grid_ex03() {
    assert_eq!(
        read_light_grid("./data/example_03.txt"),
        arr2(&[
            [false, false, true, true, true, false],
            [false, false, false, false, false, false],
            [false, false, true, true, true, false],
            [false, false, false, false, false, false],
            [false, true, false, false, false, false],
            [false, true, false, false, false, false],
        ])
    );
}

#[test]
fn read_light_grid_ex04() {
    assert_eq!(
        read_light_grid("./data/example_04.txt"),
        arr2(&[
            [false, false, false, true, false, false],
            [false, false, false, false, false, false],
            [false, false, false, true, false, false],
            [false, false, true, true, false, false],
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
        ])
    );
}

#[test]
fn read_light_grid_ex05() {
    assert_eq!(
        read_light_grid("./data/example_05.txt"),
        arr2(&[
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
            [false, false, true, true, false, false],
            [false, false, true, true, false, false],
            [false, false, false, false, false, false],
            [false, false, false, false, false, false],
        ])
    );
}

#[test]
fn read_light_grid_ex06() {
    assert_eq!(
        read_light_grid("./data/example_06.txt"),
        arr2(&[
            [true, true, false, true, false, true],
            [false, false, false, true, true, false],
            [true, false, false, false, false, true],
            [false, false, true, false, false, false],
            [true, false, true, false, false, true],
            [true, true, true, true, false, true],
        ])
    );
}

#[test]
fn read_light_grid_ex07() {
    assert_eq!(
        read_light_grid("./data/example_07.txt"),
        arr2(&[
            [true, false, true, true, false, true],
            [true, true, true, true, false, true],
            [false, false, false, true, true, false],
            [false, false, false, false, false, false],
            [true, false, false, false, true, false],
            [true, false, true, true, true, true],
        ])
    );
}

#[test]
fn read_light_grid_ex08() {
    assert_eq!(
        read_light_grid("./data/example_08.txt"),
        arr2(&[
            [true, false, false, true, false, true],
            [true, false, false, false, false, true],
            [false, true, false, true, true, false],
            [false, false, false, true, true, false],
            [false, true, false, false, true, true],
            [true, true, false, true, true, true],
        ])
    );
}

#[test]
fn read_light_grid_ex09() {
    assert_eq!(
        read_light_grid("./data/example_09.txt"),
        arr2(&[
            [true, false, false, false, true, true],
            [true, true, true, true, false, true],
            [false, false, true, true, false, true],
            [false, false, false, false, false, false],
            [true, true, false, false, false, false],
            [true, true, true, true, false, true],
        ])
    );
}

#[test]
fn read_light_grid_ex10() {
    assert_eq!(
        read_light_grid("./data/example_10.txt"),
        arr2(&[
            [true, false, true, true, true, true],
            [true, false, false, false, false, true],
            [false, false, false, true, false, false],
            [false, true, true, false, false, false],
            [true, false, false, false, false, false],
            [true, false, true, false, false, true],
        ])
    );
}

#[test]
fn read_light_grid_ex11() {
    assert_eq!(
        read_light_grid("./data/example_11.txt"),
        arr2(&[
            [true, true, false, true, true, true],
            [false, true, true, false, false, true],
            [false, true, true, false, false, false],
            [false, true, true, false, false, false],
            [true, false, true, false, false, false],
            [true, true, false, false, false, true],
        ])
    );
}

#[test]
fn find_adj_lights_ex01() {
    assert_eq!(
        HashSet::<Point>::from_iter(find_adj_lights(&Point { x: 0, y: 0 }, &[3, 3],)),
        HashSet::<Point>::from_iter(vec![
            Point { x: 0, y: 1 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 }
        ])
    );
}

#[test]
fn find_adj_lights_ex02() {
    assert_eq!(
        HashSet::<Point>::from_iter(find_adj_lights(&Point { x: 0, y: 1 }, &[3, 3],)),
        HashSet::<Point>::from_iter(vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 2 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 }
        ])
    );
}

#[test]
fn find_adj_lights_ex03() {
    assert_eq!(
        HashSet::<Point>::from_iter(find_adj_lights(&Point { x: 0, y: 2 }, &[3, 3],)),
        HashSet::<Point>::from_iter(vec![
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 }
        ])
    );
}

#[test]
fn find_adj_lights_ex04() {
    assert_eq!(
        HashSet::<Point>::from_iter(find_adj_lights(&Point { x: 1, y: 0 }, &[3, 3],)),
        HashSet::<Point>::from_iter(vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 0 }
        ])
    );
}

#[test]
fn find_adj_lights_ex05() {
    assert_eq!(
        HashSet::<Point>::from_iter(find_adj_lights(&Point { x: 1, y: 1 }, &[3, 3],)),
        HashSet::<Point>::from_iter(vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 0 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 }
        ])
    );
}

#[test]
fn find_adj_lights_ex06() {
    assert_eq!(
        HashSet::<Point>::from_iter(find_adj_lights(&Point { x: 1, y: 2 }, &[3, 3],)),
        HashSet::<Point>::from_iter(vec![
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 }
        ])
    );
}

#[test]
fn find_adj_lights_ex07() {
    assert_eq!(
        HashSet::<Point>::from_iter(find_adj_lights(&Point { x: 2, y: 0 }, &[3, 3],)),
        HashSet::<Point>::from_iter(vec![
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 }
        ])
    );
}

#[test]
fn find_adj_lights_ex08() {
    assert_eq!(
        HashSet::<Point>::from_iter(find_adj_lights(&Point { x: 2, y: 1 }, &[3, 3],)),
        HashSet::<Point>::from_iter(vec![
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 0 },
            Point { x: 2, y: 2 }
        ])
    );
}

#[test]
fn find_adj_lights_ex09() {
    assert_eq!(
        HashSet::<Point>::from_iter(find_adj_lights(&Point { x: 2, y: 2 }, &[3, 3],)),
        HashSet::<Point>::from_iter(vec![
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 1 }
        ])
    );
}

#[test]
fn new_light_state_ex1() {
    assert_eq!(
        new_light_state(
            &arr2(&[
                [true, true, false],
                [true, true, false],
                [false, false, false],
            ]),
            &Point { x: 1, y: 0 }
        ),
        true
    );
}

#[test]
fn new_light_state_ex2() {
    assert_eq!(
        new_light_state(
            &arr2(&[
                [true, false, false],
                [false, true, false],
                [false, true, false],
            ]),
            &Point { x: 1, y: 0 }
        ),
        true
    );
}

#[test]
fn new_light_state_ex3() {
    assert_eq!(
        new_light_state(
            &arr2(&[
                [true, true, false],
                [false, false, true],
                [false, false, false],
            ]),
            &Point { x: 0, y: 1 }
        ),
        true
    );
}

#[test]
fn new_light_state_ex4() {
    assert_eq!(
        new_light_state(
            &arr2(&[
                [false, false, true],
                [false, true, false],
                [false, true, true],
            ]),
            &Point { x: 1, y: 2 }
        ),
        false
    );
}

#[test]
fn new_light_state_ex5() {
    assert_eq!(
        new_light_state(
            &arr2(&[
                [false, false, false],
                [false, true, false],
                [false, false, false],
            ]),
            &Point { x: 0, y: 0 }
        ),
        false
    );
}

#[test]
fn new_light_state_ex6() {
    assert_eq!(
        new_light_state(
            &arr2(&[
                [false, false, false],
                [false, false, true],
                [false, false, true],
            ]),
            &Point { x: 2, y: 2 }
        ),
        false
    );
}

#[test]
fn incre_light_grid_ex1() {
    let start = read_light_grid("./data/example_01.txt");
    let end = read_light_grid("./data/example_05.txt");
    assert!(incre_light_grid(&start, 4, false) == end);
}

#[test]
fn incre_light_grid_ex2() {
    let start = read_light_grid("./data/example_01.txt");
    let end = read_light_grid("./data/example_02.txt");
    assert!(incre_light_grid(&start, 1, false) == end);
}

#[test]
fn incre_light_grid_ex3() {
    let start = read_light_grid("./data/example_02.txt");
    let end = read_light_grid("./data/example_03.txt");
    assert!(incre_light_grid(&start, 1, false) == end);
}

#[test]
fn incre_light_grid_ex4() {
    let start = read_light_grid("./data/example_03.txt");
    let end = read_light_grid("./data/example_04.txt");
    assert!(incre_light_grid(&start, 1, false) == end);
}

#[test]
fn incre_light_grid_ex5() {
    let start = read_light_grid("./data/example_04.txt");
    let end = read_light_grid("./data/example_05.txt");
    assert!(incre_light_grid(&start, 1, false) == end);
}

#[test]
fn incre_light_grid_ex1_cor() {
    let start = read_light_grid("./data/example_06.txt");
    let end = read_light_grid("./data/example_11.txt");
    assert!(incre_light_grid(&start, 5, true) == end);
}

#[test]
fn incre_light_grid_ex2_cor() {
    let start = read_light_grid("./data/example_06.txt");
    let end = read_light_grid("./data/example_07.txt");
    assert!(incre_light_grid(&start, 1, true) == end);
}

#[test]
fn incre_light_grid_ex3_cor() {
    let start = read_light_grid("./data/example_07.txt");
    let end = read_light_grid("./data/example_08.txt");
    assert!(incre_light_grid(&start, 1, true) == end);
}

#[test]
fn incre_light_grid_ex4_cor() {
    let start = read_light_grid("./data/example_08.txt");
    let end = read_light_grid("./data/example_09.txt");
    assert!(incre_light_grid(&start, 1, true) == end);
}

#[test]
fn incre_light_grid_ex5_cor() {
    let start = read_light_grid("./data/example_09.txt");
    let end = read_light_grid("./data/example_10.txt");
    assert!(incre_light_grid(&start, 1, true) == end);
}

#[test]
fn incre_light_grid_ex6_cor() {
    let start = read_light_grid("./data/example_10.txt");
    let end = read_light_grid("./data/example_11.txt");
    assert!(incre_light_grid(&start, 1, true) == end);
}
