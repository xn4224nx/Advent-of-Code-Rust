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
 */

#[derive(Debug, PartialEq)]
pub enum Command {
    CopyVal(i32, usize),
    CopyReg(usize, usize),
    Inc(usize),
    Dec(usize),
    JumpVal(i32, i32),
    JumpReg(usize, i32),
    Toggle(usize),
}

pub struct Computer {
    pub register: Vec<i32>,
    pub instructs: Vec<Command>,
    pub toggled: Vec<bool>,
    pub curr_instruc: usize,
}

impl Computer {
    pub fn new(instruc_file: &str) -> Self {
        Computer {
            register: Vec::new(),
            instructs: Vec::new(),
            toggled: Vec::new(),
            curr_instruc: 0,
        }
    }

    /// Fully execute the command at the specified index
    pub fn execute_instruct(&mut self, instruc_idx: usize) {}

    /// Find the final value of the a register after all instructions
    /// have completed.
    pub fn crack_safe(&mut self, init_a_reg: i32) -> i32 {
        0
    }
}

fn main() {}
