#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_13;

#[test]
fn read_example_00() {
    assert_eq!(
        day_13::read_raw_notes("./data/example_00.txt"),
        vec![vec![
            vec!['#', '.', '#', '#', '.', '.', '#', '#', '.'],
            vec!['.', '.', '#', '.', '#', '#', '.', '#', '.'],
            vec!['#', '#', '.', '.', '.', '.', '.', '.', '#'],
            vec!['#', '#', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '#', '.', '#', '#', '.', '#', '.'],
            vec!['.', '.', '#', '#', '.', '.', '#', '#', '.'],
            vec!['#', '.', '#', '.', '#', '#', '.', '#', '.']
        ]]
    );
}

#[test]
fn read_example_01() {
    assert_eq!(
        day_13::read_raw_notes("./data/example_01.txt"),
        vec![vec![
            vec!['#', '.', '.', '.', '#', '#', '.', '.', '#'],
            vec!['#', '.', '.', '.', '.', '#', '.', '.', '#'],
            vec!['.', '.', '#', '#', '.', '.', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#', '.', '#', '#', '.'],
            vec!['#', '#', '#', '#', '#', '.', '#', '#', '.'],
            vec!['.', '.', '#', '#', '.', '.', '#', '#', '#'],
            vec!['#', '.', '.', '.', '.', '#', '.', '.', '#']
        ]]
    );
}

#[test]
fn read_input() {
    assert_eq!(day_13::read_raw_notes("./data/input.txt").len(), 100);
}

#[test]
fn find_refect_example_00() {
    let examp = &day_13::read_raw_notes("./data/example_00.txt")[0];

    let horiz_comp: Vec<bool> = (0..examp.len())
        .map(|x| day_13::is_horiz_reflection(&examp, x, 0))
        .collect();
    let verti_comp: Vec<bool> = (0..examp[0].len())
        .map(|x| day_13::is_verti_reflection(&examp, x, 0))
        .collect();

    assert_eq!(
        horiz_comp,
        vec![false, false, false, false, false, false, false]
    );
    assert_eq!(
        verti_comp,
        vec![false, false, false, false, true, false, false, false, false]
    );
}

#[test]
fn find_refect_example_01() {
    let examp = &day_13::read_raw_notes("./data/example_01.txt")[0];

    let horiz_comp: Vec<bool> = (0..examp.len())
        .map(|x| day_13::is_horiz_reflection(&examp, x, 0))
        .collect();
    let verti_comp: Vec<bool> = (0..examp[0].len())
        .map(|x| day_13::is_verti_reflection(&examp, x, 0))
        .collect();

    assert_eq!(
        horiz_comp,
        vec![false, false, false, true, false, false, false]
    );
    assert_eq!(
        verti_comp,
        vec![false, false, false, false, false, false, false, false, false]
    );
}

#[test]
fn score_example_00() {
    assert_eq!(
        day_13::ashfield_score(&day_13::read_raw_notes("./data/example_00.txt")[0], 0),
        5
    );
}

#[test]
fn score_example_01() {
    assert_eq!(
        day_13::ashfield_score(&day_13::read_raw_notes("./data/example_01.txt")[0], 0),
        400
    );
}

#[test]
fn score_example_02() {
    assert_eq!(
        day_13::ashfield_score(&day_13::read_raw_notes("./data/example_00.txt")[0], 1),
        300
    );
}

#[test]
fn score_example_03() {
    assert_eq!(
        day_13::ashfield_score(&day_13::read_raw_notes("./data/example_01.txt")[0], 1),
        100
    );
}

#[test]
fn score_input() {
    assert_eq!(
        day_13::read_raw_notes("./data/input.txt")
            .iter()
            .map(|x| day_13::ashfield_score(x, 0))
            .sum::<usize>(),
        31877
    );
}
