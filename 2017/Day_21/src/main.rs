/*
 * --- Day 21: Fractal Art ---
 *
 * You find a program trying to generate some art. It uses a strange process
 * that involves repeatedly enhancing the detail of an image through a set of
 * rules.
 *
 * The image consists of a two-dimensional square grid of pixels that are either
 * on (#) or off (.). The program always begins with this pattern:
 *
 *      .#.
 *      ..#
 *      ###
 *
 * Because the pattern is both 3 pixels wide and 3 pixels tall, it is said to
 * have a size of 3.
 *
 * Then, the program repeats the following process:
 *
 *      -   If the size is evenly divisible by 2, break the pixels up into 2x2
 *          squares, and convert each 2x2 square into a 3x3 square by following
 *          the corresponding enhancement rule.
 *
 *      -   Otherwise, the size is evenly divisible by 3; break the pixels up
 *          into 3x3 squares, and convert each 3x3 square into a 4x4 square by
 *          following the corresponding enhancement rule.
 *
 * Because each square of pixels is replaced by a larger one, the image gains
 * pixels and so its size increases.
 *
 * The artist's book of enhancement rules is nearby (your puzzle input);
 * however, it seems to be missing rules. The artist explains that sometimes,
 * one must rotate or flip the input pattern to find a match. (Never rotate or
 * flip the output pattern, though.) Each pattern is written concisely: rows are
 * listed as single units, ordered top-down, and separated by slashes. For
 * example, the following rules correspond to the adjacent patterns:
 *
 *      ../.#  =    ..
 *                  .#
 *
 *                      .#.
 *      .#./..#/###  =  ..#
 *                      ###
 *
 *                              #..#
 *      #..#/..../#..#/.##.  =  ....
 *                              #..#
 *                              .##.
 * When searching for a rule to use, rotate and flip the pattern as necessary.
 * For example, all of the following patterns match the same rule:
 *
 *      .#.   .#.   #..   ###
 *      ..#   #..   #.#   ..#
 *      ###   ###   ##.   .#.
 *
 * Suppose the book contained the following two rules:
 *
 *      ../.# => ##./#../...
 *      .#./..#/### => #..#/..../..../#..#
 *
 * As before, the program begins with this pattern:
 *
 *      .#.
 *      ..#
 *      ###
 *
 * The size of the grid (3) is not divisible by 2, but it is divisible by 3. It
 * divides evenly into a single square; the square matches the second rule,
 * which produces:
 *
 *      #..#
 *      ....
 *      ....
 *      #..#
 *
 * The size of this enhanced grid (4) is evenly divisible by 2, so that rule is
 * used. It divides evenly into four squares:
 *
 *      #.|.#
 *      ..|..
 *      --+--
 *      ..|..
 *      #.|.#
 *
 * Each of these squares matches the same rule (../.# => ##./#../...), three of
 * which require some flipping and rotation to line up with the rule. The output
 * for the rule is the same in all four cases:
 *
 *      ##.|##.
 *      #..|#..
 *      ...|...
 *      ---+---
 *      ##.|##.
 *      #..|#..
 *      ...|...
 *
 * Finally, the squares are joined into a new grid:
 *
 *      ##.##.
 *      #..#..
 *      ......
 *      ##.##.
 *      #..#..
 *      ......
 *
 * Thus, after 2 iterations, the grid contains 12 pixels that are on.
 *
 * PART 1:  How many pixels stay on after 5 iterations?
 */

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pixels {
    pub square: Vec<Vec<u8>>,
}

pub struct FractalArt {
    pub transformations: HashMap<Pixels, Pixels>,
    pub state: Pixels,
}

impl Pixels {
    /// Rotate the square pixels to the right around the centre
    pub fn rotate_right(&mut self, k: usize) {
        for _ in 0..k % 4 {
            /* Transpose the square */
            for x_idx in 0..self.square.len() {
                for y_idx in x_idx..self.square.len() {
                    let tmp = self.square[y_idx][x_idx];
                    self.square[y_idx][x_idx] = self.square[x_idx][y_idx];
                    self.square[x_idx][y_idx] = tmp;
                }
            }

            /* Reverse each row */
            for rw_idx in 0..self.square.len() {
                self.square[rw_idx].reverse();
            }
        }
    }

    /// Swap the values in the square across the horizontal axis
    pub fn flip_vertical(&mut self) {
        /* Iterate over the top and bottom rows that will be mirrored. */
        for row_idx in 0..self.square.len() / 2 {
            let tp_row_idx = row_idx;
            let bt_row_idx = self.square.len() - 1 - row_idx;

            /* Swap the rows, column by column. */
            for col_idx in 0..self.square.len() {
                let tmp = self.square[tp_row_idx][col_idx];
                self.square[tp_row_idx][col_idx] = self.square[bt_row_idx][col_idx];
                self.square[bt_row_idx][col_idx] = tmp;
            }
        }
    }

    /// Swap the values in the square across the vertical axis
    pub fn flip_horizontal(&mut self) {
        for rw_idx in 0..self.square.len() {
            self.square[rw_idx].reverse();
        }
    }
}

impl FractalArt {
    pub fn new(transforms_file: &str) -> Self {
        let mut buffer = String::new();
        let mut transformations = HashMap::new();

        /* Open the transformation file. */
        let file = File::open(transforms_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Iterate over the file line by line. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            let mut tmp = Vec::new();
            let mut init_state = Vec::new();
            let mut final_state = Vec::new();

            /* Seperate the inital and final states in this transformation. */
            let (init_r, final_r) = buffer.trim().split_once(" => ").unwrap();

            /* Parse the inital state into arrays. */
            for s_chr in init_r.chars() {
                match s_chr {
                    '.' => tmp.push(0),
                    '#' => tmp.push(1),
                    '/' => {
                        init_state.push(tmp.clone());
                        tmp.clear();
                    }
                    _ => panic!("Unknown char: '{}'", s_chr),
                }
            }
            init_state.push(tmp.clone());
            tmp.clear();

            /* Parse the final state into arrays. */
            for f_chr in final_r.chars() {
                match f_chr {
                    '.' => tmp.push(0),
                    '#' => tmp.push(1),
                    '/' => {
                        final_state.push(tmp.clone());
                        tmp.clear();
                    }
                    _ => panic!("Unknown char: '{}'", f_chr),
                }
            }
            final_state.push(tmp.clone());
            tmp.clear();

            /* Keep a record of all possible inital states and start with unmodified. */
            let mut all_init = vec![Pixels {
                square: init_state.clone(),
            }];

            /* Rotate right once */
            let mut tmp_scre = Pixels {
                square: init_state.clone(),
            };
            tmp_scre.rotate_right(1);
            all_init.push(tmp_scre);

            /* Rotate right twice */
            let mut tmp_scre = Pixels {
                square: init_state.clone(),
            };
            tmp_scre.rotate_right(2);
            all_init.push(tmp_scre);

            /* Rotate right three times */
            let mut tmp_scre = Pixels {
                square: init_state.clone(),
            };
            tmp_scre.rotate_right(3);
            all_init.push(tmp_scre);

            /* Rotate right once and flip vertically */
            let mut tmp_scre = Pixels {
                square: init_state.clone(),
            };
            tmp_scre.rotate_right(1);
            tmp_scre.flip_vertical();
            all_init.push(tmp_scre);

            /* Rotate right twice and flip vertically */
            let mut tmp_scre = Pixels {
                square: init_state.clone(),
            };
            tmp_scre.rotate_right(2);
            tmp_scre.flip_vertical();
            all_init.push(tmp_scre);

            /* Rotate right three times and flip vertically */
            let mut tmp_scre = Pixels {
                square: init_state.clone(),
            };
            tmp_scre.rotate_right(3);
            tmp_scre.flip_vertical();
            all_init.push(tmp_scre);

            /* Rotate right once and flip horizontally */
            let mut tmp_scre = Pixels {
                square: init_state.clone(),
            };
            tmp_scre.rotate_right(1);
            tmp_scre.flip_horizontal();
            all_init.push(tmp_scre);

            /* Rotate right twice and flip horizontally */
            let mut tmp_scre = Pixels {
                square: init_state.clone(),
            };
            tmp_scre.rotate_right(2);
            tmp_scre.flip_horizontal();
            all_init.push(tmp_scre);

            /* Rotate right three times and flip horizontally */
            let mut tmp_scre = Pixels {
                square: init_state.clone(),
            };
            tmp_scre.rotate_right(3);
            tmp_scre.flip_horizontal();
            all_init.push(tmp_scre);

            /* Flip horizontally */
            let mut tmp_scre = Pixels {
                square: init_state.clone(),
            };
            tmp_scre.flip_horizontal();
            all_init.push(tmp_scre);

            /* Flip vertically */
            let mut tmp_scre = Pixels {
                square: init_state.clone(),
            };
            tmp_scre.flip_vertical();
            all_init.push(tmp_scre);

            /* Add in all the possible transformations */
            for trans in all_init.drain(..) {
                transformations.insert(
                    trans,
                    Pixels {
                        square: final_state.clone(),
                    },
                );
            }
            buffer.clear();
        }

        return FractalArt {
            transformations,
            state: Pixels {
                square: vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1]],
            },
        };
    }

    /// How many pixels are on in the artwork
    pub fn on_pixels(&self) -> usize {
        let mut on_pixel_cnt: usize = 0;

        for row_idx in 0..self.state.square.len() {
            for col_idx in 0..self.state.square.len() {
                on_pixel_cnt += self.state.square[row_idx][col_idx] as usize;
            }
        }
        return on_pixel_cnt;
    }

    /// Split the artwork into 2x2 or 3x3 chunks and create a new iteration of
    /// the artwork by substituting in different transformations
    pub fn transform(&mut self) {
        /* Calculate the chunk size, n. */
        let n = if self.state.square.len() % 2 == 0 {
            2
        } else if self.state.square.len() % 3 == 0 {
            3
        } else {
            panic!(
                "Square of size {} cannot be processed!",
                self.state.square.len()
            );
        };
        let new_side_len = (n + 1) * self.state.square.len() / n;

        /* Preallocate the new pixels. */
        let mut new_pixels: Vec<Vec<u8>> = vec![vec![0; new_side_len]; new_side_len];

        /* Iterate over every chunk and find its transformation. */
        for x in 0..self.state.square.len() / n {
            for y in 0..self.state.square.len() / n {
                let mut old_chunk = Pixels {
                    square: vec![vec![0; n]; n],
                };

                /* Fill the old chunk. */
                for c_x in 0..n {
                    for c_y in 0..n {
                        old_chunk.square[c_x][c_y] = self.state.square[x * n + c_x][y * n + c_y];
                    }
                }

                /* Lookup what the chuck transforms into. */
                let new_chunk = self
                    .transformations
                    .get(&old_chunk)
                    .expect(&format!("Chunk {:?} could not be found!", old_chunk));

                /* Set the values of the new pixels based on this new chunk. */
                for c_x in 0..new_chunk.square.len() {
                    for c_y in 0..new_chunk.square.len() {
                        let chx = x * (n + 1) + c_x;
                        let chy = y * (n + 1) + c_y;
                        new_pixels[chx][chy] = new_chunk.square[c_x][c_y];
                    }
                }
            }
        }
        self.state.square = new_pixels;
    }

    /// Count the pixels are on after a certain number of transformations
    pub fn final_on_pixels(&mut self, num_transforms: usize) -> usize {
        for _ in 0..num_transforms {
            self.transform();
        }
        return self.on_pixels();
    }
}

fn main() {
    println!(
        "Part 1 = {}\nPart 2 = {}\n",
        FractalArt::new("./data/input.txt").final_on_pixels(5),
        FractalArt::new("./data/input.txt").final_on_pixels(18)
    );
}
