/*
 * --- Day 23: Coprocessor Conflagration ---
 *
 * You decide to head directly to the CPU and fix the printer from there. As you
 * get close, you find an experimental coprocessor doing so much work that the
 * local programs are afraid it will halt and catch fire. This would cause
 * serious issues for the rest of the computer, so you head in and see what you
 * can do.
 *
 * The code it's running seems to be a variant of the kind you saw recently on
 * that tablet. The general functionality seems very similar, but some of the
 * instructions are different:
 *
 *      -   set X Y sets register X to the value of Y.
 *
 *      -   sub X Y decreases register X by the value of Y.
 *
 *      -   mul X Y sets register X to the result of multiplying the value
 *          contained in register X by the value of Y.
 *
 *      -   jnz X Y jumps with an offset of the value of Y, but only if the value
 *          of X is not zero. (An offset of 2 skips the next instruction, an
 *          offset of -1 jumps to the previous instruction, and so on.)
 *
 * Only the instructions listed above are used. The eight registers here, named a
 * through h, all start at 0.
 *
 * The coprocessor is currently set to some kind of debug mode, which allows for
 * testing, but prevents it from doing any meaningful work.
 *
 * PART 1:  If you run the program (your puzzle input), how many times is the mul
 *          instruction invoked?
 *
 * Now, it's time to fix the problem.
 *
 * The debug mode switch is wired directly to register a. You flip the switch,
 * which makes register a now start at 1 when the program is executed.
 *
 * Immediately, the coprocessor begins to overheat. Whoever wrote this program
 * obviously didn't choose a very efficient implementation. You'll need to
 * optimize the program if it has any hope of completing before Santa needs that
 * printer working.
 *
 * The coprocessor's ultimate goal is to determine the final value left in
 * register h once the program completes. Technically, if it had that... it
 * wouldn't even need to run the program.
 *
 * PART 2:  After setting register a to 1, if the program were to run to
 *          completion, what value would be left in register h?
 */

use slow_primes::is_prime_miller_rabin;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub enum Command {
    SetVal(usize, i32),
    SubVal(usize, i32),
    MulVal(usize, i32),
    JnzVal(usize, i32),
    SetReg(usize, usize),
    SubReg(usize, usize),
    MulReg(usize, usize),
    JnzReg(usize, usize),
    JnzValVal(i32, i32),
}

pub struct CPU {
    pub register: Vec<i32>,
    pub instructions: Vec<Command>,
    pub instruc_idx: i32,
}

impl Command {
    pub fn new(raw_command: &str) -> Self {
        let mut switch: Vec<bool> = Vec::new();
        let mut values: Vec<i32> = Vec::new();
        let mut registers: Vec<usize> = Vec::new();

        let parts: Vec<&str> = raw_command.trim().split_whitespace().collect();

        /* Parse the values. */
        for idx in 1..3 {
            if parts[idx].parse::<i32>().is_ok() {
                switch.push(false);
                values.push(parts[idx].parse::<i32>().unwrap());
                registers.push(0);
            } else {
                switch.push(true);
                values.push(0);
                registers.push((parts[idx].chars().next().unwrap() as usize) - 97);
            };
        }

        /* Select the right command and return it. */
        return match parts[0] {
            "set" => {
                if switch[1] {
                    Command::SetReg(registers[0], registers[1])
                } else {
                    Command::SetVal(registers[0], values[1])
                }
            }
            "sub" => {
                if switch[1] {
                    Command::SubReg(registers[0], registers[1])
                } else {
                    Command::SubVal(registers[0], values[1])
                }
            }
            "mul" => {
                if switch[1] {
                    Command::MulReg(registers[0], registers[1])
                } else {
                    Command::MulVal(registers[0], values[1])
                }
            }
            "jnz" => {
                if switch[0] && switch[1] {
                    Command::JnzReg(registers[0], registers[1])
                } else if switch[0] && !switch[1] {
                    Command::JnzVal(registers[0], values[1])
                } else {
                    Command::JnzValVal(values[0], values[1])
                }
            }
            _ => panic!("'{}' is not a recognised command!", parts[0]),
        };
    }
}

impl CPU {
    pub fn new(command_file: &str) -> Self {
        let mut instructions = Vec::new();
        let mut buffer = String::new();

        /* Open the file. */
        let file = File::open(command_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Iterate over the file line by line. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            instructions.push(Command::new(&buffer));
            buffer.clear();
        }

        return CPU {
            register: vec![0; 8],
            instructions,
            instruc_idx: 0,
        };
    }

    /// Run the command pointed to by the index
    pub fn execute_instruc(&mut self) {
        match self.instructions[self.instruc_idx as usize] {
            Command::SetVal(reg_a, val) => self.register[reg_a] = val,
            Command::SubVal(reg_a, val) => self.register[reg_a] -= val,
            Command::MulVal(reg_a, val) => self.register[reg_a] *= val,
            Command::JnzVal(reg_a, val) => {
                if self.register[reg_a] != 0 {
                    self.instruc_idx += val - 1;
                }
            }
            Command::SetReg(reg_a, reg_b) => self.register[reg_a] = self.register[reg_b],
            Command::SubReg(reg_a, reg_b) => self.register[reg_a] -= self.register[reg_b],
            Command::MulReg(reg_a, reg_b) => self.register[reg_a] *= self.register[reg_b],
            Command::JnzReg(reg_a, reg_b) => {
                if self.register[reg_a] != 0 {
                    self.instruc_idx += self.register[reg_b] - 1;
                }
            }
            Command::JnzValVal(val_a, val_b) => {
                if val_a != 0 {
                    self.instruc_idx += val_b - 1;
                }
            }
        };
        self.instruc_idx += 1;
    }

    /// Run the commands to the end and count the number of times the command
    /// mul was used.
    pub fn run_all(&mut self) -> usize {
        let mut num_muls = 0;

        /* Run the current instruction till it goes out of range. */
        while self.instruc_idx >= 0 && self.instruc_idx < self.instructions.len() as i32 {
            match self.instructions[self.instruc_idx as usize] {
                Command::MulReg(_, _) | Command::MulVal(_, _) => num_muls += 1,
                _ => {}
            };
            self.execute_instruc();
        }
        return num_muls;
    }

    /// Optimised version of part 2. Find the number of non-primes between two
    /// specified values.
    pub fn final_reg_value(&mut self) -> i32 {
        let mut counter = 0;
        let mut values = Vec::new();

        /* Extract values from the instructions. */
        if let Command::SetVal(_, val) = self.instructions[0] {
            values.push(val);
        }
        if let Command::MulVal(_, val) = self.instructions[4] {
            values.push(val);
        }
        if let Command::SubVal(_, val) = self.instructions[5] {
            values.push(val);
        }
        if let Command::SubVal(_, val) = self.instructions[7] {
            values.push(val);
        }
        if let Command::SubVal(_, val) = self.instructions[self.instructions.len() - 2] {
            values.push(val);
        }

        let first_val = values[0] * values[1] - values[2];
        let last_val = first_val - values[3];

        for value in (first_val..last_val).step_by(-values[4] as usize) {
            if !is_prime_miller_rabin(value as u64) {
                counter += 1;
            }
        }
        return counter;
    }
}

fn main() {
    println!(
        "Part 1 = {}\nPart 2 = {}\n",
        CPU::new("./data/input.txt").run_all(),
        CPU::new("./data/input.txt").final_reg_value()
    );
}
