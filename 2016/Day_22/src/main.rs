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
 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Debug)]
pub enum Status {
    Blocked,
    Empty,
    Full,
}

pub struct ComputingGrid {
    pub node_locs: Vec<(u32, u32)>,
    pub start_node_status: Vec<Status>,
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
        let mut empt_node_idx: Option<usize> = None;
        let mut empt_node_cap: Option<u32> = None;

        for node_idx in 0..node_locs.len() {
            if usages[node_idx] == 0 {
                empt_node_idx = Some(node_idx);
                empt_node_cap = Some(capacities[node_idx]);
            }
        }

        if empt_node_cap.is_none() || empt_node_cap.is_none() {
            panic!("No empty node was found in the data!")
        }
        let empt_node_idx = empt_node_idx.unwrap();
        let empt_node_cap = empt_node_cap.unwrap();

        /* Label the nodes. */
        let start_node_status = usages
            .iter()
            .map(|x| match x {
                0 => Status::Empty,
                d if *d <= empt_node_cap => Status::Full,
                d if *d > empt_node_cap => Status::Blocked,
                _ => panic!("{x} is not covered!"),
            })
            .collect();

        return ComputingGrid {
            node_locs,
            start_node_status,
        };
    }

    pub fn len_viable_swaps(&self, state: &Vec<Status>) -> usize {
        0
    }
}

fn main() {}
