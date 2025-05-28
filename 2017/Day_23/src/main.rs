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
 */

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
}

pub struct CPU {
    pub register: Vec<i32>,
    pub instructions: Vec<Command>,
    pub instruc_idx: usize,
}

impl Command {
    pub fn new(raw_command: &str) -> Self {
        Command::SetReg(0, 0)
    }
}

impl CPU {
    pub fn new(command_file: &str) -> Self {
        CPU {
            register: Vec::new(),
            instructions: Vec::new(),
            instruc_idx: 0,
        }
    }

    /// Run the command pointed to by the index
    pub fn execute_instruc(&mut self) {}

    /// Run the commands to the end and count the number of times the command
    /// mul was used.
    pub fn run_all(&mut self) -> usize {
        0
    }
}

fn main() {}
