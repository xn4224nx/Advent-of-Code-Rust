/*
 * --- Day 7: Recursive Circus ---
 *
 * Wandering further through the circuits of the computer, you come upon a tower
 * of programs that have gotten themselves into a bit of trouble. A recursive
 * algorithm has gotten out of hand, and now they're balanced precariously in a
 * large tower.
 *
 * One program at the bottom supports the entire tower. It's holding a large
 * disc, and on the disc are balanced several more sub-towers. At the bottom of
 * these sub-towers, standing on the bottom disc, are other programs, each
 * holding their own disc, and so on. At the very tops of these sub-sub-sub-...-
 * towers, many programs stand simply keeping the disc below them balanced but
 * with no disc of their own.
 *
 * You offer to help, but first you need to understand the structure of these
 * towers. You ask each program to yell out their name, their weight, and (if
 * they're holding a disc) the names of the programs immediately above them
 * balancing on that disc. You write this information down (your puzzle input).
 * Unfortunately, in their panic, they don't do this in an orderly fashion; by
 * the time you're done, you're not sure which program gave which information.
 *
 * For example, if your list is the following:
 *
 *      pbga (66)
 *      xhth (57)
 *      ebii (61)
 *      havc (66)
 *      ktlj (57)
 *      fwft (72) -> ktlj, cntj, xhth
 *      qoyq (66)
 *      padx (45) -> pbga, havc, qoyq
 *      tknk (41) -> ugml, padx, fwft
 *      jptl (61)
 *      ugml (68) -> gyxo, ebii, jptl
 *      gyxo (61)
 *      cntj (57)
 *
 * ...then you would be able to recreate the structure of the towers that looks
 * like this:
 *
 *                 gyxo
 *              /
 *         ugml - ebii
 *       /      \
 *      |         jptl
 *      |
 *      |         pbga
 *     /        /
 * tknk --- padx - havc
 *     \        \
 *      |         qoyq
 *      |
 *      |         ktlj
 *       \      /
 *         fwft - cntj
 *              \
 *                xhth
 *
 * In this example, tknk is at the bottom of the tower (the bottom program), and
 * is holding up ugml, padx, and fwft. Those programs are, in turn, holding up
 * other programs; in this example, none of those programs are holding up any
 * other programs, and are all the tops of their own towers. (The actual tower
 * balancing in front of you is much larger.)
 *
 * PART 1:  Before you're ready to help them, you need to make sure your
 *          information is correct. What is the name of the bottom program?
 */

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub name: String,
    pub weight: u32,
    pub above: Vec<String>,
    pub above_weight: u32,
}

impl Program {
    pub fn new(raw_program_info: &str) -> Self {
        let re_prog = Regex::new(r"([a-z]{4}) \(([0-9]+)\)").unwrap();

        /* Parse a program with otehrs directly above it. */
        let (above, prog_info) = if raw_program_info.contains("->") {
            let parts = raw_program_info.split_once("->").unwrap();
            (
                parts.1.split(",").map(|x| x.trim().to_string()).collect(),
                parts.0,
            )

        /* Parse a program with nothing above it. */
        } else {
            (Vec::new(), raw_program_info)
        };

        /* Attempt to extract the name and weight of the program. */
        let cap = re_prog.captures(prog_info).unwrap();
        let name = cap[1].to_string();
        let weight = cap[2].parse::<u32>().unwrap();

        return Program {
            name,
            weight,
            above,
            above_weight: 0,
        };
    }
}

pub struct ProgramStack {
    pub all: HashMap<String, Program>,
    pub bottom: String,
}

impl ProgramStack {
    pub fn new(data_file: &str) -> Self {
        let mut buffer = String::new();
        let mut all = HashMap::new();

        /* Load the file. */
        let file = File::open(data_file).unwrap();
        let mut f_ptr = BufReader::new(file);

        /* Iterate over the file line by line and parse. */
        while f_ptr.read_line(&mut buffer).unwrap() > 0 {
            let tmp_prog = Program::new(&buffer);
            let name = tmp_prog.name.clone();
            all.insert(name, tmp_prog);
            buffer.clear();
        }

        return ProgramStack {
            all,
            bottom: String::new(),
        };
    }

    pub fn find_bottom(&mut self) {
        let mut bottom_cand: Vec<&String> = self
            .all
            .keys()
            .filter(|&x| self.all.get(x).unwrap().above.len() > 0)
            .collect();

        /* The bottom program will be the one that isn't above any of them. */
        'cands: for prog_0 in bottom_cand.iter() {
            'checks: for prog_1 in bottom_cand.iter() {
                if prog_0 == prog_1 {
                    continue 'checks;
                };

                if self.all.get(*prog_1).unwrap().above.contains(*prog_0) {
                    continue 'cands;
                }
            }

            /* If all checks pass this must be at the bottom. */
            self.bottom = prog_0.to_string();
            return;
        }
    }
}

fn main() {}
