/*
 * --- Day 21: Scrambled Letters and Hash ---
 *
 * The computer system you're breaking into uses a weird scrambling function to
 * store its passwords. It shouldn't be much trouble to create your own
 * scrambled password so you can add it to the system; you just have to
 * implement the scrambler.
 *
 * The scrambling function is a series of operations (the exact list is provided
 * in your puzzle input). Starting with the password to be scrambled, apply each
 * operation in succession to the string. The individual operations behave as
 * follows:
 *
 *      -   swap position X with position Y means that the letters at indexes X
 *          and Y (counting from 0) should be swapped.
 *
 *      -   swap letter X with letter Y means that the letters X and Y should be
 *          swapped (regardless of where they appear in the string).
 *
 *      -   rotate left/right X steps means that the whole string should be
 *          rotated; for example, one right rotation would turn abcd into dabc.
 *
 *      -   rotate based on position of letter X means that the whole string
 *          should be rotated to the right based on the index of letter X
 *          (counting from 0) as determined before this instruction does any
 *          rotations. Once the index is determined, rotate the string to the
 *          right one time, plus a number of times equal to that index, plus one
 *          additional time if the index was at least 4.
 *
 *      -   reverse positions X through Y means that the span of letters at
 *          indexes X through Y (including the letters at X and Y) should be
 *          reversed in order.
 *
 *      -   move position X to position Y means that the letter which is at
 *          index X should be removed from the string, then inserted such that
 *          it ends up at index Y.
 *
 *  PART 1: Now, you just need to generate a new scrambled password and you can
 *          access the system. Given the list of scrambling operations in your
 *          puzzle input, what is the result of scrambling abcdefgh?
 */

#[derive(PartialEq, Debug)]
pub enum Command {
    SwapIndex(usize, usize),
    SwapLetter(char, char),
    Rotate(i32),
    RotateLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

pub struct SecretHasher {
    pub initial_letters: Vec<char>,
    pub curr_letters: Vec<char>,
    pub instructs: Vec<Command>,
}

impl SecretHasher {
    pub fn new(starting_letters: &str, instruc_file: &str) -> Self {
        SecretHasher {
            initial_letters: Vec::new(),
            curr_letters: Vec::new(),
            instructs: Vec::new(),
        }
    }

    /// Take a command and mutate the current letters according to the command
    pub fn impl_command(&mut self, comm: &Command) {}

    /// Show the letters in there current state
    pub fn show(&self) -> String {
        "".to_string()
    }

    /// Find the final form of the letters after all commands
    pub fn final_state(&mut self) -> String {
        "".to_string()
    }
}

fn main() {}
