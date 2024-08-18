/*
 * --- Day 7: Some Assembly Required ---
 *
 * This year, Santa brought little Bobby Tables a set of wires and
 * bitwise logic gates! Unfortunately, little Bobby is a little under the
 * recommended age range, and he needs help assembling the circuit.
 *
 * Each wire has an identifier (some lowercase letters) and can carry a
 * 16-bit signal (a number from 0 to 65535). A signal is provided to each
 * wire by a gate, another wire, or some specific value. Each wire can
 * only get a signal from one source, but can provide its signal to
 * multiple destinations. A gate provides no signal until all of its
 * inputs have a signal.
 *
 * The included instructions booklet describes how to connect the parts
 * together: x AND y -> z means to connect wires x and y to an AND gate,
 * and then connect its output to wire z.
 *
 * Other possible gates include OR (bitwise OR) and RSHIFT (right-shift).
 * If, for some reason, you'd like to emulate the circuit instead, almost
 * all programming languages (for example, C, JavaScript, or Python)
 * provide operators for these gates.
 *
 * PART 1:  In little Bobby's kit's instructions booklet (provided as
 *          your puzzle input), what signal is ultimately provided to
 *          wire a?
 *
 * Now, take the signal you got on wire a, override wire b to that signal,
 * and reset the other wires (including wire a).
 *
 * PART 1:  What new signal is ultimately provided to wire a?
 */

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Operation {
    Assign,
    And,
    Or,
    LShift,
    RShift,
    Not,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Signal {
    Wire(String),
    Num(u16),
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Instruction {
    pub output: Signal,
    pub opper: Operation,
    pub inputs: Vec<Signal>,
}

/// Convert the raw string to a structured type
pub fn parse_wire_id(raw: String) -> Signal {
    return if raw.chars().all(|x| x.is_digit(10)) {
        Signal::Num(raw.parse::<u16>().unwrap())
    } else {
        Signal::Wire(raw)
    };
}

/// Parse the booklet data on disk and convert to a machine readable
/// format
pub fn read_booklet(file_path: &str, b_val: Option<u16>) -> Vec<Instruction> {
    let file = File::open(file_path).unwrap();
    let mut fp = BufReader::new(file);

    let mut data = Vec::new();
    let mut buffer = String::new();

    /* Read the file line by line. */
    while fp.read_line(&mut buffer).unwrap() > 0 {
        let mut parts: Vec<String> = buffer.split_whitespace().map(|x| x.to_string()).collect();
        let mut inputs: Vec<Signal> = Vec::new();
        let oper;

        /* Determine the output for the instruction. */
        let outp = parse_wire_id(parts.pop().unwrap());

        /* Remove "->" */
        parts.pop().unwrap();

        /* Determine the command and inputs for this line. */
        if buffer.contains("AND") {
            oper = Operation::And;
            inputs.push(parse_wire_id(parts.pop().unwrap()));
            parts.pop().unwrap();
            inputs.push(parse_wire_id(parts.pop().unwrap()));
        } else if buffer.contains("OR") {
            oper = Operation::Or;
            inputs.push(parse_wire_id(parts.pop().unwrap()));
            parts.pop().unwrap();
            inputs.push(parse_wire_id(parts.pop().unwrap()));
        } else if buffer.contains("LSHIFT") {
            oper = Operation::LShift;
            inputs.push(parse_wire_id(parts.pop().unwrap()));
            parts.pop().unwrap();
            inputs.push(parse_wire_id(parts.pop().unwrap()));
        } else if buffer.contains("RSHIFT") {
            oper = Operation::RShift;
            inputs.push(parse_wire_id(parts.pop().unwrap()));
            parts.pop().unwrap();
            inputs.push(parse_wire_id(parts.pop().unwrap()));
        } else if buffer.contains("NOT") {
            oper = Operation::Not;
            inputs.push(parse_wire_id(parts.pop().unwrap()));
        } else {
            oper = Operation::Assign;
            inputs.push(parse_wire_id(parts.pop().unwrap()));
        };

        /* Overwrite the input to the b wire */
        if outp == Signal::Wire(String::from("b")) {
            match b_val {
                Some(val) => {
                    inputs.clear();
                    inputs.push(Signal::Num(val));
                }
                None => (),
            }
        };

        data.push(Instruction {
            output: outp,
            opper: oper,
            inputs: inputs,
        });

        buffer.clear();
    }

    return data;
}

/// Work out what the value of a Signal either as a value or a value
/// from a HashMap.
pub fn resolve_value(raw: &Signal, lookup: &HashMap<String, u16>) -> u16 {
    return match raw {
        Signal::Num(val) => *val,
        Signal::Wire(val) => *lookup.get(val).unwrap(),
    };
}

/// Go through the instructions and finally determine the final
/// signal passed to the a wire
pub fn final_signals(all_inst: &Vec<Instruction>) -> HashMap<String, u16> {
    let mut wire_sigs = HashMap::new();
    let mut unused_idxs = vec![true; all_inst.len()];

    while unused_idxs.contains(&true) {
        /* Try and execute every unused instruction. */
        'rev_instr: for (idx, inst) in all_inst.iter().enumerate() {
            if !unused_idxs[idx] {
                continue 'rev_instr;
            }

            /* Confirm that all the inputs exist. */
            for input in inst.inputs.iter() {
                match input {
                    Signal::Num(_val) => continue,
                    Signal::Wire(val) => {
                        if !wire_sigs.contains_key(val) {
                            continue 'rev_instr;
                        }
                    }
                }
            }

            /* Determine the new signal. */
            let sig = match inst.opper {
                Operation::And => {
                    resolve_value(&inst.inputs[0], &wire_sigs)
                        & resolve_value(&inst.inputs[1], &wire_sigs)
                }
                Operation::Or => {
                    resolve_value(&inst.inputs[0], &wire_sigs)
                        | resolve_value(&inst.inputs[1], &wire_sigs)
                }
                Operation::LShift => {
                    resolve_value(&inst.inputs[1], &wire_sigs)
                        .overflowing_shl(resolve_value(&inst.inputs[0], &wire_sigs) as u32)
                        .0
                }
                Operation::RShift => {
                    resolve_value(&inst.inputs[1], &wire_sigs)
                        .overflowing_shr(resolve_value(&inst.inputs[0], &wire_sigs) as u32)
                        .0
                }
                Operation::Not => !resolve_value(&inst.inputs[0], &wire_sigs),
                Operation::Assign => resolve_value(&inst.inputs[0], &wire_sigs),
            };

            /* Set the output wire to the new signal. */
            match &inst.output {
                Signal::Wire(val) => wire_sigs.insert(val.clone(), sig),
                Signal::Num(_val) => panic!("Outputs cannot be numbers!"),
            };

            unused_idxs[idx] = false;
        }
    }

    return wire_sigs;
}

fn main() {
    let data = read_booklet("./data/input.txt", None);
    let sigs = final_signals(&data);
    let a_sig = resolve_value(&Signal::Wire("a".to_string()), &sigs);

    println!("Part 1 = {}", a_sig);

    let data2 = read_booklet("./data/input.txt", Some(a_sig));
    let sigs2 = final_signals(&data2);
    let a_sig_2 = resolve_value(&Signal::Wire("a".to_string()), &sigs2);

    println!("Part 2 = {}", a_sig_2);
}
