/*
 * --- Day 3: Crossed Wires ---
 *
 * The gravity assist was successful, and you're well on your way to the Venus
 * refuelling station. During the rush back on Earth, the fuel management system
 * wasn't completely installed, so that's next on the priority list.
 *
 * Opening the front panel reveals a jumble of wires. Specifically, two wires
 * are connected to a central port and extend outward on a grid. You trace the
 * path each wire takes as it leaves the central port, one wire per line of text
 * (your puzzle input).
 *
 * The wires twist and turn, but the two wires occasionally cross paths. To fix
 * the circuit, you need to find the intersection point closest to the central
 * port. Because the wires are on a grid, use the Manhattan distance for this
 * measurement. While the wires do technically cross right at the central port
 * where they both start, this point does not count, nor does a wire count as
 * crossing with itself.
 *
 * For example, if the first wire's path is R8,U5,L5,D3, then starting from the
 * central port (o), it goes right 8, up 5, left 5, and finally down 3:
 *
 *      ...........
 *      ...........
 *      ...........
 *      ....+----+.
 *      ....|....|.
 *      ....|....|.
 *      ....|....|.
 *      .........|.
 *      .o-------+.
 *      ...........
 *
 * Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down
 * 4, and left 4:
 *
 *      ...........
 *      .+-----+...
 *      .|.....|...
 *      .|..+--X-+.
 *      .|..|..|.|.
 *      .|.-X--+.|.
 *      .|..|....|.
 *      .|.......|.
 *      .o-------+.
 *      ...........
 *
 * These wires cross at two locations (marked X), but the lower-left one is
 * closer to the central port: its distance is 3 + 3 = 6.
 *
 * Here are a few more examples:
 *
 *      R75,D30,R83,U83,L12,D49,R71,U7,L72
 *      U62,R66,U55,R34,D71,R55,D58,R83             = distance 159
 *
 *      R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
 *      U98,R91,D20,R16,D67,R40,U7,R15,U6,R7        = distance 135
 *
 * PART 1:  What is the Manhattan distance from the central port to the closest
 *          intersection?
 */

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct WireInteraction {
    pub wire_loc_dists: Vec<HashMap<(i32, i32), usize>>,
}

impl WireInteraction {
    pub fn from_file(directs_file: &str) -> Self {
        let mut buffer = String::new();
        let mut wire_loc_dists = Vec::new();

        /* Open the file. */
        let file = File::open(directs_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Each line of the file will be the complete path of a single wire. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            let mut all_dists = HashMap::new();

            /* Start the wire at an origin. */
            let mut curr_pnt = (0, 0);

            /* Keep track of the length of the wire. */
            let mut curr_wire_len = 0;

            /* Move the point based on the direction and distance given. */
            for raw_move in buffer.trim_end().split(",") {
                let raw = raw_move.chars().collect::<Vec<char>>();
                let dist = raw[1..]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();

                /* Follow the wire along its directed path */
                for _ in 0..dist {
                    curr_wire_len += 1;

                    if raw[0] == 'U' {
                        curr_pnt = (curr_pnt.0, curr_pnt.1 + 1)
                    } else if raw[0] == 'D' {
                        curr_pnt = (curr_pnt.0, curr_pnt.1 - 1)
                    } else if raw[0] == 'L' {
                        curr_pnt = (curr_pnt.0 - 1, curr_pnt.1)
                    } else if raw[0] == 'R' {
                        curr_pnt = (curr_pnt.0 + 1, curr_pnt.1)
                    } else {
                        panic!("Unknown direction: '{}'", raw[0]);
                    }

                    /* Save the new position the wire has reached. */
                    all_dists.insert(curr_pnt, curr_wire_len);
                }
            }
            wire_loc_dists.push(all_dists);
            buffer.clear();
        }
        return WireInteraction { wire_loc_dists };
    }

    pub fn short_circuit_dist(&self) -> usize {
        let mut crossed_pnts = HashSet::new();

        /* Find all the points where the wires cross. */
        for pnt_0 in self.wire_loc_dists[0].keys() {
            if crossed_pnts.contains(pnt_0) {
                continue;
            }

            if self.wire_loc_dists[1].contains_key(pnt_0) {
                crossed_pnts.insert(pnt_0);
            }
        }

        /* Find the point of crossing closest to the origin. */
        return crossed_pnts
            .iter()
            .map(|(x, y)| x.abs() + y.abs())
            .min()
            .unwrap() as usize;
    }
}

fn main() {}
