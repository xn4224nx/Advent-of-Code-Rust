/*
 * --- Day 23: Safe Cracking ---
 *
 * This is one of the top floors of the nicest tower in EBHQ. The Easter Bunny's
 * private office is here, complete with a safe hidden behind a painting, and
 * who wouldn't hide a star in a safe behind a painting?
 *
 * The safe has a digital screen and keypad for code entry. A sticky note
 * attached to the safe has a password hint on it: "eggs". The painting is of a
 * large rabbit coloring some eggs. You see 7.
 *
 * When you go to type the code, though, nothing appears on the display;
 * instead, the keypad comes apart in your hands, apparently having been
 * smashed. Behind it is some kind of socket - one that matches a connector in
 * your prototype computer! You pull apart the smashed keypad and extract the
 * logic circuit, plug it into your computer, and plug your computer into the
 * safe.
 *
 * Now, you just need to figure out what output the keypad would have sent to
 * the safe. You extract the assembunny code from the logic chip (your puzzle
 * input).
 *
 * The code looks like it uses almost the same architecture and instruction set
 * that the monorail computer used! You should be able to use the same
 * assembunny interpreter for this as you did there, but with one new
 * instruction:
 *
 * tgl x toggles the instruction x away (pointing at instructions like jnz does:
 * positive means forward; negative means backward):
 *
 *      -   For one-argument instructions, inc becomes dec, and all other one-
 *          argument instructions become inc.
 *
 *      -   For two-argument instructions, jnz becomes cpy, and all other two-
 *          instructions become jnz.
 *
 *      -   The arguments of a toggled instruction are not affected.
 *
 *      -   If an attempt is made to toggle an instruction outside the program,
 *          nothing happens.
 *
 *      -   If toggling produces an invalid instruction (like cpy 1 2) and an
 *          attempt is later made to execute that instruction, skip it instead.
 *
 *      -   If tgl toggles itself (for example, if a is 0, tgl a would target
 *          itself and become inc a), the resulting instruction is not executed
 *          until the next time it is reached.
 *
 * The rest of the electronics seem to place the keypad entry (the number of
 * eggs, 7) in register a, run the code, and then send the value left in
 * register a to the safe.
 *
 * PART 1:  What value should be sent to the safe?
 *
 * The safe doesn't open, but it does make several angry noises to express its
 * frustration.
 *
 * You're quite sure your logic is working correctly, so the only other thing
 * is... you check the painting again. As it turns out, colored eggs are still
 * eggs. Now you count 12.
 *
 * As you run the program with this new input, the prototype computer begins to
 * overheat. You wonder what's taking so long, and whether the lack of any
 * instruction more powerful than "add one" has anything to do with it. Don't
 * bunnies usually multiply?
 *
 * PART 2:  Anyway, what value should actually be sent to the safe?
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
    JumpVal(usize, i32),
    JumpReg(i32, usize),
    Toggle(usize),
}

pub struct Computer {
    pub register: Vec<i32>,
    pub instructs: Vec<Command>,
    pub toggled: Vec<bool>,
    pub curr_instruc: usize,
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
            Regex::new(r"jnz ([a-d]) (\-?\d+)").unwrap(),
            Regex::new(r"jnz (\-?\d+) ([a-d])").unwrap(),
            Regex::new(r"tgl ([a-d])").unwrap(),
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
                            convert_char_to_idx(caps.get(1).unwrap().as_str()),
                            caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                        ),
                        5 => Command::JumpReg(
                            caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                            convert_char_to_idx(caps.get(2).unwrap().as_str()),
                        ),
                        6 => Command::Toggle(convert_char_to_idx(caps.get(1).unwrap().as_str())),
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
            toggled: vec![false; num_instructs],
            curr_instruc: 0,
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
        match self.instructs[instruc_idx] {
            Command::CopyVal(val, reg_idx) => {
                if self.toggled[instruc_idx] {
                    if val != 0 {
                        self.modify_instruc_idx(self.register[reg_idx]);
                    } else {
                        self.curr_instruc += 1;
                    }
                } else {
                    self.register[reg_idx] = val;
                    self.curr_instruc += 1;
                }
            }
            Command::CopyReg(reg_idx_scr, reg_idx_dest) => {
                if self.toggled[instruc_idx] {
                    if self.register[reg_idx_scr] != 0 {
                        self.modify_instruc_idx(self.register[reg_idx_dest]);
                    } else {
                        self.curr_instruc += 1;
                    }
                } else {
                    self.register[reg_idx_dest] = self.register[reg_idx_scr];
                    self.curr_instruc += 1;
                }
            }
            Command::Inc(reg_idx) => {
                if self.toggled[instruc_idx] {
                    self.register[reg_idx] -= 1;
                } else {
                    self.register[reg_idx] += 1;
                };
                self.curr_instruc += 1;
            }
            Command::Dec(reg_idx) => {
                if self.toggled[instruc_idx] {
                    self.register[reg_idx] += 1;
                } else {
                    self.register[reg_idx] -= 1;
                };
                self.curr_instruc += 1;
            }
            Command::JumpVal(reg_idx, mov_val) => {
                if self.toggled[instruc_idx] {
                    /* This inverted instruction is meaningless. */
                    self.curr_instruc += 1;
                } else {
                    if self.register[reg_idx] != 0 {
                        self.modify_instruc_idx(mov_val);
                    } else {
                        self.curr_instruc += 1;
                    }
                };
            }
            Command::JumpReg(test_val, reg_idx) => {
                if self.toggled[instruc_idx] {
                    self.register[reg_idx] = test_val;
                    self.curr_instruc += 1;
                } else {
                    if test_val != 0 {
                        self.modify_instruc_idx(self.register[reg_idx]);
                    } else {
                        self.curr_instruc += 1;
                    }
                };
            }
            Command::Toggle(reg_idx) => {
                if self.toggled[instruc_idx] {
                    self.register[reg_idx] += 1;
                } else {
                    let flip_idx = self.curr_instruc as i32 + self.register[reg_idx];

                    /* Check the flip is valid. */
                    if flip_idx >= 0 && flip_idx < self.toggled.len() as i32 {
                        let f_idx = flip_idx as usize;
                        self.toggled[f_idx] = !self.toggled[f_idx];
                    }
                };
                self.curr_instruc += 1;
            }
        }
    }

    /// Find the final value of the a register after all instructions
    /// have completed.
    pub fn crack_safe(&mut self, init_a_reg: i32) -> i32 {
        self.register[0] = init_a_reg;

        /* Execute instructions until the index goes out of bounds. */
        while self.curr_instruc < self.instructs.len() {
            self.execute_instruct(self.curr_instruc);
        }
        return self.register[0];
    }
}

fn main() {
    println!(
        "Part 1 = {}",
        Computer::new("./data/input.txt").crack_safe(7)
    );
    println!(
        "Part 2 = {}",
        Computer::new("./data/input.txt").crack_safe(12)
    );
}
