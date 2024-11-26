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
    pub fn assign(&mut self, bot: usize, value: u32) {}

    /// Conduct a bot giving action.
    pub fn give(
        &mut self,
        s_bot: usize,
        l_dest: usize,
        h_dest: usize,
        l_is_bot: bool,
        h_is_bot: bool,
    ) {
    }

    /// Run all instructions and find the index of the bot that compares the
    /// two supplied values.
    pub fn execute_all(&mut self, val_1: u32, val_2: u32) -> usize {
        0
    }
}

fn main() {}