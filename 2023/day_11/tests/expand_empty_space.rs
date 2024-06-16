#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_11;

#[test]
fn expand_example_01() {
    let mut galaxy_positions = day_11::read_galaxy_img("./data/example_01.txt");
    let empty_space = day_11::find_empty_space(&galaxy_positions);
    day_11::expand_empty_space(&mut galaxy_positions, &empty_space, 2);

    assert_eq!(
        galaxy_positions,
        day_11::read_galaxy_img("./data/example_02.txt")
    );
}
