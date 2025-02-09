/*
 * --- Day 25: Clock Signal ---
 *
 * You open the door and find yourself on the roof. The city sprawls away from
 * you for miles and miles.
 *
 * There's not much time now - it's already Christmas, but you're nowhere near
 * the North Pole, much too far to deliver these stars to the sleigh in time.
 *
 * However, maybe the huge antenna up here can offer a solution. After all, the
 * sleigh doesn't need the stars, exactly; it needs the timing data they
 * provide, and you happen to have a massive signal generator right here.
 *
 * You connect the stars you have to your prototype computer, connect that to
 * the antenna, and begin the transmission.
 *
 * Nothing happens.
 *
 * You call the service number printed on the side of the antenna and quickly
 * explain the situation. "I'm not sure what kind of equipment you have
 * connected over there," he says, "but you need a clock signal." You try to
 * explain that this is a signal for a clock.
 *
 * "No, no, a clock signal - timing information so the antenna computer knows
 * how to read the data you're sending it. An endless, alternating pattern of 0,
 * 1, 0, 1, 0, 1, 0, 1, 0, 1...." He trails off.
 *
 * You ask if the antenna can handle a clock signal at the frequency you would
 * need to use for the data from the stars. "There's no way it can! The only
 * antenna we've installed capable of that is on top of a top-secret Easter
 * Bunny installation, and you're definitely not-" You hang up the phone.
 *
 * You've extracted the antenna's clock signal generation assembunny code (your
 * puzzle input); it looks mostly compatible with code you worked on just
 * recently.
 *
 * This antenna code, being a signal generator, uses one extra instruction:
 *
 *      -   out x transmits x (either an integer or the value of a register) as
 *          the next value for the clock signal.
 *
 * The code takes a value (via register a) that describes the signal to
 * generate, but you're not sure how it's used. You'll have to find the input to
 * produce the right signal through experimentation.
 *
 * PART 1:  What is the lowest positive integer that can be used to initialize
 *          register a and cause the code to output a clock signal of 0, 1, 0,
 *          1... repeating forever?
 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub enum Command {
    CopyVal(i32, usize),
    CopyReg(usize, usize),
    Inc(usize),
    Dec(usize),
    JumpVal(i32, i32),
    JumpReg(usize, i32),
    Out(usize),
}

pub struct Computer {
    pub register: Vec<i32>,
    pub instructs: Vec<Command>,
    pub curr_instruc: usize,
    pub display: Vec<i32>,
}

/// Convert a char to the index it has in the alphabet
pub fn convert_char_to_idx(letter: &str) -> usize {
    return letter.chars().next().unwrap() as usize - 'a' as usize;
}

impl Computer {
    pub fn new(instruc_file: &str) -> Self {
        let mut buffer = String::new();
        let mut instructs = Vec::new();

        /* Open the file */
        let file = File::open(instruc_file).unwrap();
        let mut fp = BufReader::new(file);

        let re_pats = vec![
            Regex::new(r"cpy (\-?\d+) ([a-d])").unwrap(),
            Regex::new(r"cpy ([a-d]) ([a-d])").unwrap(),
            Regex::new(r"inc ([a-d])").unwrap(),
            Regex::new(r"dec ([a-d])").unwrap(),
            Regex::new(r"jnz (\-?\d+) (\-?\d+)").unwrap(),
            Regex::new(r"jnz ([a-d]) (\-?\d+)").unwrap(),
            Regex::new(r"out ([a-d])").unwrap(),
        ];

        /* Iterate over it line by line. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            for (idx, pattern) in re_pats.iter().enumerate() {
                if let Some(caps) = pattern.captures(&buffer) {
                    instructs.push(match idx {
                        0 => Command::CopyVal(
                            caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                            convert_char_to_idx(caps.get(2).unwrap().as_str()),
                        ),
                        1 => Command::CopyReg(
                            convert_char_to_idx(caps.get(1).unwrap().as_str()),
                            convert_char_to_idx(caps.get(2).unwrap().as_str()),
                        ),
                        2 => Command::Inc(convert_char_to_idx(caps.get(1).unwrap().as_str())),
                        3 => Command::Dec(convert_char_to_idx(caps.get(1).unwrap().as_str())),
                        4 => Command::JumpVal(
                            caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                            caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                        ),
                        5 => Command::JumpReg(
                            convert_char_to_idx(caps.get(1).unwrap().as_str()),
                            caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                        ),
                        6 => Command::Out(convert_char_to_idx(caps.get(1).unwrap().as_str())),
                        _ => panic!("Unknown command index encountered!"),
                    });

                    /* After the first regex pattern that matches stop looking. */
                    break;
                }
            }
            buffer.clear();
        }

        let num_instructs: usize = instructs.len();
        return Computer {
            register: vec![0; 4],
            instructs,
            curr_instruc: 0,
            display: Vec::new(),
        };
    }

    /// Change the instruction index
    pub fn modify_instruc_idx(&mut self, modifying_val: i32) {
        let mag_val = modifying_val.abs() as usize;

        if modifying_val >= 0 {
            self.curr_instruc += mag_val;
        } else if mag_val <= self.curr_instruc {
            self.curr_instruc -= mag_val;

        /* If the instruction would be invalid set as a particular value. */
        } else {
            self.curr_instruc = usize::MAX;
        }
    }

    /// Fully execute the command at the specified index
    pub fn execute_instruct(&mut self, instruc_idx: usize) {
        let mut jmp_occured = false;
        match self.instructs[instruc_idx] {
            Command::CopyVal(val, reg_idx) => {
                self.register[reg_idx] = val;
            }
            Command::CopyReg(reg_idx_scr, reg_idx_dest) => {
                self.register[reg_idx_dest] = self.register[reg_idx_scr];
            }
            Command::Inc(reg_idx) => {
                self.register[reg_idx] += 1;
            }
            Command::Dec(reg_idx) => {
                self.register[reg_idx] -= 1;
            }
            Command::JumpVal(test_val, mov_val) => {
                if test_val != 0 {
                    self.modify_instruc_idx(mov_val);
                    jmp_occured = true;
                }
            }
            Command::JumpReg(reg_idx, mov_val) => {
                if self.register[reg_idx] != 0 {
                    self.modify_instruc_idx(mov_val);
                    jmp_occured = true;
                }
            }
            Command::Out(reg_idx) => {
                self.display.push(self.register[reg_idx]);
            }
        };
        if !jmp_occured {
            self.curr_instruc += 1;
        };
    }

    /// Determine if the specified number of outputs match the target output
    pub fn verify_iter_signal(&mut self, inital_a: i32) -> bool {
        self.register = vec![inital_a, 0, 0, 0];
        self.curr_instruc = 0;
        self.display = Vec::new();

        /* Generate values until a discrepency is found or the max ammount is created. */
        while self.display.len() < 100 {
            let curr_display_len = self.display.len();
            self.execute_instruct(self.curr_instruc);

            /* Check if a new digit has been generated. */
            if self.display.len() > curr_display_len {
                let new_digit = self.display[self.display.len() - 1] as usize;

                /* Ensure only valid digits are created. */
                if new_digit != 0 && new_digit != 1 {
                    return false;
                }

                /* Check its the right one for the position. */
                if self.display.len() % 2 == new_digit {
                    return false;
                }
            }
        }
        return true;
    }

    /// Find the lowest initial value of the a register that causes an
    /// alternating pattern of ones and zeros.
    pub fn find_low_pattern_starter(&mut self) -> i32 {
        let mut a_init_val = 0;

        loop {
            if self.verify_iter_signal(a_init_val) {
                return a_init_val;
            };
            a_init_val += 1;
        }
    }
}

fn main() {
    println!(
        "Part 1 = {}",
        Computer::new("./data/input.txt").find_low_pattern_starter()
    );
}
