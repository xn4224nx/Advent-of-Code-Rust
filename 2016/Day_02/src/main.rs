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
 * PART 1:  Your puzzle input is the instructions from the document you found at
 *          the front desk. What is the bathroom code?
 */

#[derive(PartialEq, Debug)]
pub enum Direc {
    Up,
    Down,
    Left,
    Right,
}

pub struct KeyPad {
    pub vals: Vec<Vec<char>>,
    pub pos: (usize, usize),
    pub directs: Vec<Vec<Direc>>,
}

impl KeyPad {
    pub fn new(grid: Vec<Vec<char>>, start_pnt: (usize, usize)) -> Self {
        Self {
            vals: grid,
            pos: start_pnt,
            directs: Vec::new(),
        }
    }

    pub fn read_keypad_commands(&mut self, file_path: &str) {}

    pub fn move_position(&mut self, curr_direct: Direc) {}

    pub fn find_access_code(&mut self) -> String {
        String::from("")
    }
}

fn main() {}
