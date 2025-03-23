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
 *
 * The programs explain the situation: they can't get down. Rather, they could
 * get down, if they weren't expending all of their energy trying to keep the
 * tower balanced. Apparently, one program has the wrong weight, and until it's
 * fixed, they're stuck here.
 *
 * For any program holding a disc, each program standing on that disc forms a
 * sub-tower. Each of those sub-towers are supposed to be the same weight, or
 * the disc itself isn't balanced. The weight of a tower is the sum of the
 * weights of the programs in that tower.
 *
 * In the example above, this means that for ugml's disc to be balanced, gyxo,
 * ebii, and jptl must all have the same weight, and they do: 61.
 *
 * However, for tknk to be balanced, each of the programs standing on its disc
 * and all programs above it must each match. This means that the following sums
 * must all be the same:
 *
 *      -   ugml + (gyxo + ebii + jptl) = 68 + (61 + 61 + 61) = 251
 *      -   padx + (pbga + havc + qoyq) = 45 + (66 + 66 + 66) = 243
 *      -   fwft + (ktlj + cntj + xhth) = 72 + (57 + 57 + 57) = 243
 *
 * As you can see, tknk's disc is unbalanced: ugml's stack is heavier than the
 * other two. Even though the nodes above ugml are balanced, ugml itself is too
 * heavy: it needs to be 8 units lighter for its stack to weigh 243 and keep the
 * towers balanced. If this change were made, its weight would be 60.
 *
 * PART 2:  Given that exactly one program is the wrong weight, what would its
 *          weight need to be to balance the entire tower?
 */

use regex::Regex;
use std::collections::{HashMap, HashSet};
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
        let re_prog = Regex::new(r"([a-z]+) \(([0-9]+)\)").unwrap();

        /* Parse a program with others directly above it. */
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

#[derive(Debug)]
pub struct ProgramStack {
    pub all: HashMap<String, Program>,
    pub bottom: String,
    pub unbalanced: String,
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
            unbalanced: String::new(),
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

    pub fn calc_above_weights(&mut self) {
        let mut uncalc_progs: HashSet<String> = self
            .all
            .keys()
            .filter(|&x| self.all.get(x).unwrap().above.len() > 0)
            .map(|x| x.clone())
            .collect();

        while uncalc_progs.len() > 0 {
            let mut rm_progs = Vec::new();

            /* Find programs that have everything above them calculated. */
            'prog_calc: for prog_name in uncalc_progs.iter() {
                let mut abv_sum = 0;

                /* Determine if all the programs above this one are calculated. */
                for abv_prog in self.all.get(prog_name).unwrap().above.iter() {
                    abv_sum += self.all.get(abv_prog).unwrap().weight;
                    abv_sum += self.all.get(abv_prog).unwrap().above_weight;

                    if uncalc_progs.contains(abv_prog) {
                        continue 'prog_calc;
                    }
                }

                /* Set the above value for this program. */
                self.all
                    .entry(prog_name.to_string())
                    .and_modify(|x| x.above_weight = abv_sum);

                /* Ensure the program is not calculated again. */
                rm_progs.push(prog_name.clone());
            }

            /* Remove the programs that were calculated. */
            for prog_name in rm_progs.iter() {
                uncalc_progs.remove(prog_name);
            }
        }
    }

    pub fn balance_stack(&mut self) -> u32 {
        let mut record_of_weights: Vec<u32> = Vec::new();
        let mut curr_prog = self.bottom.clone();

        /* Starting at the bottom move upwards. */
        loop {
            /* Calculate the weights of the programs above this one. */
            let mut abv_weights: HashMap<u32, Vec<&String>> = HashMap::new();

            /* Group the above programs by weight. */
            for prog_name in self.all.get(&curr_prog).unwrap().above.iter() {
                let tmp_prog = self.all.get(prog_name).unwrap();
                let prog_weight = tmp_prog.weight + tmp_prog.above_weight;
                abv_weights
                    .entry(prog_weight)
                    .and_modify(|x| x.push(prog_name))
                    .or_insert(vec![prog_name]);
            }

            /* If all values are equal this is the effected node. */
            if abv_weights.len() == 1 {
                return record_of_weights.last().unwrap()
                    - self.all.get(&curr_prog).unwrap().above_weight;
            }

            /* Otherwise select the unbalanced program above. */
            curr_prog = abv_weights
                .values()
                .min_by_key(|x| x.len())
                .unwrap()
                .first()
                .unwrap()
                .to_string();

            /* Save the weight of the above nodes */
            record_of_weights.push(*abv_weights.iter().max_by_key(|x| x.1.len()).unwrap().0)
        }
    }
}

fn main() {
    let mut rec_circus = ProgramStack::new("./data/input.txt");
    rec_circus.find_bottom();
    rec_circus.calc_above_weights();
    let new_program_weight = rec_circus.balance_stack();
    println!(
        "Part 1 = {}\nPart 2 = {}\n",
        rec_circus.bottom, new_program_weight
    );
}
