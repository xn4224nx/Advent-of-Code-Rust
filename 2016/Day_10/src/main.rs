/*
 * --- Day 10: Balance Bots ---
 *
 * You come upon a factory in which many robots are zooming around handing small
 * microchips to each other.
 *
 * Upon closer examination, you notice that each bot only proceeds when it has
 * two microchips, and once it does, it gives each one to a different bot or
 * puts it in a marked "output" bin. Sometimes, bots take microchips from
 * "input" bins, too.
 *
 * Inspecting one of the microchips, it seems like they each contain a single
 * number; the bots must use some logic to decide what to do with each chip. You
 * access the local control computer and download the bots' instructions (your
 * puzzle input).
 *
 * Some of the instructions specify that a specific-valued microchip should be
 * given to a specific bot; the rest of the instructions indicate what a given
 * bot should do with its lower-value or higher-value chip.
 *
 * PART 1:  Based on your instructions, what is the number of the bot that is
 *          responsible for comparing value-61 microchips with value-17
 *          microchips?
 *
 * PART 2:  What do you get if you multiply together the values of one chip in
 *          each of outputs 0, 1, and 2?
 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub enum Instruc {
    Assign(usize, u32),
    Give(usize, usize, usize, bool, bool),
}

pub struct Factory {
    pub instructions: Vec<Instruc>,
    pub bots: Vec<Vec<u32>>,
    pub outputs: Vec<Vec<u32>>,
    pub used_instrs: Vec<bool>,
    pub data_file: String,
}

impl Factory {
    pub fn new(file_path: &str) -> Self {
        return Factory {
            instructions: Vec::new(),
            bots: Vec::new(),
            outputs: Vec::new(),
            used_instrs: Vec::new(),
            data_file: file_path.to_string(),
        };
    }

    /// Read the instructions and parse them, then intilise the bot & output
    /// storage based on the maximum mentioned bot & output index.
    pub fn initialise(&mut self) {
        let mut buffer = String::new();
        let mut max_bot_idx = 0;
        let mut max_out_idx = 0;

        /* Define the Regexs to capture the data */
        let re_assign = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
        let re_give =
            Regex::new(r"bot (\d+) gives low to (output|bot) (\d+) and high to (output|bot) (\d+)")
                .unwrap();

        /* Open the file. */
        let file = File::open(&self.data_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Iterate over the file line by line. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            /* Test for the value assignment command. */
            if let Some(caps) = re_assign.captures(&buffer) {
                let bot_idx = caps[2].parse::<usize>().unwrap();

                self.instructions
                    .push(Instruc::Assign(bot_idx, caps[1].parse::<u32>().unwrap()));

                /* Check for new highest bot index. */
                if bot_idx > max_bot_idx {
                    max_bot_idx = bot_idx;
                };

            /* Test for the value give command */
            } else if let Some(caps) = re_give.captures(&buffer) {
                let src_bot = caps[1].parse::<usize>().unwrap();
                let l_dest = caps[3].parse::<usize>().unwrap();
                let h_dest = caps[5].parse::<usize>().unwrap();
                let l_is_bot = caps[2] == *"bot";
                let h_is_bot = caps[4] == *"bot";

                self.instructions
                    .push(Instruc::Give(src_bot, l_dest, h_dest, l_is_bot, h_is_bot));

                /* Check for new high indexes */
                if l_is_bot && l_dest > max_bot_idx {
                    max_bot_idx = l_dest;
                } else if !l_is_bot && l_dest > max_out_idx {
                    max_out_idx = l_dest;
                };

                if h_is_bot && h_dest > max_bot_idx {
                    max_bot_idx = h_dest;
                } else if !h_is_bot && h_dest > max_out_idx {
                    max_out_idx = h_dest;
                };
            };
            buffer.clear();
        }

        /* Initialise bot storage. */
        self.bots = vec![Vec::new(); max_bot_idx + 1];

        /* Initialise output storage. */
        self.outputs = vec![Vec::new(); max_out_idx + 1];

        /* Set all assignments as unused. */
        self.used_instrs = vec![false; self.instructions.len()];
    }

    /// Assign a value to a bot.
    pub fn assign(&mut self, bot_idx: usize, value: u32) {
        self.bots[bot_idx].push(value)
    }

    /// Conduct a bot giving action.
    pub fn give(
        &mut self,
        s_bot: usize,
        l_dest: usize,
        h_dest: usize,
        l_is_bot: bool,
        h_is_bot: bool,
    ) {
        let min_val = *self.bots[s_bot].iter().min().unwrap();
        let max_val = *self.bots[s_bot].iter().max().unwrap();

        /* Move the low value. */
        if l_is_bot {
            self.bots[l_dest].push(min_val);
        } else {
            self.outputs[l_dest].push(min_val);
        }

        /* Move the high value. */
        if h_is_bot {
            self.bots[h_dest].push(max_val);
        } else {
            self.outputs[h_dest].push(max_val);
        }

        /* Empty the source bot. */
        self.bots[s_bot].clear();
    }

    /// Run all instructions and find the index of the bot that compares the
    /// two supplied values.
    pub fn execute_all(&mut self, val_1: u32, val_2: u32) -> usize {
        let mut comp_bot = 0;

        /* Execute the instructions in order until all are used up. */
        while self.used_instrs.contains(&false) {
            for instr_idx in 0..self.instructions.len() {
                if self.used_instrs[instr_idx] {
                    continue;
                }

                match self.instructions[instr_idx] {
                    Instruc::Assign(as_idx, as_val) => {
                        self.assign(as_idx, as_val);
                    }

                    Instruc::Give(s_idx, l_idx, h_idx, l_bot, h_bot) => {
                        if self.bots[s_idx].len() != 2 {
                            continue;
                        }

                        /* Determine if the value comparison happens  */
                        if (self.bots[s_idx][0] == val_1 && self.bots[s_idx][1] == val_2)
                            || (self.bots[s_idx][1] == val_1 && self.bots[s_idx][0] == val_2)
                        {
                            comp_bot = s_idx;
                        }
                        self.give(s_idx, l_idx, h_idx, l_bot, h_bot);
                    }
                }

                /* This instruction should not be run again. */
                self.used_instrs[instr_idx] = true;
            }
        }
        return comp_bot;
    }

    /// Find the product of the first three output bin's first value.
    pub fn output_prod(&self) -> u32 {
        return self.outputs[0][0] * self.outputs[1][0] * self.outputs[2][0];
    }
}

fn main() {
    let mut b_factory = Factory::new("./data/input.txt");
    b_factory.initialise();
    println!("Part 1 = {}", b_factory.execute_all(61, 17));
    println!("Part 2 = {}", b_factory.output_prod());
}
