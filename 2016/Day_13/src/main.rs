/*
 * --- Day 13: A Maze of Twisty Little Cubicles ---
 *
 * You arrive at the first floor of this new building to discover a much less
 * welcoming environment than the shiny atrium of the last one. Instead, you
 * are in a maze of twisty little cubicles, all alike.
 *
 * Every location in this area is addressed by a pair of non-negative integers
 * (x,y). Each such coordinate is either a wall or an open space. You can't
 * move diagonally. The cube maze starts at 0,0 and seems to extend infinitely
 * toward positive x and y; negative values are invalid, as they represent a
 * location outside the building. You are in a small waiting area at 1,1.
 *
 * While it seems chaotic, a nearby morale-boosting poster explains, the layout
 * is actually quite logical. You can determine whether a given x,y coordinate
 * will be a wall or an open space using a simple system:
 *
 *  -   Find x*x + 3*x + 2*x*y + y + y*y.
 *
 *  -   Add the office designer's favorite number (your puzzle input).
 *
 *  -   Find the binary representation of that sum; count the number of bits
 *      that are 1:
 *
 *      -   If the number of bits that are 1 is even, it's an open space.
 *
 *      -   If the number of bits that are 1 is odd, it's a wall.
 *
 * PART 1:  What is the fewest number of steps required for you to reach 31,39?
 */

use std::collections::HashSet;

pub struct Maze {
    pub seed: u32,
    pub start: (u32, u32),
}

impl Maze {
    pub fn new(seed: u32) -> Self {
        return Maze {
            seed,
            start: (1, 1),
        };
    }

    /// Is a point in the maze open for travel
    pub fn is_open_space(&self, point: (u32, u32)) -> bool {
        let x = point.0;
        let y = point.1;
        let number = x * x + 3 * x + 2 * x * y + y + y * y + self.seed;

        /* Count the number of bits in the number. */
        let bin_num_bits = number.count_ones();

        /* An open area has an even number of bits. */
        return bin_num_bits % 2 == 0;
    }

    /// From a particular point in the maze work out the viable next moves
    pub fn next_viable_moves(&self, point: (u32, u32)) -> Vec<(u32, u32)> {
        let x = point.0;
        let y = point.1;
        let mut moves = Vec::new();

        /* The square above. */
        if y > 0 && self.is_open_space((x, y - 1)) {
            moves.push((x, y - 1));
        }

        /* The square below. */
        if self.is_open_space((x, y + 1)) {
            moves.push((x, y + 1));
        }

        /* The square to the left. */
        if x > 0 && self.is_open_space((x - 1, y)) {
            moves.push((x - 1, y));
        }

        /* The square to the right. */
        if self.is_open_space((x + 1, y)) {
            moves.push((x + 1, y));
        }

        return moves;
    }

    /// Find the length of the shortest route to a point in the maze
    pub fn shortest_route_to_point(&self, end_point: (u32, u32)) -> u32 {
        let mut seen_pnts = HashSet::new();
        let mut curr_pnts = HashSet::from([self.start]);
        let mut move_cnt = 0;

        while !curr_pnts.contains(&end_point) {
            let mut next_pnts = HashSet::new();

            /* Work out all the next possible moves */
            for c_pnt in curr_pnts.iter() {
                for n_pnt in self.next_viable_moves(*c_pnt).iter() {
                    if !seen_pnts.contains(n_pnt) {
                        next_pnts.insert(*n_pnt);
                        seen_pnts.insert(*n_pnt);
                    }
                }
            }
            curr_pnts = next_pnts;
            move_cnt += 1;
        }
        return move_cnt;
    }
}

fn main() {}
