/*
 * --- Day 8: I Heard You Like Registers ---
 *
 * You receive a signal directly from the CPU. Because of your recent assistance
 * with jump instructions, it would like you to compute the result of a series
 * of unusual register instructions.
 *
 * Each instruction consists of several parts: the register to modify, whether
 * to increase or decrease that register's value, the amount by which to
 * increase or decrease it, and a condition. If the condition fails, skip the
 * instruction without modifying the register. The registers all start at 0. The
 * instructions look like this:
 *
 *      b inc 5 if a > 1
 *      a inc 1 if b < 5
 *      c dec -10 if a >= 1
 *      c inc -20 if c == 10
 *
 * These instructions would be processed as follows:
 *
 *      -   Because a starts at 0, it is not greater than 1, and so b is not
 *          modified.
 *
 *      -   a is increased by 1 (to 1) because b is less than 5 (it is 0).
 *
 *      -   c is decreased by -10 (to 10) because a is now greater than or equal
 *          to 1 (it is 1).
 *
 *      -   c is increased by -20 (to -10) because c is equal to 10.
 *
 * After this process, the largest value in any register is 1.
 *
 * You might also encounter <= (less than or equal to) or != (not equal to).
 * However, the CPU doesn't have the bandwidth to tell you what all the
 * registers are named, and leaves that to you to determine.
 *
 * PART 1:  What is the largest value in any register after completing the
 *          instructions in your puzzle input?
 */

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Debug)]
pub enum Comp {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    MoreThan,
    MoreThanOrEqual,
}

#[derive(PartialEq, Debug)]
pub struct Command {
    pub change_reg: String,
    pub change_val: i32,
    pub test_reg: String,
    pub comp_opp: Comp,
    pub test_val: i32,
}

pub struct Computer {
    pub register: HashMap<String, i32>,
    pub instrucs: Vec<Command>,
}

impl Computer {
    pub fn new(data_file: &str) -> Self {
        let mut buffer = String::new();
        let mut instrucs = Vec::new();
        let mut register = HashMap::new();
        let re_instruc =
            Regex::new(r"([a-z]+) (inc|dec) (-?[0-9]+) if ([a-z]+) ([=<>!]+) (-?[0-9]+)").unwrap();

        /* Open the file. */
        let file = File::open(data_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Iterate over the file line by line. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            /* Try and extract the data from the line. */
            let Some(caps) = re_instruc.captures(&buffer) else {
                println!("Line could not be parsed: '{}'", buffer);
                continue;
            };

            /* Parse the individual fields. */
            let change_reg = String::from(&caps[1]);
            let mut change_val = caps[3].parse::<i32>().unwrap();
            let test_reg = String::from(&caps[4]);
            let test_val = caps[6].parse::<i32>().unwrap();

            /* Change the change value based on the command. */
            if &caps[2] == "dec" {
                change_val *= -1;
            };

            /* Fill the register. */
            register.insert(change_reg.clone(), 0);
            register.insert(test_reg.clone(), 0);

            /* Determine the comparison operator. */
            let comp_opp = if &caps[5] == "==" {
                Comp::Equal
            } else if &caps[5] == "!=" {
                Comp::NotEqual
            } else if &caps[5] == "<" {
                Comp::LessThan
            } else if &caps[5] == "<=" {
                Comp::LessThanOrEqual
            } else if &caps[5] == ">" {
                Comp::MoreThan
            } else if &caps[5] == ">=" {
                Comp::MoreThanOrEqual
            } else {
                panic!("Unknown comparison operator: '{}'", &caps[5])
            };

            instrucs.push(Command {
                change_reg,
                change_val,
                test_reg,
                comp_opp,
                test_val,
            });
            buffer.clear();
        }
        return Computer { register, instrucs };
    }

    /// Execute all the instructions and return the largest value in the final
    /// register.
    pub fn largest_value(&mut self) -> i32 {
        /* Sequentially execute the instructions. */
        for idx in 0..self.instrucs.len() {
            match self.instrucs[idx].comp_opp {
                Comp::Equal => {
                    if *self.register.get(&self.instrucs[idx].test_reg).unwrap()
                        == self.instrucs[idx].test_val
                    {
                        *self
                            .register
                            .get_mut(&self.instrucs[idx].change_reg)
                            .unwrap() += self.instrucs[idx].change_val;
                    };
                }
                Comp::NotEqual => {
                    if *self.register.get(&self.instrucs[idx].test_reg).unwrap()
                        != self.instrucs[idx].test_val
                    {
                        *self
                            .register
                            .get_mut(&self.instrucs[idx].change_reg)
                            .unwrap() += self.instrucs[idx].change_val;
                    };
                }
                Comp::LessThan => {
                    if *self.register.get(&self.instrucs[idx].test_reg).unwrap()
                        < self.instrucs[idx].test_val
                    {
                        *self
                            .register
                            .get_mut(&self.instrucs[idx].change_reg)
                            .unwrap() += self.instrucs[idx].change_val;
                    };
                }
                Comp::LessThanOrEqual => {
                    if *self.register.get(&self.instrucs[idx].test_reg).unwrap()
                        <= self.instrucs[idx].test_val
                    {
                        *self
                            .register
                            .get_mut(&self.instrucs[idx].change_reg)
                            .unwrap() += self.instrucs[idx].change_val;
                    };
                }
                Comp::MoreThan => {
                    if *self.register.get(&self.instrucs[idx].test_reg).unwrap()
                        > self.instrucs[idx].test_val
                    {
                        *self
                            .register
                            .get_mut(&self.instrucs[idx].change_reg)
                            .unwrap() += self.instrucs[idx].change_val;
                    };
                }
                Comp::MoreThanOrEqual => {
                    if *self.register.get(&self.instrucs[idx].test_reg).unwrap()
                        >= self.instrucs[idx].test_val
                    {
                        *self
                            .register
                            .get_mut(&self.instrucs[idx].change_reg)
                            .unwrap() += self.instrucs[idx].change_val;
                    };
                }
            }
        }
        /* Return the largest value in the registers. */
        return *self.register.values().max().unwrap();
    }
}

fn main() {
    println!(
        "Part 1 = {}",
        Computer::new("./data/input.txt").largest_value()
    )
}
