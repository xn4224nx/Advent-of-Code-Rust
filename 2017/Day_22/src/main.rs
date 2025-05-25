/*
 * --- Day 22: Sporifica Virus ---
 *
 * Diagnostics indicate that the local grid computing cluster has been
 * contaminated with the Sporifica Virus. The grid computing cluster is a
 * seemingly-infinite two-dimensional grid of compute nodes. Each node is either
 * clean or infected by the virus.
 *
 * To prevent overloading the nodes (which would render them useless to the
 * virus) or detection by system administrators, exactly one virus carrier moves
 * through the network, infecting or cleaning nodes as it moves. The virus
 * carrier is always located on a single node in the network (the current node)
 * and keeps track of the direction it is facing.
 *
 * To avoid detection, the virus carrier works in bursts; in each burst, it
 * wakes up, does some work, and goes back to sleep. The following steps are all
 * executed in order one time each burst:
 *
 *      -   If the current node is infected, it turns to its right. Otherwise,
 *          it turns to its left. (Turning is done in-place; the current node
 *          does not change.)
 *
 *      -   If the current node is clean, it becomes infected. Otherwise, it
 *          becomes cleaned. (This is done after the node is considered for the
 *          purposes of changing direction.)
 *
 *      -   The virus carrier moves forward one node in the direction it is
 *          facing.
 *
 * Diagnostics have also provided a map of the node infection status (your
 * puzzle input). Clean nodes are shown as .; infected nodes are shown as #.
 * This map only shows the center of the grid; there are many more nodes beyond
 * those shown, but none of them are currently infected.
 *
 * The virus carrier begins in the middle of the map facing up.
 *
 * For example, suppose you are given a map like this:
 *
 *          ..#
 *          #..
 *          ...
 *
 * Then, the middle of the infinite grid looks like this, with the virus
 * carrier's position marked with [ ]:
 *
 *          . . . . . . . . .
 *          . . . . . . . . .
 *          . . . . . . . . .
 *          . . . . . # . . .
 *          . . . #[.]. . . .
 *          . . . . . . . . .
 *          . . . . . . . . .
 *          . . . . . . . . .
 *
 * The virus carrier is on a clean node, so it turns left, infects the node, and
 * moves left:
 *
 *          . . . . . . . . .
 *          . . . . . . . . .
 *          . . . . . . . . .
 *          . . . . . # . . .
 *          . . .[#]# . . . .
 *          . . . . . . . . .
 *          . . . . . . . . .
 *          . . . . . . . . .
 *
 * The virus carrier is on an infected node, so it turns right, cleans the node,
 * and moves up:
 *
 *          . . . . . . . . .
 *          . . . . . . . . .
 *          . . . . . . . . .
 *          . . .[.]. # . . .
 *          . . . . # . . . .
 *          . . . . . . . . .
 *          . . . . . . . . .
 *          . . . . . . . . .
 *
 * Four times in a row, the virus carrier finds a clean, infects it, turns left,
 * and moves forward, ending in the same place and still facing up:
 *
 *          . . . . . . . . .
 *          . . . . . . . . .
 *          . . . . . . . . .
 *          . . #[#]. # . . .
 *          . . # # # . . . .
 *          . . . . . . . . .
 *          . . . . . . . . .
 *          . . . . . . . . .
 *
 * Now on the same node as before, it sees an infection, which causes it to turn
 * right, clean the node, and move forward:
 *
 *          . . . . . . . . .
 *          . . . . . . . . .
 *          . . . . . . . . .
 *          . . # .[.]# . . .
 *          . . # # # . . . .
 *          . . . . . . . . .
 *          . . . . . . . . .
 *          . . . . . . . . .
 *
 * After the above actions, a total of 7 bursts of activity had taken place. Of
 * them, 5 bursts of activity caused an infection.
 *
 * After a total of 70, the grid looks like this, with the virus carrier facing
 * up:
 *
 *          . . . . . # # . .
 *          . . . . # . . # .
 *          . . . # . . . . #
 *          . . # . #[.]. . #
 *          . . # . # . . # .
 *          . . . . . # # . .
 *          . . . . . . . . .
 *          . . . . . . . . .
 *
 * By this time, 41 bursts of activity caused an infection (though most of those
 * nodes have since been cleaned).
 *
 * After a total of 10000 bursts of activity, 5587 bursts will have caused an
 * infection.
 *
 * PART 1:  Given your actual map, after 10000 bursts of activity, how many
 *          bursts cause a node to become infected? (Do not count nodes that
 *          begin infected.)
 */

use num::complex::Complex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Infection {
    pub carr_loc: (i32, i32),
    pub carr_dir: Complex<i32>,
    pub infected_nodes: HashSet<(i32, i32)>,
}

impl Infection {
    pub fn new(initial_state: &str) -> Self {
        let mut buffer = String::new();
        let mut infected_nodes = HashSet::new();

        /* Open the File. */
        let file = File::open(initial_state).unwrap();
        let mut fp = BufReader::new(file);

        /* Iterate over the file line by line. */
        let mut row_idx = 0;
        let mut col_idx = 0;
        while fp.read_line(&mut buffer).unwrap() > 0 {
            col_idx = 0;

            /* Register only the infected nodes. */
            for comp_char in buffer.trim().chars() {
                if comp_char == '#' {
                    infected_nodes.insert((col_idx, row_idx));
                }
                col_idx += 1;
            }
            buffer.clear();
            row_idx += 1;
        }

        return Infection {
            carr_loc: (col_idx / 2, row_idx / 2),
            carr_dir: Complex::new(0, 1),
            infected_nodes,
        };
    }

    /// Simulate the carrier moving once
    pub fn burst(&mut self) {
        /* Currently on an infected node. */
        if self.infected_nodes.contains(&self.carr_loc) {
            self.carr_dir *= Complex::new(0, -1);
            self.infected_nodes.remove(&self.carr_loc);

        /* Currently on a clean node. */
        } else {
            self.carr_dir *= Complex::new(0, 1);
            self.infected_nodes.insert(self.carr_loc);
        }

        /* Move the carrier one space in direction it is pointing. */
        self.carr_loc = (
            self.carr_loc.0 + self.carr_dir.re,
            self.carr_loc.1 - self.carr_dir.im,
        );
    }

    /// Count the number of burst that cause a node to become infected.
    pub fn num_infected_nodes(&mut self, num_bursts: usize) -> usize {
        let mut infected_cnt = 0;

        for _ in 0..num_bursts {
            let old_infected_cnt = self.infected_nodes.len();
            self.burst();

            if self.infected_nodes.len() > old_infected_cnt {
                infected_cnt += 1;
            }
        }
        return infected_cnt;
    }
}

fn main() {
    println!(
        "Part 1 = {}",
        Infection::new("./data/input.txt").num_infected_nodes(10000)
    );
}
