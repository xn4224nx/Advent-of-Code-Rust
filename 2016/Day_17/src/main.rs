/*
 * --- Day 17: Two Steps Forward ---
 *
 * You're trying to access a secure vault protected by a 4x4 grid of small
 * rooms connected by doors. You start in the top-left room (marked S), and you
 * can access the vault (marked V) once you reach the bottom-right room:
 *
 *      #########
 *      #S| | | #
 *      #-#-#-#-#
 *      # | | | #
 *      #-#-#-#-#
 *      # | | | #
 *      #-#-#-#-#
 *      # | | |
 *      ####### V
 *
 * Fixed walls are marked with #, and doors are marked with - or |.
 *
 * The doors in your current room are either open or closed (and locked) based
 * on the hexadecimal MD5 hash of a passcode (your puzzle input) followed by a
 * sequence of uppercase characters representing the path you have taken so far
 * (U for up, D for down, L for left, and R for right).
 *
 * Only the first four characters of the hash are used; they represent,
 * respectively, the doors up, down, left, and right from your current
 * position. Any b, c, d, e, or f means that the corresponding door is open;
 * any other character (any number or a) means that the corresponding door is
 * closed and locked.
 *
 * To access the vault, all you need to do is reach the bottom-right room;
 * reaching this room opens the vault and all doors in the maze.
 *
 * PART 1:  Given your vault's passcode, what is the shortest path (the actual
 *          path, not just the length) to reach the vault?
 */

pub struct Maze {
    pub strt_pnt: (usize, usize),
    pub end_pnt: (usize, usize),
    pub seed: String,
}

impl Maze {
    pub fn new(seed: &str) -> Self {
        Maze {
            strt_pnt: (0, 0),
            end_pnt: (3, 3),
            seed: String::from(seed),
        }
    }

    /// Find out where in the maze a set of instructions will take you
    pub fn directs_2_coords(&self, direcs: &Vec<char>) -> (usize, usize) {
        self.strt_pnt
    }

    /// Determine the viable next steps in the maze based on the previous steps
    pub fn viable_dirs(&self, prev_path: &Vec<char>) -> Vec<char> {
        Vec::new()
    }

    /// Find the shortest path through the maze
    pub fn find_shortest_path(&self) -> String {
        "".to_string()
    }
}

fn main() {}
