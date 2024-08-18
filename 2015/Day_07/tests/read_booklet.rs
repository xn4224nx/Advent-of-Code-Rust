#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_07;
use day_07::{read_booklet, Instruction, Operation, Signal};

#[test]
fn read_exp_01() {
    assert_eq!(
        read_booklet("./data/example_01.txt", None),
        vec![
            Instruction {
                output: Signal::Wire("x".to_string()),
                opper: Operation::Assign,
                inputs: vec![Signal::Num(123)]
            },
            Instruction {
                output: Signal::Wire("y".to_string()),
                opper: Operation::Assign,
                inputs: vec![Signal::Num(456)]
            },
            Instruction {
                output: Signal::Wire("d".to_string()),
                opper: Operation::And,
                inputs: vec![Signal::Wire("y".to_string()), Signal::Wire("x".to_string())]
            },
            Instruction {
                output: Signal::Wire("e".to_string()),
                opper: Operation::Or,
                inputs: vec![Signal::Wire("y".to_string()), Signal::Wire("x".to_string())]
            },
            Instruction {
                output: Signal::Wire("f".to_string()),
                opper: Operation::LShift,
                inputs: vec![Signal::Num(2), Signal::Wire("x".to_string())]
            },
            Instruction {
                output: Signal::Wire("g".to_string()),
                opper: Operation::RShift,
                inputs: vec![Signal::Num(2), Signal::Wire("y".to_string())]
            },
            Instruction {
                output: Signal::Wire("h".to_string()),
                opper: Operation::Not,
                inputs: vec![Signal::Wire("x".to_string())]
            },
            Instruction {
                output: Signal::Wire("i".to_string()),
                opper: Operation::Not,
                inputs: vec![Signal::Wire("y".to_string())]
            }
        ]
    )
}
