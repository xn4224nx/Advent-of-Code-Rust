/*
 * --- Day 6: Probably a Fire Hazard ---
 *
 * Because your neighbors keep defeating you in
 * the holiday house decorating contest year after
 * year, you've decided to deploy one million
 * lights in a 1000x1000 grid.
 *
 * Furthermore, because you've been especially
 * nice this year, Santa has mailed you
 * instructions on how to display the ideal
 * lighting configuration.
 *
 * Lights in your grid are numbered from 0 to 999
 * in each direction; the lights at each corner
 * are at 0,0, 0,999, 999,999, and 999,0. The
 * instructions include whether to turn on, turn
 * off, or toggle various inclusive ranges given as
 * coordinate pairs. Each coordinate pair
 * represents opposite corners of a rectangle,
 * inclusive; a coordinate pair like 0,0 through
 * 2,2 therefore refers to 9 lights in a 3x3
 * square. The lights all start turned off.
 *
 * To defeat your neighbors this year, all you
 * have to do is set up your lights by doing the
 * instructions Santa sent you in order.
 *
 * PART 1:  After following the instructions, how
 *          many lights are lit?
 */

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, PartialEq)]
pub struct Instruct {
    pub cmd: Command,
    pub srt_x: u32,
    pub srt_y: u32,
    pub end_x: u32,
    pub end_y: u32,
}

/// Open the datafile and parse the instructions
pub fn read_instrucs(filepath: &str) -> Vec<Instruct> {
    let file = File::open(filepath).unwrap();
    let mut fp = BufReader::new(file);
    let re_num = Regex::new(r"\d+").unwrap();

    let mut instr = Vec::new();
    let mut buffer = String::new();

    /* Read the file line by line. */
    while fp.read_line(&mut buffer).unwrap() > 0 {
        /* Work out the command for this line. */
        let com = if buffer.contains("turn on") {
            Command::TurnOn
        } else if buffer.contains("turn off") {
            Command::TurnOff
        } else if buffer.contains("toggle") {
            Command::Toggle
        } else {
            panic!("Invalid line!");
        };

        /* Extract the coordinates of the command. */
        let nums: Vec<u32> = re_num
            .find_iter(&buffer)
            .map(|x| x.as_str().parse::<u32>().unwrap())
            .collect();

        if nums.len() != 4 {
            panic!("Invalid amounts of numbers on line!");
        }

        instr.push(Instruct {
            cmd: com,
            srt_x: nums[0],
            srt_y: nums[1],
            end_x: nums[2],
            end_y: nums[3],
        });

        buffer.clear();
    }

    return instr;
}

fn main() {
    let instruc = read_instrucs("./data/input.txt");
}
