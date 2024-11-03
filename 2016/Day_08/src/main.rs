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

use ndarray::{s, Array, Array2};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    pub fn new(com_file_path: &str, size: (usize, usize)) -> Self {
        return Screen {
            size,
            pixels: Array2::from_elem(size, false),
            commands: Vec::new(),
            com_file_path: com_file_path.to_string(),
        };
    }

    /// Read and parse the commands from com_file_path
    pub fn load_commands(&mut self) {
        let mut buffer = String::new();

        /* Define the regexes to extract the key bits of data from a line. */
        let re_set_rec = Regex::new(r"rect (\d+)x(\d+)").unwrap();
        let re_rot_row = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
        let re_rot_col = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();

        /* Open the file. */
        let file = File::open(&self.com_file_path).unwrap();
        let mut fp = BufReader::new(file);

        /* Iterate over the file line by line */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            /* Test for a set command. */
            if let Some(caps) = re_set_rec.captures(&buffer) {
                self.commands.push(Instruc::Rect(
                    caps[1].parse::<usize>().unwrap(),
                    caps[2].parse::<usize>().unwrap(),
                ));

            /* Test for a rotate column command. */
            } else if let Some(caps) = re_rot_row.captures(&buffer) {
                self.commands.push(Instruc::RotRow(
                    caps[1].parse::<usize>().unwrap(),
                    caps[2].parse::<usize>().unwrap(),
                ))

            /* Test for a rotate row command. */
            } else if let Some(caps) = re_rot_col.captures(&buffer) {
                self.commands.push(Instruc::RotCol(
                    caps[1].parse::<usize>().unwrap(),
                    caps[2].parse::<usize>().unwrap(),
                ))
            };
            buffer.clear();
        }
    }

    /// Set a particular rectangle at the origin to be on
    pub fn set_rect(&mut self, a_val: usize, b_val: usize) {
        self.pixels.slice_mut(s![..b_val, ..a_val]).fill(true)
    }

    /// Shift a row of the screen right by a certain amount
    pub fn rotate_row(&mut self, row_idx: usize, shift: usize) {
        /* Remove excess rotations */
        let shift_r = shift % self.size.1;

        /* Shift a copy of the row to the right */
        let mut orig_row = self.pixels.row(row_idx).to_vec();
        orig_row.rotate_right(shift_r);

        /* Set the cycled column as the new version. */
        self.pixels
            .slice_mut(s![row_idx, ..])
            .assign(&Array::from_vec(orig_row));
    }

    /// Shift a column of the screen down by a certain amount
    pub fn rotate_col(&mut self, col_idx: usize, shift: usize) {
        /* Remove excess rotations */
        let shift_c = shift % self.size.0;

        /* Shift a copy of the column to the right */
        let mut orig_col = self.pixels.column(col_idx).to_vec();
        orig_col.rotate_right(shift_c);

        /* Set the cycled column as the new version. */
        self.pixels
            .slice_mut(s![.., col_idx])
            .assign(&Array::from_vec(orig_col));
    }

    /// Enact the all the commands stored in self.commands
    pub fn execute_commands(&mut self) {
        for cmd_idx in 0..self.commands.len() {
            match self.commands[cmd_idx] {
                Instruc::Rect(a, b) => self.set_rect(a, b),
                Instruc::RotRow(a, b) => self.rotate_row(a, b),
                Instruc::RotCol(a, b) => self.rotate_col(a, b),
            }
        }
    }

    /// Create a string that represents the current state of the screen
    pub fn render(&self) -> String {
        let mut view = String::new();

        /* Iterate over the array and conver to a char based on the value. */
        for (idx, scr_char) in self.pixels.iter().enumerate() {
            /* Detect the end of a row of pixels. */
            if idx > 0 && idx % self.size.1 == 0 {
                view.push('\n');
            }

            match scr_char {
                true => view.push('#'),
                false => view.push('.'),
            }
        }
        return view;
    }

    /// Determine the total number of on pixels in the screen
    pub fn on_pixels(&self) -> usize {
        self.pixels.iter().filter(|x| **x).count()
    }

    /// Show the pixel array to screen
    fn show(&self) {
        println!("{}", self.render());
    }
}

fn main() {}
