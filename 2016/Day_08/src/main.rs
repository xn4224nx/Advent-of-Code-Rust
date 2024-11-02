/*
 * --- Day 8: Two-Factor Authentication ---
 *
 * You come across a door implementing what you can only assume is an
 * implementation of two-factor authentication after a long game of requirements
 * telephone.
 *
 * To get past the door, you first swipe a keycard (no problem; there was one on
 * a nearby desk). Then, it displays a code on a little screen, and you type
 * that code on a keypad. Then, presumably, the door unlocks.
 *
 * Unfortunately, the screen has been smashed. After a few minutes, you've taken
 * everything apart and figured out how it works. Now you just have to work out
 * what the screen would have displayed.
 *
 * The magnetic strip on the card you swiped encodes a series of instructions
 * for the screen; these instructions are your puzzle input. The screen is 50
 * pixels wide and 6 pixels tall, all of which start off, and is capable of
 * three somewhat peculiar operations:
 *
 *      -   rect AxB turns on all of the pixels in a rectangle at the top-left
 *          of the screen which is A wide and B tall.
 *
 *      -   rotate row y=A by B shifts all of the pixels in row A (0 is the top
 *          row) right by B pixels. Pixels that would fall off the right end
 *          appear at the left end of the row.
 *
 *      -   rotate column x=A by B shifts all of the pixels in column A (0 is
 *          the left column) down by B pixels. Pixels that would fall off the
 *          bottom appear at the top of the column.
 *
 * As you can see, this display technology is extremely powerful, and will soon
 * dominate the tiny-code-displaying-screen market. That's what the
 * advertisement on the back of the display tries to convince you, anyway.
 *
 * PART 1:  There seems to be an intermediate check of the voltage used by the
 *          display: after you swipe your card, if the screen did work, how many
 *          pixels should be lit?
 */

use ndarray::Array2;

#[derive(Debug, PartialEq)]
pub enum Instruc {
    Rect(usize, usize),
    RotRow(usize, usize),
    RotCol(usize, usize),
}

pub struct Screen {
    pub size: (usize, usize),
    pub pixels: Array2<bool>,
    pub commands: Vec<Instruc>,
    pub com_file_path: String,
}

impl Screen {
    pub fn new(com_file_path: &str) -> Self {
        return Screen {
            size: (0, 0),
            pixels: Array2::from(Vec::<[bool; 2]>::new()),
            commands: Vec::new(),
            com_file_path: com_file_path.to_string(),
        };
    }

    pub fn load_commands(&self) {}

    pub fn set_rect(&mut self, a_val: usize, b_val: usize) {}

    pub fn rotate_row(&mut self, row_idx: usize, shift: usize) {}

    pub fn rotate_col(&mut self, col_idx: usize, shift: usize) {}

    pub fn execute_commands(&mut self) {}

    pub fn render(&self) -> Vec<String> {
        Vec::new()
    }

    pub fn on_pixels(&self) -> usize {
        0
    }

    fn show(self) {}
}

fn main() {}
