#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_7;
use day_7::{Program, ProgramStack};
use std::collections::HashMap;

#[test]
fn parse_program_exp1() {
    let test = Program::new("pbga (66)");
    assert_eq!(test.name, String::from("pbga"));
    assert_eq!(test.weight, 66);
    assert_eq!(test.above, Vec::<String>::new());
    assert_eq!(test.above_weight, 0);
}

#[test]
fn parse_program_exp2() {
    let test = Program::new("fwft (72) -> ktlj, cntj, xhth");
    assert_eq!(test.name, String::from("fwft"));
    assert_eq!(test.weight, 72);
    assert_eq!(
        test.above,
        vec![
            String::from("ktlj"),
            String::from("cntj"),
            String::from("xhth")
        ]
    );
    assert_eq!(test.above_weight, 0);
}

#[test]
fn parse_program_exp3() {
    let test = Program::new("padx (45) -> pbga, havc, qoyq");
    assert_eq!(test.name, String::from("padx"));
    assert_eq!(test.weight, 45);
    assert_eq!(
        test.above,
        vec![
            String::from("pbga"),
            String::from("havc"),
            String::from("qoyq")
        ]
    );
    assert_eq!(test.above_weight, 0);
}

#[test]
fn parse_program_exp4() {
    let test = Program::new("tknk (41) -> ugml, padx, fwft");
    assert_eq!(test.name, String::from("tknk"));
    assert_eq!(test.weight, 41);
    assert_eq!(
        test.above,
        vec![
            String::from("ugml"),
            String::from("padx"),
            String::from("fwft")
        ]
    );
    assert_eq!(test.above_weight, 0);
}

#[test]
fn parse_program_exp5() {
    let test = Program::new("gyxo (61)");
    assert_eq!(test.name, String::from("gyxo"));
    assert_eq!(test.weight, 61);
    assert_eq!(test.above, Vec::<String>::new());
    assert_eq!(test.above_weight, 0);
}

#[test]
fn parse_program_stack() {
    let test = ProgramStack::new("./data/example_01.txt");
    assert_eq!(test.bottom, String::new());
    assert_eq!(
        test.all,
        HashMap::from([
            (
                String::from("pbga"),
                Program {
                    name: String::from("pbga"),
                    weight: 66,
                    above: Vec::new(),
                    above_weight: 0
                }
            ),
            (
                String::from("xhth"),
                Program {
                    name: String::from("xhth"),
                    weight: 57,
                    above: Vec::new(),
                    above_weight: 0
                }
            ),
            (
                String::from("ebii"),
                Program {
                    name: String::from("ebii"),
                    weight: 61,
                    above: Vec::new(),
                    above_weight: 0
                }
            ),
            (
                String::from("havc"),
                Program {
                    name: String::from("havc"),
                    weight: 66,
                    above: Vec::new(),
                    above_weight: 0
                }
            ),
            (
                String::from("ktlj"),
                Program {
                    name: String::from("ktlj"),
                    weight: 57,
                    above: Vec::new(),
                    above_weight: 0
                }
            ),
            (
                String::from("fwft"),
                Program {
                    name: String::from("fwft"),
                    weight: 72,
                    above: vec![
                        String::from("ktlj"),
                        String::from("cntj"),
                        String::from("xhth")
                    ],
                    above_weight: 0
                }
            ),
            (
                String::from("qoyq"),
                Program {
                    name: String::from("qoyq"),
                    weight: 66,
                    above: Vec::new(),
                    above_weight: 0
                }
            ),
            (
                String::from("padx"),
                Program {
                    name: String::from("padx"),
                    weight: 45,
                    above: vec![
                        String::from("pbga"),
                        String::from("havc"),
                        String::from("qoyq")
                    ],
                    above_weight: 0
                }
            ),
            (
                String::from("tknk"),
                Program {
                    name: String::from("tknk"),
                    weight: 41,
                    above: vec![
                        String::from("ugml"),
                        String::from("padx"),
                        String::from("fwft")
                    ],
                    above_weight: 0
                }
            ),
            (
                String::from("jptl"),
                Program {
                    name: String::from("jptl"),
                    weight: 61,
                    above: Vec::new(),
                    above_weight: 0
                }
            ),
            (
                String::from("ugml"),
                Program {
                    name: String::from("ugml"),
                    weight: 68,
                    above: vec![
                        String::from("gyxo"),
                        String::from("ebii"),
                        String::from("jptl]")
                    ],
                    above_weight: 0
                }
            ),
            (
                String::from("gyxo"),
                Program {
                    name: String::from("gyxo"),
                    weight: 61,
                    above: Vec::new(),
                    above_weight: 0
                }
            ),
            (
                String::from("cntj"),
                Program {
                    name: String::from("cntj"),
                    weight: 57,
                    above: Vec::new(),
                    above_weight: 0
                }
            ),
        ])
    );
}

#[test]
fn find_bottom() {
    let mut test = ProgramStack::new("./data/example_01.txt");
    assert_eq!(test.bottom, String::from("tknk"));
}
