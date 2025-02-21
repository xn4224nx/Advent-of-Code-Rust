/*
 * --- Day 5: A Maze of Twisty Trampolines, All Alike ---
 *
 * An urgent interrupt arrives from the CPU: it's trapped in a maze of jump
 * instructions, and it would like assistance from any programs with spare
 * cycles to help find the exit.
 *
 * The message includes a list of the offsets for each jump. Jumps are
 * relative: -1 moves to the previous instruction, and 2 skips the next one.
 * Start at the first instruction in the list. The goal is to follow the jumps
 * until one leads outside the list.
 *
 * In addition, these instructions are a little strange; after each jump, the
 * offset of that instruction increases by 1. So, if you come across an offset
 * of 3, you would move three instructions forward, but change it to a 4 for
 * the next time it is encountered.
 *
 * For example, consider the following list of jump offsets:
 *
 *      0
 *      3
 *      0
 *      1
 *      -3
 *
 * Positive jumps ("forward") move downward; negative jumps move upward. For
 * legibility in this example, these offset values will be written all on one
 * line, with the current instruction marked in parentheses. The following
 * steps would be taken before an exit is found:
 *
 *      -   (0) 3  0  1  -3  - before we have taken any steps.
 *
 *      -   (1) 3  0  1  -3  - jump with offset 0 (that is, don't jump at all).
 *          Fortunately, the instruction is then incremented to 1.
 *
 *      -    2 (3) 0  1  -3  - step forward because of the instruction we just
 *          modified. The first instruction is incremented again, now to 2.
 *
 *      -    2  4  0  1 (-3) - jump all the way to the end; leave a 4 behind.
 *
 *      -    2 (4) 0  1  -2  - go back to where we just were; increment -3 to
 *          -2.
 *
 *      -    2  5  0  1  -2  - jump 4 steps forward, escaping the maze.
 *
 * In this example, the exit is reached in 5 steps.
 *
 * PART 1:  How many steps does it take to reach the exit?
 */

use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
pub struct Instructions {
    pub jumps: Vec<i32>,
    pub jmp_idx: usize,
}

impl Instructions {
    pub fn new(datafile: &str) -> Self {
        let jumps = read_to_string(datafile)
            .unwrap()
            .split_ascii_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        return Instructions { jumps, jmp_idx: 0 };
    }

    /// Change the jumps based on the current jump index
    pub fn execute_curr(&mut self) {
        let pre_move_idx = self.jmp_idx;
        let move_mag = self.jumps[pre_move_idx] as usize;

        /* Change the jump index based on the jump value. */
        self.jmp_idx = self.jmp_idx.overflowing_add(move_mag).0;

        /* Change the previous value pointed to by the index. */
        self.jumps[pre_move_idx] += 1;
    }

    /// Count the number of steps to exit the jumps
    pub fn steps_to_exit(&mut self) -> u32 {
        let mut step_cnt = 0;

        while self.jmp_idx < self.jumps.len() {
            self.execute_curr();
            step_cnt += 1;
        }
        return step_cnt;
    }
}

fn main() {
    println!(
        "Part 1 = {}",
        Instructions::new("./data/input.txt").steps_to_exit()
    );
}
