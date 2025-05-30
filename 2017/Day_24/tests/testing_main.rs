#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_24;
use day_24::BridgeBuilder;

#[test]
fn new_builder_exp01() {
    let test = BridgeBuilder::new("./data/example_01.txt");
    assert_eq!(
        test.parts,
        vec![
            (0, 2),
            (2, 2),
            (2, 3),
            (3, 4),
            (3, 5),
            (0, 1),
            (10, 1),
            (9, 10),
        ]
    );
}

#[test]
fn bridge_strength_exp01() {
    assert_eq!(
        BridgeBuilder::new("./data/example_01.txt").bridge_strength(&vec![5, 6, 7]),
        31
    );
}

#[test]
fn bridge_strength_exp02() {
    assert_eq!(
        BridgeBuilder::new("./data/example_01.txt").bridge_strength(&vec![0]),
        2
    );
}

#[test]
fn bridge_strength_exp03() {
    assert_eq!(
        BridgeBuilder::new("./data/example_01.txt").bridge_strength(&vec![0, 1]),
        6
    );
}

#[test]
fn starting_components_exp01() {
    assert_eq!(
        BridgeBuilder::new("./data/example_01.txt").starting_components(),
        vec![0, 5]
    );
}

#[test]
fn extract_values_exp01() {
    assert_eq!(
        BridgeBuilder::new("./data/example_01.txt").extract_values(&vec![5, 6, 7]),
        vec![0, 1, 1, 10, 10, 9]
    );
}

#[test]
fn extract_values_exp02() {
    assert_eq!(
        BridgeBuilder::new("./data/example_01.txt").extract_values(&vec![0, 2, 4]),
        vec![0, 2, 2, 3, 3, 5]
    );
}

#[test]
fn strongest_bridge_exp_01() {
    assert_eq!(
        BridgeBuilder::new("./data/example_01.txt").strongest_bridge(),
        (31, 19)
    );
}
