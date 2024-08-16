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
 *
 * You just finish implementing your winning light
 * pattern when you realize you mistranslated
 * Santa's message from Ancient Nordic Elvish.
 *
 * The light grid you bought actually has
 * individual brightness controls; each light can
 * have a brightness of zero or more. The lights
 * all start at zero.
 *
 * The phrase turn on actually means that you
 * should increase the brightness of those lights
 * by 1.
 *
 * The phrase turn off actually means that you
 * should decrease the brightness of those lights
 * by 1, to a minimum of zero.
 *
 * The phrase toggle actually means that you
 * should increase the brightness of those lights
 * by 2.
 *
 * PART 2:  What is the total brightness of all
 *          lights combined after following
 *          Santa's instructions?
 */

use ndarray::{s, Array};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

static GRID_SIZE: usize = 1000;

#[derive(Debug, PartialEq)]
pub enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, PartialEq)]
pub struct Instruct {
    pub cmd: Command,
    pub srt_x: usize,
    pub srt_y: usize,
    pub end_x: usize,
    pub end_y: usize,
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
        let nums: Vec<usize> = re_num
            .find_iter(&buffer)
            .map(|x| x.as_str().parse::<usize>().unwrap())
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

/// Go through a set of light instructions and after all of them have executed
/// and count the number of lights that are on in the 1000x1000 grid.
/// True indicates on, false indicates off.
pub fn cnt_on_lights(instrucs: &Vec<Instruct>, init_state: bool) -> usize {
    let mut grid = Array::from_elem((GRID_SIZE, GRID_SIZE), init_state);

    for ins in instrucs.iter() {
        match ins.cmd {
            Command::TurnOn => grid
                .slice_mut(s![ins.srt_x..=ins.end_x, ins.srt_y..=ins.end_y])
                .fill(true),
            Command::TurnOff => grid
                .slice_mut(s![ins.srt_x..=ins.end_x, ins.srt_y..=ins.end_y])
                .fill(false),
            Command::Toggle => grid
                .slice_mut(s![ins.srt_x..=ins.end_x, ins.srt_y..=ins.end_y])
                .map_inplace(|x| *x = !*x),
        }
    }

    /* Count the number of trues in the grid and return them. */
    return grid.iter().filter(|x| **x).count();
}

/// Assume that the instructions modify the brightness not the status of the
/// lights.
pub fn sum_nord_lights(instrucs: &Vec<Instruct>, init_state: u32) -> u32 {
    let mut grid = Array::from_elem((GRID_SIZE, GRID_SIZE), init_state);

    for ins in instrucs.iter() {
        match ins.cmd {
            Command::TurnOn => grid
                .slice_mut(s![ins.srt_x..=ins.end_x, ins.srt_y..=ins.end_y])
                .map_inplace(|x| *x = x.saturating_add(1)),
            Command::TurnOff => grid
                .slice_mut(s![ins.srt_x..=ins.end_x, ins.srt_y..=ins.end_y])
                .map_inplace(|x| *x = x.saturating_sub(1)),
            Command::Toggle => grid
                .slice_mut(s![ins.srt_x..=ins.end_x, ins.srt_y..=ins.end_y])
                .map_inplace(|x| *x = x.saturating_add(2)),
        }
    }

    /* Sum the total brightness levels*/
    return grid.sum();
}

fn main() {
    let instruc = read_instrucs("./data/input.txt");
    println!("Part 1 = {}", cnt_on_lights(&instruc, false));
    println!("Part 2 = {}", sum_nord_lights(&instruc, 0));
}
