#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_15;
use day_15::{cookie_calories, highest_cookie_score, read_cookie_data, score_cookie_comb, Cookie};

#[test]
fn read_cookie_data_ex_01() {
    assert_eq!(
        read_cookie_data("./data/example_01.txt"),
        vec![
            Cookie {
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                calories: 8,
            },
            Cookie {
                capacity: 2,
                durability: 3,
                flavor: -2,
                texture: -1,
                calories: 3,
            },
        ]
    );
}

#[test]
fn score_cookie_comb_ex_01() {
    let data = read_cookie_data("./data/example_01.txt");
    assert_eq!(score_cookie_comb(&data, &vec![44, 56]), 62842880);
}

#[test]
fn highest_cookie_score_ex_01() {
    let data = read_cookie_data("./data/example_01.txt");
    assert_eq!(highest_cookie_score(&data, 100, false), 62842880);
}

#[test]
fn highest_cookie_score_cal_cnt_ex_01() {
    let data = read_cookie_data("./data/example_01.txt");
    assert_eq!(highest_cookie_score(&data, 100, true), 57600000);
}

#[test]
fn cookie_calories_ex_01() {
    let data = read_cookie_data("./data/example_01.txt");
    assert_eq!(cookie_calories(&data, &vec![40, 60]), 500)
}
