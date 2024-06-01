#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_08;
use std::collections::HashMap;

#[test]
fn convert_simple_map() {
    let result = day_08::parse_map(vec![(
        String::from("AAA"),
        String::from("BBB"),
        String::from("CCC"),
    )]);

    assert_eq!(
        result,
        HashMap::from([(
            String::from("AAA"),
            (String::from("BBB"), String::from("CCC"))
        ),])
    );
}

#[test]
fn convert_medium_map() {
    let result = day_08::parse_map(vec![
        (
            String::from("AAA"),
            String::from("BBB"),
            String::from("CCC"),
        ),
        (
            String::from("BBB"),
            String::from("AAA"),
            String::from("CCC"),
        ),
        (
            String::from("CCC"),
            String::from("AAA"),
            String::from("BBB"),
        ),
    ]);

    assert_eq!(
        result,
        HashMap::from([
            (
                String::from("AAA"),
                (String::from("BBB"), String::from("CCC"))
            ),
            (
                String::from("BBB"),
                (String::from("AAA"), String::from("CCC"))
            ),
            (
                String::from("CCC"),
                (String::from("AAA"), String::from("BBB"))
            ),
        ])
    );
}
