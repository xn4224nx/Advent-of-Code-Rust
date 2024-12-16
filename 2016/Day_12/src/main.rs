/*
 * --- Day 12: Leonardo's Monorail ---
 *
 * You finally reach the top floor of this building: a garden with a slanted
 * glass ceiling. Looks like there are no more stars to be had.
 *
 * While sitting on a nearby bench amidst some tiger lilies, you manage to
 * Decrypt some of the files you extracted from the servers downstairs.
 *
 * According to these documents, Easter Bunny HQ isn't just this building - it's
 * a collection of buildings in the nearby area. They're all connected by a
 * local monorail, and there's another building not far from here!
 * Unfortunately, being night, the monorail is currently not operating.
 *
 * You remotely connect to the monorail control systems and discover that the
 * boot sequence expects a password. The password-checking logic (your puzzle
 * input) is easy to extract, but the code it uses is strange: it's assembunny
 * code designed for the new computer you just assembled. You'll have to execute
 * the code and get the password.
 *
 * The assembunny code you've extracted operates on four registers (a, b, c, and
 * d) that start at 0 and can hold any integer. However, it seems to make use of
 * only a few instructions:
 *
 *      -   cpy x y copies x (either an integer or the value of a register) into
 *          register y.
 *
 *      -   inc x Increases the value of register x by one.
 *
 *      -   dec x Decreases the value of register x by one.
 *
 *      -   jnz x y jumps to an instruction y away (positive means forward;
 *          negative means backward), but only if x is not zero.
 *
 * The jnz instruction moves relative to itself: an offset of -1 would continue
 * at the previous instruction, while an offset of 2 would skip over the next
 * instruction.
 *
 * PART 1:  After executing the assembunny code in your puzzle input, what value
 *          is left in register a?
 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub enum Command {
    CopyVal(i32, usize),
    CopyReg(usize, usize),
    Incr(usize),
    Decr(usize),
    JumpVal(i32, i32),
    JumpReg(usize, i32),
}

pub struct Computer {
    pub register: Vec<i32>,
    pub data_file: String,
    pub instructs: Vec<Command>,
    pub instruct_idx: usize,
}

impl Computer {
    pub fn new(datafile: &str) -> Self {
        return Computer {
            register: vec![0; 4],
            data_file: datafile.to_string(),
            instructs: Vec::new(),
            instruct_idx: 0,
        };
    }

    pub fn parse_instructs(&mut self) {
        let mut buffer = String::new();
        let idx_offset = 'a' as usize;

        /* Command Regexes */
        let cv_pat = Regex::new(r"cpy (-?[\d]+) ([a-z])").unwrap();
        let cr_pat = Regex::new(r"cpy ([a-z]) ([a-z])").unwrap();
        let in_pat = Regex::new(r"inc ([a-z])").unwrap();
        let de_pat = Regex::new(r"dec ([a-z])").unwrap();
        let jv_pat = Regex::new(r"jnz (-?[\d]+) (-?[\d]+)").unwrap();
        let jr_pat = Regex::new(r"jnz ([a-z]) (-?[\d]+)").unwrap();

        /* Open the file. */
        let file = File::open(&self.data_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Iterate over the file line by line and extract commands. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            if let Some(caps) = cv_pat.captures(&buffer) {
                self.instructs.push(Command::CopyVal(
                    caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    ord_char(caps.get(2).unwrap().as_str()),
                ));
            } else if let Some(caps) = cr_pat.captures(&buffer) {
                self.instructs.push(Command::CopyReg(
                    ord_char(caps.get(1).unwrap().as_str()),
                    ord_char(caps.get(2).unwrap().as_str()),
                ));
            } else if let Some(caps) = in_pat.captures(&buffer) {
                self.instructs
                    .push(Command::Incr(ord_char(caps.get(1).unwrap().as_str())));
            } else if let Some(caps) = de_pat.captures(&buffer) {
                self.instructs
                    .push(Command::Decr(ord_char(caps.get(1).unwrap().as_str())));
            } else if let Some(caps) = jv_pat.captures(&buffer) {
                self.instructs.push(Command::JumpVal(
                    caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                ));
            } else if let Some(caps) = jr_pat.captures(&buffer) {
                self.instructs.push(Command::JumpReg(
                    ord_char(caps.get(1).unwrap().as_str()),
                    caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                ));
            }
            buffer.clear()
        }
    }

    pub fn exe_curr_instr(&mut self) {}

    pub fn execute_all(&mut self) {}
}

fn ord_char(number: &str) -> usize {
    return number.chars().next().unwrap() as usize - 'a' as usize;
}

fn main() {}
