/*
 * --- Day 18: Duet ---
 *
 * You discover a tablet containing some strange assembly code labeled simply
 * "Duet". Rather than bother the sound card with it, you decide to run the
 * code yourself. Unfortunately, you don't see any documentation, so you're
 * left to figure out what the instructions mean on your own.
 *
 * It seems like the assembly is meant to operate on a set of registers that
 * are each named with a single letter and that can each hold a single integer.
 * You suppose each register should start with a value of 0.
 *
 * There aren't that many instructions, so it shouldn't be hard to figure out
 * what they do. Here's what you determine:
 *
 *      -   snd X plays a sound with a frequency equal to the value of X.
 *
 *      -   set X Y sets register X to the value of Y.
 *
 *      -   add X Y increases register X by the value of Y.
 *
 *      -   mul X Y sets register X to the result of multiplying the value
 *          contained in register X by the value of Y.
 *
 *      -   mod X Y sets register X to the remainder of dividing the value
 *          contained in register X by the value of Y (that is, it sets X to
 *          the result of X modulo Y).
 *
 *      -   rcv X recovers the frequency of the last sound played, but only
 *          when the value of X is not zero. (If it is zero, the command does
 *          nothing.)
 *
 *      -   jgz X Y jumps with an offset of the value of Y, but only if the
 *          value of X is greater than zero. (An offset of 2 skips the next
 *          instruction, an offset of -1 jumps to the previous instruction, and
 *          so on.)
 *
 * Many of the instructions can take either a register (a single letter) or a
 * number. The value of a register is the integer it contains; the value of a
 * number is that number.
 *
 * After each jump instruction, the program continues with the instruction to
 * which the jump jumped. After any other instruction, the program continues
 * with the next instruction. Continuing (or jumping) off either end of the
 * program terminates it.
 *
 * For example:
 *
 *      set a 1
 *      add a 2
 *      mul a a
 *      mod a 5
 *      snd a
 *      set a 0
 *      rcv a
 *      jgz a -1
 *      set a 1
 *      jgz a -2
 *
 *      -   The first four instructions set a to 1, add 2 to it, square it, and
 *          then set it to itself modulo 5, resulting in a value of 4.
 *
 *      -   Then, a sound with frequency 4 (the value of a) is played.
 *
 *      -   After that, a is set to 0, causing the subsequent rcv and jgz
 *          instructions to both be skipped (rcv because a is 0, and jgz
 *          because a is not greater than 0).
 *
 *      -   Finally, a is set to 1, causing the next jgz instruction to
 *          activate, jumping back two instructions to another jump, which
 *          jumps again to the rcv, which ultimately triggers the recover
 *          operation.
 *
 * At the time the recover operation is executed, the frequency of the last
 * sound played is 4.
 *
 * PART 1:  At the time the recover operation is executed, the frequency of the
 *          last sound played is 4.
 */

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub enum Cmd {
    SendVal(i64),
    SendReg(usize),
    SetVal(usize, i64),
    SetReg(usize, usize),
    AddVal(usize, i64),
    AddReg(usize, usize),
    MulVal(usize, i64),
    MulReg(usize, usize),
    ModVal(usize, i64),
    ModReg(usize, usize),
    RcvVal(i64),
    RcvReg(usize),
    JgzRegVal(usize, i64),
    JgzValVal(i64, i64),
    JgzRegReg(usize, usize),
    JgzValReg(i64, usize),
}

pub struct Duo {
    pub instructions: Vec<Cmd>,
    pub register: Vec<i64>,
    pub buffer0: Vec<i64>,
    pub buffer1: Vec<i64>,
    pub cmd_idx: usize,
}

impl Duo {
    pub fn new(datafile: &str) -> Self {
        let mut instructions = Vec::new();
        let mut max_reg = 0;

        /* Open the file. */
        let file = File::open(datafile).unwrap();
        let mut fp = BufReader::new(file);

        /* Iterate over the file line by line. */
        let mut buf = String::new();
        while fp.read_line(&mut buf).unwrap() > 0 {
            let (val0, val1);
            let (reg0, reg1);

            /* Split the parts of the command. */
            let rw_cmd: Vec<&str> = buf.split_whitespace().collect();

            /* Extract the first identifier. */
            if rw_cmd[1].parse::<i64>().is_ok() {
                val0 = Some(rw_cmd[1].parse::<i64>().unwrap());
                reg0 = None;
            } else {
                val0 = None;
                reg0 = Some(rw_cmd[1].chars().next().unwrap() as usize - 'a' as usize);
            }

            /* If there is a second identifier, parse it. */
            if rw_cmd.len() == 3 && rw_cmd[2].parse::<i64>().is_ok() {
                val1 = Some(rw_cmd[2].parse::<i64>().unwrap());
                reg1 = None;
            } else if rw_cmd.len() == 3 {
                val1 = None;
                reg1 = Some(rw_cmd[2].chars().next().unwrap() as usize - 'a' as usize);
            } else {
                val1 = None;
                reg1 = None;
            }

            /* Make a record of the highest register index encountered. */
            if reg0.is_some() && reg0.unwrap() > max_reg {
                max_reg = reg0.unwrap()
            }
            if reg1.is_some() && reg1.unwrap() > max_reg {
                max_reg = reg0.unwrap()
            }

            /* Save the full instruction for this line. */
            instructions.push(if rw_cmd[0] == "snd" {
                if val0.is_some() {
                    Cmd::SendVal(val0.unwrap())
                } else {
                    Cmd::SendReg(reg0.unwrap())
                }
            } else if rw_cmd[0] == "set" {
                if val1.is_some() {
                    Cmd::SetVal(reg0.unwrap(), val1.unwrap())
                } else {
                    Cmd::SetReg(reg0.unwrap(), reg1.unwrap())
                }
            } else if rw_cmd[0] == "add" {
                if val1.is_some() {
                    Cmd::AddVal(reg0.unwrap(), val1.unwrap())
                } else {
                    Cmd::AddReg(reg0.unwrap(), reg1.unwrap())
                }
            } else if rw_cmd[0] == "mul" {
                if val1.is_some() {
                    Cmd::MulVal(reg0.unwrap(), val1.unwrap())
                } else {
                    Cmd::MulReg(reg0.unwrap(), reg1.unwrap())
                }
            } else if rw_cmd[0] == "mod" {
                if val1.is_some() {
                    Cmd::ModVal(reg0.unwrap(), val1.unwrap())
                } else {
                    Cmd::ModReg(reg0.unwrap(), reg1.unwrap())
                }
            } else if rw_cmd[0] == "rcv" {
                if val0.is_some() {
                    Cmd::RcvVal(val0.unwrap())
                } else {
                    Cmd::RcvReg(reg0.unwrap())
                }
            } else if rw_cmd[0] == "jgz" {
                if val0.is_some() && val1.is_some() {
                    Cmd::JgzValVal(val0.unwrap(), val1.unwrap())
                } else if reg0.is_some() && reg1.is_some() {
                    Cmd::JgzRegReg(reg0.unwrap(), reg1.unwrap())
                } else if val0.is_some() && reg1.is_some() {
                    Cmd::JgzValReg(val0.unwrap(), reg1.unwrap())
                } else if reg0.is_some() && val1.is_some() {
                    Cmd::JgzRegVal(reg0.unwrap(), val1.unwrap())
                } else {
                    panic!("Unknown jump combination '{}'", buf);
                }
            } else {
                panic!("Unrecognised command '{}'", rw_cmd[0]);
            });
            buf.clear();
        }

        Duo {
            instructions,
            buffer0: Vec::new(),
            buffer1: Vec::new(),
            register: vec![0; max_reg + 1],
            cmd_idx: 0,
        }
    }

    pub fn execute_cmd(&mut self, instr_idx: usize) {
        match self.instructions[instr_idx] {
            Cmd::SendVal(val0) => self.buffer0.push(val0),
            Cmd::SendReg(reg0) => self.buffer0.push(self.register[reg0]),
            Cmd::SetVal(reg0, val1) => self.register[reg0] = val1,
            Cmd::SetReg(reg0, reg1) => self.register[reg0] = self.register[reg1],
            Cmd::AddVal(reg0, val1) => self.register[reg0] += val1,
            Cmd::AddReg(reg0, reg1) => self.register[reg0] += self.register[reg1],
            Cmd::MulVal(reg0, val1) => self.register[reg0] *= val1,
            Cmd::MulReg(reg0, reg1) => self.register[reg0] *= self.register[reg1],
            Cmd::ModVal(reg0, val1) => self.register[reg0] %= val1,
            Cmd::ModReg(reg0, reg1) => self.register[reg0] %= self.register[reg1],
            Cmd::RcvVal(val) => {
                if val != 0 && self.buffer0.len() > 0 {
                    self.buffer1.push(self.buffer0.pop().unwrap());
                }
            }
            Cmd::RcvReg(reg0) => {
                if self.register[reg0] != 0 && self.buffer0.len() > 0 {
                    self.buffer1.push(self.buffer0.pop().unwrap());
                }
            }

            Cmd::JgzRegVal(reg0, val1) => {
                if self.register[reg0] > 0 {
                    self.cmd_idx = self.cmd_idx.overflowing_add((val1 - 1) as usize).0;
                }
            }
            Cmd::JgzValVal(val0, val1) => {
                if val0 > 0 {
                    self.cmd_idx = self.cmd_idx.overflowing_add((val1 - 1) as usize).0;
                }
            }

            Cmd::JgzRegReg(reg0, reg1) => {
                if self.register[reg0] > 0 {
                    self.cmd_idx = self
                        .cmd_idx
                        .overflowing_add((self.register[reg1] - 1) as usize)
                        .0;
                }
            }
            Cmd::JgzValReg(val0, reg1) => {
                if val0 > 0 {
                    self.cmd_idx = self
                        .cmd_idx
                        .overflowing_add((self.register[reg1] - 1) as usize)
                        .0;
                }
            }
        };
        self.cmd_idx += 1;
    }

    pub fn first_recent_sound(&mut self) -> i64 {
        while self.cmd_idx < self.instructions.len() && self.buffer1.len() < 1 {
            self.execute_cmd(self.cmd_idx);
        }
        return self.buffer1[0];
    }
}

fn main() {
    println!(
        "Part 1 = {}",
        Duo::new("./data/input.txt").first_recent_sound()
    );
}
