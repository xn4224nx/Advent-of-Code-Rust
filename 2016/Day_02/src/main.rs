/*
 * --- Day 2: Bathroom Security ---
 *
 * You arrive at Easter Bunny Headquarters under cover of darkness. However, you
 * left in such a rush that you forgot to use the bathroom! Fancy office
 * buildings like this one usually have keypad locks on their bathrooms, so you
 * search the front desk for the code.
 *
 * "In order to improve security," the document you find says, "bathroom codes
 * will no longer be written down. Instead, please memorize and follow the
 * procedure below to access the bathrooms."
 *
 * The document goes on to explain that each button to be pressed can be found
 * by starting on the previous button and moving to adjacent buttons on the
 * keypad: U moves up, D moves down, L moves left, and R moves right. Each line
 * of instructions corresponds to one button, starting at the previous button
 * (or, for the first line, the "5" button); press whatever button you're on at
 * the end of each line. If a move doesn't lead to a button, ignore it.
 *
 * You can't hold it much longer, so you decide to figure out the code as you
 * walk to the bathroom. You picture a keypad like this:
 *
 * 1 2 3
 * 4 5 6
 * 7 8 9
 *
 * PART 1:  Your puzzle input is the instructions from the document you found at
 *          the front desk. What is the bathroom code?
 *
 * You finally arrive at the bathroom (it's a several minute walk from the lobby
 * so visitors can behold the many fancy conference rooms and water coolers on
 * this floor) and go to punch in the code. Much to your bladder's dismay, the
 * keypad is not at all like you imagined it. Instead, you are confronted with
 * the result of hundreds of man-hours of bathroom-keypad-design meetings:
 *
 *     1
 *   2 3 4
 * 5 6 7 8 9
 *   A B C
 *     D
 *
 * You still start at "5" and stop when you're at an edge.
 *
 * PART 2:  Using the same instructions in your puzzle input, what is the
 *          correct bathroom code?
 */

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Debug)]
pub enum Direc {
    Up,
    Down,
    Left,
    Right,
}

pub struct KeyPad {
    pub grid: Vec<Vec<char>>,
    pub pos: (usize, usize),
    pub directs: Vec<Vec<Direc>>,
}

impl KeyPad {
    /// Define the keypad grid
    pub fn new(grid: Vec<Vec<char>>, start_pnt: (usize, usize)) -> Self {
        Self {
            grid: grid,
            pos: start_pnt,
            directs: Vec::new(),
        }
    }

    /// Read the directions on the keypad from file
    pub fn read_keypad_commands(&mut self, file_path: &str) {
        let mut buffer = Vec::new();

        /* Open the file */
        let file = File::open(file_path).unwrap();
        let mut fp = BufReader::new(file);

        /* Read the file line by line. */
        while fp.read_until(b'\n', &mut buffer).unwrap() > 0 {
            let mut line_instr = Vec::new();

            /* For each character in the line */
            for direction in &buffer {
                match direction {
                    85 => line_instr.push(Direc::Up),
                    68 => line_instr.push(Direc::Down),
                    76 => line_instr.push(Direc::Left),
                    82 => line_instr.push(Direc::Right),
                    _ => break,
                }
            }

            self.directs.push(line_instr);
            buffer.clear();
        }
    }

    /// Move the position on the keypad based on a instruction
    pub fn move_position(&mut self, line_idx: usize, direc_idx: usize) {
        let curr_direct = &self.directs[line_idx][direc_idx];
        let old_pos = self.pos;

        /* Verify the move doesn't go beyond the range of the grid. */
        if (self.pos.0 >= self.grid.len() - 1 && curr_direct == &Direc::Down)
            || (self.pos.1 >= self.grid[0].len() - 1 && curr_direct == &Direc::Right)
        {
            return;
        };

        /* Move the pnt. */
        match curr_direct {
            Direc::Up => self.pos = (self.pos.0.saturating_sub(1), self.pos.1),
            Direc::Down => self.pos = (self.pos.0 + 1, self.pos.1),
            Direc::Left => self.pos = (self.pos.0, self.pos.1.saturating_sub(1)),
            Direc::Right => self.pos = (self.pos.0, self.pos.1 + 1),
        };

        /* If the command moves it onto an out of bounds square, revert it. */
        if self.grid[self.pos.0][self.pos.1] == '0' {
            self.pos = old_pos;
        }
    }

    pub fn find_access_code(&mut self) -> String {
        let mut password = String::new();

        /* Go over every instruction line and instruction. */
        for line_idx in 0..self.directs.len() {
            for direc_idx in 0..self.directs[line_idx].len() {
                self.move_position(line_idx, direc_idx);
            }

            /* Record the letter it finally lands on */
            password.push(self.grid[self.pos.0][self.pos.1])
        }
        return password;
    }
}

fn main() {
    let mut normal_pad = KeyPad::new(
        vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ],
        (1, 1),
    );

    normal_pad.read_keypad_commands("./data/input.txt");
    println!("Part 1 = {}", normal_pad.find_access_code());

    let mut star_pad = KeyPad::new(
        vec![
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '2', '3', '4', '0'],
            vec!['5', '6', '7', '8', '9'],
            vec!['0', 'A', 'B', 'C', '0'],
            vec!['0', '0', 'D', '0', '0'],
        ],
        (2, 0),
    );

    star_pad.read_keypad_commands("./data/input.txt");
    println!("Part 2 = {}", star_pad.find_access_code());
}
