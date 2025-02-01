/*
 * --- Day 22: Grid Computing ---
 *
 * You gain access to a massive storage cluster arranged in a grid; each storage
 * node is only connected to the four nodes directly adjacent to it (three if
 * the node is on an edge, two if it's in a corner).
 *
 * You can directly access data only on node /dev/grid/node-x0-y0, but you can
 * perform some limited actions on the other nodes:
 *
 *      -   You can get the disk usage of all nodes (via df). The result of
 *          doing this is in your puzzle input.
 *
 *      -   You can instruct a node to move (not copy) all of its data to an
 *          adjacent node (if the destination node has enough space to receive
 *          the data). The sending node is left empty after this operation.
 *
 * Nodes are named by their position: the node named node-x10-y10 is adjacent to
 * nodes node-x9-y10, node-x11-y10, node-x10-y9, and node-x10-y11.
 *
 * Before you begin, you need to understand the arrangement of data on these
 * nodes. Even though you can only move data between directly connected nodes,
 * you're going to need to rearrange a lot of the data to get access to the data
 * you need. Therefore, you need to work out how you might be able to shift data
 * around.
 *
 * To do this, you'd like to count the number of viable pairs of nodes. A viable
 * pair is any two nodes (A,B), regardless of whether they are directly
 * connected, such that:
 *
 *      -   Node A is not empty (its Used is not zero).
 *
 *      -   Nodes A and B are not the same node.
 *
 *      -   The data on node A (its Used) would fit on node B (its Avail).
 *
 *  PART 1: How many viable pairs of nodes are there?
 *
 * Now that you have a better understanding of the grid, it's time to get to
 * work.
 *
 * Your goal is to gain access to the data which begins in the node with y=0 and
 * the highest x (that is, the node in the top-right corner).
 *
 * PART 2:  What is the fewest number of steps required to move your goal data
 *          to node-x0-y0?
 */

use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Debug)]
pub enum Status {
    Blocked,
    Empty,
    Full,
    Goal,
}

pub struct ComputingGrid {
    pub node_locs: Vec<(u32, u32)>,
    pub node_status: Vec<Status>,
    pub goal_node_idx: usize,
    pub dest_node_idx: usize,
    pub empt_node_idx: usize,
}

impl ComputingGrid {
    pub fn new(usage_readout: &str) -> Self {
        let mut buffer = String::new();
        let mut node_locs = Vec::new();
        let mut usages = Vec::new();
        let mut capacities = Vec::new();
        let re = Regex::new(r"(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%").unwrap();

        /* Open the file. */
        let file = File::open(usage_readout).unwrap();
        let mut fp = BufReader::new(file);

        /* Read line by line. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            if let Some(caps) = re.captures(&buffer) {
                node_locs.push((
                    caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                    caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                ));
                capacities.push(caps.get(3).unwrap().as_str().parse::<u32>().unwrap());
                usages.push(caps.get(4).unwrap().as_str().parse::<u32>().unwrap());
            }
            buffer.clear();
        }

        /* Find the empty node */
        let mut empt_node_cap: Option<u32> = None;
        let mut empt_node_idx: Option<usize> = None;
        for node_idx in 0..node_locs.len() {
            if usages[node_idx] == 0 {
                empt_node_cap = Some(capacities[node_idx]);
                empt_node_idx = Some(node_idx);
            }
        }
        if empt_node_cap.is_none() || empt_node_cap.is_none() {
            panic!("No empty node was found in the data!")
        }
        let empt_node_cap = empt_node_cap.unwrap();
        let empt_node_idx = empt_node_idx.unwrap();

        /* Label the nodes. */
        let mut node_status: Vec<Status> = usages
            .iter()
            .map(|x| match x {
                0 => Status::Empty,
                d if *d <= empt_node_cap => Status::Full,
                d if *d > empt_node_cap => Status::Blocked,
                _ => panic!("{x} is not covered!"),
            })
            .collect();

        /* Label the goal node as the node with the top right corner. */
        let mut goal_node_idx = 0;
        let mut highest_x = 0;
        for n_idx in 0..node_locs.len() {
            let x_coord = node_locs[n_idx].0;
            let y_coord = node_locs[n_idx].1;

            if y_coord == 0 && x_coord > highest_x {
                goal_node_idx = n_idx;
                highest_x = x_coord;
            }
        }
        node_status[goal_node_idx] = Status::Goal;

        /* Label the node to the left of the goal node. */
        let mut dest_node_idx = 0;
        let mut sec_highest_x = 0;
        for n_idx in 0..node_locs.len() {
            let x_coord = node_locs[n_idx].0;
            let y_coord = node_locs[n_idx].1;

            if y_coord == 0 && x_coord > sec_highest_x && n_idx != goal_node_idx {
                dest_node_idx = n_idx;
                sec_highest_x = x_coord;
            }
        }
        return ComputingGrid {
            node_locs,
            node_status,
            goal_node_idx,
            dest_node_idx,
            empt_node_idx,
        };
    }

    /// Count the number of swaps that could happen in the grid
    pub fn len_viable_swaps(&self) -> usize {
        return (0..self.node_status.len())
            .permutations(2)
            .filter(|x| {
                self.node_status[x[0]] != Status::Blocked && self.node_status[x[1]] == Status::Empty
            })
            .count();
    }

    /// Determine the moves that the specific node can make in this config and
    /// return the indexes of the nodes that the
    pub fn viable_moves(&self, s_node_idx: usize) -> Vec<usize> {
        let mut moves = Vec::new();
        let curr_x = self.node_locs[s_node_idx].0;
        let curr_y = self.node_locs[s_node_idx].1;

        for n_idx in 0..self.node_locs.len() {
            let node_x = self.node_locs[n_idx].0;
            let node_y = self.node_locs[n_idx].1;

            /* A node cannot move to its own location. */
            if n_idx == s_node_idx {
                continue;
            };

            /* Is the node above, below, left or right the current one. */
            if (node_x == curr_x && (node_y == curr_y + 1 || node_y + 1 == curr_y))
                || (node_y == curr_y && (node_x == curr_x + 1 || node_x + 1 == curr_x))
            {
                moves.push(n_idx)
            }
        }
        return moves;
    }

    /// Determine the number of moves to get the empty node to the destination
    /// node. The node to the left of the goal node. Attempt to brute force it
    /// with Dijkstra's algorithm.
    pub fn fewest_steps_to_dest(&self) -> u32 {
        let mut unvisted_nodes: HashSet<usize> = (0..self.node_locs.len()).collect();
        let mut node_dist: HashMap<usize, u32> =
            (0..self.node_locs.len()).map(|x| (x, u32::MAX)).collect();

        /* Start at the empty node. */
        node_dist.insert(self.empt_node_idx, 0);

        loop {
            let mut curr_idx = 0;
            let mut curr_dist = u32::MAX;

            /* Find the unvisited node with the smallest distance. */
            for uv_nd_idx in unvisted_nodes.iter() {
                if node_dist[uv_nd_idx] < curr_dist {
                    curr_idx = *uv_nd_idx;
                    curr_dist = node_dist[uv_nd_idx];
                }
            }
            let next_curr_dist = node_dist[&curr_idx] + 1;

            /* If this node is the target node terminate the loop. */
            if curr_idx == self.dest_node_idx {
                return curr_dist;
            }

            /* For that node find all its possible neighbours. */
            for n_node in self.viable_moves(curr_idx) {
                /* Ignore the Blocked nodes */
                if self.node_status[n_node] == Status::Blocked {
                    continue;
                }

                /* Update the distances of the neighbours. */
                if next_curr_dist < node_dist[&n_node] {
                    node_dist.insert(n_node, next_curr_dist);
                };
            }
            unvisted_nodes.remove(&curr_idx);
        }
    }

    /// Calculate the fewest number of steps required to move the goal data to
    /// the node-x0-y0.
    pub fn fewest_steps_to_data(&self) -> u32 {
        return 1 + self.fewest_steps_to_dest() + 5 * self.node_locs[self.dest_node_idx].0;
    }
}

fn main() {
    let storage = ComputingGrid::new("./data/input.txt");
    println!("Part 1 = {}", storage.len_viable_swaps());
    println!("Part 2 = {}", storage.fewest_steps_to_data());
}
