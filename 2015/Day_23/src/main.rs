/*
 * --- Day 23: Opening the Turing Lock ---
 *
 * Little Jane Marie just got her very first computer for Christmas from some
 * unknown benefactor. It comes with instructions and an example program, but
 * the computer itself seems to be malfunctioning. She's curious what the
 * program does, and would like you to help her run it.
 *
 * The manual explains that the computer supports two registers and six
 * instructions (truly, it goes on to remind the reader, a state-of-the-art
 * technology). The registers are named a and b, can hold any non-negative
 * integer, and begin with a value of 0. The instructions are as follows:
 *
 *      -   hlf r sets register r to half its current value, then continues with
 *          the next instruction.
 *
 *      -   tpl r sets register r to triple its current value, then continues
 *          with the next instruction.
 *
 *      -   inc r increments register r, adding 1 to it, then continues with the
 *          next instruction.
 *
 *      -   jmp offset is a jump; it continues with the instruction offset away
 *          relative to itself.
 *
 *      -   jie r, offset is like jmp, but only jumps if register r is even
 *          ("jump if even").
 *
 *      -   jio r, offset is like jmp, but only jumps if register r is 1 ("jump
 *          if one", not odd).
 *
 * All three jump instructions work with an offset relative to that instruction.
 * The offset is always written with a prefix + or - to indicate the direction
 * of the jump (forward or backward, respectively). For example, jmp +1 would
 * simply continue with the next instruction, while jmp +0 would continuously
 * jump back to itself forever.
 *
 * The program exits when it tries to run an instruction beyond the ones
 * defined.
 *
 * PART 1:  What is the value in register b when the program in your puzzle
 *          input is finished executing?
 */

#[derive(Debug, PartialEq)]
pub enum Comm {
    Half(char),
    Triple(char),
    Increm(char),
    Jump(i32),
    JumpIfEven((char, i32)),
    JumpIfOne((char, i32)),
}

impl Comm {
    pub fn new(raw_instruction: String) -> Self {
        Comm::Half('a')
    }
}

pub struct Computer {
    pub reg_a: u32,
    pub reg_b: u32,
    pub ptr: usize,
    pub coms: Vec<Comm>,
}

impl Computer {
    pub fn new(value_of_a: u32, value_of_b: u32) -> Self {
        Self {
            reg_a: value_of_a,
            reg_b: value_of_b,
            ptr: 0,
            coms: Vec::new(),
        }
    }

    pub fn execute_comms(&mut self) {}

    pub fn read_comms(&mut self, file_path: &str) {}
}

fn main() {}
