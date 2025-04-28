/*
 * --- Day 16: Permutation Promenade ---
 *
 * You come upon a very unusual sight; a group of programs here appear to be
 * dancing.
 *
 * There are sixteen programs in total, named a through p. They start by
 * standing in a line: a stands in position 0, b stands in position 1, and so on
 * until p, which stands in position 15.
 *
 * The programs' dance consists of a sequence of dance moves:
 *
 *      -   Spin, written sX, makes X programs move from the end to the front,
 *          but maintain their order otherwise. (For example, s3 on abcde
 *          produces cdeab).
 *
 *      -   Exchange, written xA/B, makes the programs at positions A and B swap
 *          places.
 *
 *      -   Partner, written pA/B, makes the programs named A and B swap places.
 *
 * For example, with only five programs standing in a line (abcde), they could
 * do the following dance:
 *
 *      -   s1, a spin of size 1: eabcd.
 *
 *      -   x3/4, swapping the last two programs: eabdc.
 *
 *      -   pe/b, swapping programs e and b: baedc.
 *
 * After finishing their dance, the programs end up in order baedc.
 *
 * PART 1:  You watch the dance for a while and record their dance moves (your
 *          puzzle input). In what order are the programs standing after their
 *          dance?
 */

use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
pub enum Move {
    Spin(i32),
    Exchange(usize, usize),
    Partner(char, char),
}

pub struct Promenade {
    pub group: Vec<char>,
    pub instruc: Vec<Move>,
}

impl Promenade {
    pub fn new(programs: &str, datafile: &str) -> Self {
        let mut instruc = Vec::new();

        /* Parse the string commands to enums */
        for cmd in read_to_string(datafile).unwrap().split(",") {
            let id = cmd.chars().next().unwrap();
            let info: String = cmd.chars().skip(1).collect();
            let parts: Vec<&str> = info.split("/").map(|x| x.trim()).collect();

            instruc.push(match id {
                's' => Move::Spin(parts[0].parse::<i32>().unwrap()),
                'x' => Move::Exchange(
                    parts[0].parse::<usize>().unwrap(),
                    parts[1].parse::<usize>().unwrap(),
                ),
                'p' => Move::Partner(
                    parts[0].chars().next().unwrap(),
                    parts[1].chars().next().unwrap(),
                ),
                _ => {
                    panic!("Invalid instruction '{cmd}'")
                }
            });
        }

        return Promenade {
            group: programs.chars().collect(),
            instruc,
        };
    }

    /// Rotate the order of the programs left or right.
    pub fn spin(&mut self, spin_mag: i32) {
        if spin_mag > 0 {
            self.group.rotate_right(spin_mag.abs() as usize);
        } else if spin_mag < 0 {
            self.group.rotate_right(spin_mag.abs() as usize);
        }
    }

    /// Swap programs based on index.
    pub fn exchange(&mut self, prog_0_idx: usize, prog_1_idx: usize) {
        let prog_0 = self.group[prog_0_idx];
        let prog_1 = self.group[prog_1_idx];
        self.group[prog_0_idx] = prog_1;
        self.group[prog_1_idx] = prog_0;
    }

    /// Swap programs based on value.
    pub fn partner(&mut self, prog_0: char, prog_1: char) {
        let prog_0_idx = self.group.iter().position(|x| *x == prog_0).unwrap();
        let prog_1_idx = self.group.iter().position(|x| *x == prog_1).unwrap();
        self.exchange(prog_0_idx, prog_1_idx);
    }

    /// Execute each instruction in order once.
    pub fn dance(&mut self) {
        for insr_idx in 0..self.instruc.len() {
            match self.instruc[insr_idx] {
                Move::Spin(mag) => self.spin(mag),
                Move::Exchange(idx0, idx1) => self.exchange(idx0, idx1),
                Move::Partner(chr0, chr1) => self.partner(chr0, chr1),
            }
        }
    }

    /// Give the value after executing each instruction in order once.
    pub fn one_dance(&mut self) -> String {
        self.dance();
        return self.group.iter().collect();
    }
}

fn main() {
    println!(
        "Part 1 = {}",
        Promenade::new("abcdefghijklmnop", "./data/input.txt").one_dance()
    );
}
