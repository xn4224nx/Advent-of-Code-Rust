#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_11;

#[test]
fn empty_space_in_example_01() {
    let positions = day_11::read_galaxy_img("./data/example_01.txt");
    assert_eq!(
        day_11::find_empty_space(&positions),
        (vec![2, 5, 8], vec![3, 7])
    );
}

#[test]
fn empty_space_in_example_02() {
    let positions = day_11::read_galaxy_img("./data/example_02.txt");
    assert_eq!(
        day_11::find_empty_space(&positions),
        (vec![2, 3, 6, 7, 10, 11], vec![3, 4, 8, 9])
    );
}
