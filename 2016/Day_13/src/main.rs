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
        false
    }

    /// From a particular point in the maze work out the viable next moves
    pub fn next_viable_moves(&self, point: (u32, u32)) -> Vec<(u32, u32)> {
        Vec::new()
    }

    /// Find the length of the shortest route to a point in the maze
    pub fn shortest_route_to_point(&self, end_point: (u32, u32)) -> u32 {
        0
    }
}

fn main() {}
