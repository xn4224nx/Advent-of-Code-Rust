#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_12;

#[test]
fn read_example_00() {
    assert_eq!(
        day_12::read_spring_condition_data("./data/example_00.txt"),
        vec![
            (String::from("#.#.###"), vec![1, 1, 3]),
            (String::from(".#...#....###."), vec![1, 1, 3]),
            (String::from(".#.###.#.######"), vec![1, 3, 1, 6]),
            (String::from("####.#...#..."), vec![4, 1, 1]),
            (String::from("#....######..#####."), vec![1, 6, 5]),
            (String::from(".###.##....#"), vec![3, 2, 1])
        ]
    )
}

#[test]
fn read_example_01() {
    assert_eq!(
        day_12::read_spring_condition_data("./data/example_01.txt"),
        vec![
            (String::from("???.###"), vec![1, 1, 3]),
            (String::from(".??..??...?##."), vec![1, 1, 3]),
            (String::from("?#?#?#?#?#?#?#?"), vec![1, 3, 1, 6]),
            (String::from("????.#...#..."), vec![4, 1, 1]),
            (String::from("????.######..#####."), vec![1, 6, 5]),
            (String::from("?###????????"), vec![3, 2, 1])
        ]
    )
}

#[test]
fn generate_spring_stats_00() {
    assert_eq!(
        day_12::generate_spring_stats(&String::from("???.###"), &vec![1, 1, 3]),
        (3, 2, 1, 1)
    );
}

#[test]
fn generate_spring_stats_01() {
    assert_eq!(
        day_12::generate_spring_stats(&String::from(".??..??...?##."), &vec![1, 1, 3]),
        (2, 3, 7, 2)
    );
}

#[test]
fn generate_spring_stats_02() {
    assert_eq!(
        day_12::generate_spring_stats(&String::from("?#?#?#?#?#?#?#?"), &vec![1, 3, 1, 6]),
        (7, 4, 0, 4)
    );
}

#[test]
fn generate_spring_stats_03() {
    assert_eq!(
        day_12::generate_spring_stats(&String::from("????.#...#..."), &vec![4, 1, 1]),
        (2, 4, 7, 0)
    );
}

#[test]
fn generate_spring_stats_04() {
    assert_eq!(
        day_12::generate_spring_stats(&String::from("????.######..#####."), &vec![1, 6, 5]),
        (11, 1, 4, 3)
    );
}

#[test]
fn generate_spring_stats_05() {
    assert_eq!(
        day_12::generate_spring_stats(&String::from("?###????????"), &vec![3, 2, 1]),
        (3, 3, 0, 6)
    );
}

#[test]
fn validate_spring_config_00() {
    assert_eq!(
        day_12::validate_spring_config(&String::from("#.#.###"), &vec![1, 1, 3], true),
        true
    );
}

#[test]
fn validate_spring_config_01() {
    assert_eq!(
        day_12::validate_spring_config(&String::from(".#...#....###."), &vec![1, 1, 3], true),
        true
    );
}

#[test]
fn validate_spring_config_02() {
    assert_eq!(
        day_12::validate_spring_config(&String::from(".#.###.#.######"), &vec![1, 3, 1, 6], true),
        true
    );
}

#[test]
fn validate_spring_config_03() {
    assert_eq!(
        day_12::validate_spring_config(&String::from("####.#...#..."), &vec![4, 1, 1], true),
        true
    );
}

#[test]
fn validate_spring_config_04() {
    assert_eq!(
        day_12::validate_spring_config(&String::from("#....######..#####."), &vec![1, 6, 5], true),
        true
    );
}

#[test]
fn validate_spring_config_05() {
    assert_eq!(
        day_12::validate_spring_config(&String::from(".###.##....#"), &vec![3, 2, 1], true),
        true
    );
}

#[test]
fn verify_hidden_spring_comb_00() {
    assert_eq!(
        day_12::gen_valid_spring_config(&String::from("???.###"), &vec![1, 1, 3])[0],
        String::from("#.#.###")
    );
}

#[test]
fn verify_hidden_spring_comb_01() {
    assert_eq!(
        day_12::gen_valid_spring_config(&String::from("?###????????"), &vec![3, 2, 1]).len(),
        10
    );
}

#[test]
fn verify_hidden_spring_comb_02() {
    assert_eq!(
        day_12::gen_valid_spring_config(&String::from(".??..??...?##."), &vec![1, 1, 3]).len(),
        4
    );
}

#[test]
fn verify_hidden_spring_comb_03() {
    assert_eq!(
        day_12::gen_valid_spring_config(&String::from("?#?#?#?#?#?#?#?"), &vec![1, 3, 1, 6]).len(),
        1
    );
}

#[test]
fn verify_hidden_spring_comb_04() {
    assert_eq!(
        day_12::gen_valid_spring_config(&String::from("????.#...#..."), &vec![4, 1, 1]).len(),
        1
    );
}

#[test]
fn verify_hidden_spring_comb_05() {
    assert_eq!(
        day_12::gen_valid_spring_config(&String::from("????.######..#####."), &vec![1, 6, 5]).len(),
        4
    );
}
