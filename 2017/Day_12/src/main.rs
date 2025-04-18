/*
 * --- Day 12: Digital Plumber ---
 *
 * Walking along the memory banks of the stream, you find a small village that
 * is experiencing a little confusion: some programs can't communicate with
 * each other.
 *
 * Programs in this village communicate using a fixed system of pipes.
 * Messages are passed between programs using these pipes, but most programs
 * aren't connected to each other directly. Instead, programs pass messages
 * between each other until the message reaches the intended recipient.
 *
 * For some reason, though, some of these messages aren't ever reaching their
 * intended recipient, and the programs suspect that some pipes are missing.
 * They would like you to investigate.
 *
 * You walk through the village and record the ID of each program and the IDs
 * with which it can communicate directly (your puzzle input). Each program
 * has one or more programs with which it can communicate, and these pipes are
 * bidirectional; if 8 says it can communicate with 11, then 11 will say it
 * can communicate with 8.
 *
 * You need to figure out how many programs are in the group that contains
 * program ID 0.
 *
 * For example, suppose you go door-to-door like a travelling salesman and
 * record the following list:
 *
 * 0 <-> 2
 * 1 <-> 1
 * 2 <-> 0, 3, 4
 * 3 <-> 2, 4
 * 4 <-> 2, 3, 6
 * 5 <-> 6
 * 6 <-> 4, 5
 *
 * In this example, the following programs are in the group that contains
 * program ID 0:
 *
 *      -   Program 0 by definition.
 *      -   Program 2, directly connected to program 0.
 *      -   Program 3 via program 2.
 *      -   Program 4 via program 2.
 *      -   Program 5 via programs 6, then 4, then 2.
 *      -   Program 6 via programs 4, then 2.
 *
 * Therefore, a total of 6 programs are in this group; all but program 1,
 * which has a pipe that connects it to itself.
 *
 * PART 1:  How many programs are in the group that contains program ID 0?
 */

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct ProgramVillage {
    pub links: HashMap<u16, HashSet<u16>>,
}

impl ProgramVillage {
    pub fn new(data_file: &str) -> Self {
        let mut buffer = String::new();
        let mut links = HashMap::new();

        /* Open the data file. */
        let file = File::open(data_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Read the file line by line. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            let (l_prog, r_prog_lnks) = buffer.split_once(" <-> ").unwrap();

            /* Parse the program ids. */
            let l_prog = l_prog.parse::<u16>().unwrap();

            let r_prog_lnks = if r_prog_lnks.contains(",") {
                r_prog_lnks
                    .split(",")
                    .map(|x| x.trim().parse::<u16>().unwrap())
                    .collect()
            } else {
                vec![r_prog_lnks.trim().parse::<u16>().unwrap()]
            };

            /* Add the main id into the record. */
            if !links.contains_key(&l_prog) {
                links.insert(l_prog, HashSet::new());
            };

            /* Record the program links in both programs records. */
            for prog_id in r_prog_lnks.into_iter() {
                if !links.contains_key(&prog_id) {
                    links.insert(prog_id, HashSet::new());
                };

                /* A program cannot link to itself. */
                if prog_id != l_prog {
                    if let Some(x) = links.get_mut(&prog_id) {
                        x.insert(l_prog);
                    };

                    if let Some(x) = links.get_mut(&l_prog) {
                        x.insert(prog_id);
                    };
                }
            }
            buffer.clear();
        }
        return ProgramVillage { links };
    }

    /// Calculate the group a specific program is in
    pub fn full_prog_group(&self, prog_id: u16) -> HashSet<u16> {
        let mut seen_progs = HashSet::new();
        let mut progs_to_check = HashSet::from([prog_id]);

        while !progs_to_check.is_empty() {
            let mut nxt_progs = HashSet::new();

            for p_prog in &progs_to_check {
                seen_progs.insert(*p_prog);

                /* Find every program this one is connected to. */
                for n_prog in self.links.get(&p_prog).unwrap() {
                    if !seen_progs.contains(n_prog) {
                        nxt_progs.insert(*n_prog);
                        seen_progs.insert(*n_prog);
                    };
                }
            }
            progs_to_check = nxt_progs;
        }
        return seen_progs;
    }
}

fn main() {}
