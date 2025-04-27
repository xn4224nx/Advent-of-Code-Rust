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
        Promenade {
            group: Vec::new(),
            instruc: Vec::new(),
        }
    }

    pub fn spin(&mut self, spin_mag: i32) {}

    pub fn exchange(&mut self, prog_0_idx: usize, prog_1_idx: usize) {}

    pub fn partner(&mut self, prog_0: char, prog_1: char) {}

    pub fn dance(&mut self) {}

    pub fn one_dance(&mut self) -> String {
        String::new()
    }
}

fn main() {}
