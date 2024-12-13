/*
 * --- Day 11: Radioisotope Thermoelectric Generators ---
 *
 * You come upon a column of four floors that have been entirely sealed off from
 * the rest of the building except for a small dedicated lobby. There are some
 * radiation warnings and a big sign which reads "Radioisotope Testing
 * Facility".
 *
 * According to the project status board, this facility is currently being used
 * to experiment with Radioisotope Thermoelectric Generators (RTGs, or simply
 * "generators") that are designed to be paired with specially-constructed
 * microchips. Basically, an RTG is a highly radioactive rock that generates
 * electricity through heat.
 *
 * The experimental RTGs have poor radiation containment, so they're dangerously
 * radioactive. The chips are prototypes and don't have normal radiation
 * shielding, but they do have the ability to generate an electromagnetic
 * radiation shield when powered. Unfortunately, they can only be powered by
 * their corresponding RTG. An RTG powering a microchip is still dangerous to
 * other microchips.
 *
 * In other words, if a chip is ever left in the same area as another RTG, and
 * it's not connected to its own RTG, the chip will be fried. Therefore, it is
 * assumed that you will follow procedure and keep chips connected to their
 * corresponding RTG when they're in the same room, and away from other RTGs
 * otherwise.
 *
 * These microchips sound very interesting and useful to your current
 * activities, and you'd like to try to retrieve them. The fourth floor of the
 * facility has an assembling machine which can make a self-contained, shielded
 * computer for you to take with you - that is, if you can bring it all of the
 * RTGs and microchips.
 *
 * Within the radiation-shielded part of the facility (in which it's safe to
 * have these pre-assembly RTGs), there is an elevator that can move between the
 * four floors. Its capacity rating means it can carry at most yourself and two
 * RTGs or microchips in any combination. (They're rigged to some heavy
 * diagnostic equipment - the assembling machine will detach it for you.) As a
 * security measure, the elevator will only function if it contains at least one
 * RTG or microchip. The elevator always stops on each floor to recharge, and
 * this takes long enough that the items within it and the items on that floor
 * can irradiate each other. (You can prevent this if a Microchip and its
 * Generator end up on the same floor in this way, as they can be connected
 * while the elevator is recharging.)
 *
 * You make some notes of the locations of each component of interest (your
 * puzzle input). Before you don a hazmat suit and start moving things around,
 * you'd like to have an idea of what you need to do.
 *
 * When you enter the containment area, you and the elevator will start on the
 * first floor.
 *
 * PART 1:  In your situation, what is the minimum number of steps required to
 *          bring all of the objects to the fourth floor?
 */

use itertools::Itertools;
use regex::Regex;
use std::collections::{BTreeMap, HashSet};
use std::fs::read_to_string;

static MAX_LVL: u8 = 3;

/// Open a file and parse the state of generators and microchips in a building
pub fn read_generator_state(data_file: &str) -> Vec<u8> {
    let mut gens: BTreeMap<&str, u8> = BTreeMap::new();
    let mut mcrs: BTreeMap<&str, u8> = BTreeMap::new();

    let pat_mcr = Regex::new(r"([A-Za-z]+)-compatible microchip").unwrap();
    let pat_gen = Regex::new(r"([A-Za-z]+) generator").unwrap();

    /* Open the file. */
    let whole_file = read_to_string(data_file).unwrap();

    /* Read the file line by line and find the elements. */
    for (idx, line) in whole_file.lines().enumerate() {
        /* Extract the generators. */
        for (_, [ele]) in pat_gen.captures_iter(line).map(|x| x.extract()) {
            gens.insert(ele, idx as u8);
        }

        /* Extract the microchips. */
        for (_, [ele]) in pat_mcr.captures_iter(line).map(|x| x.extract()) {
            mcrs.insert(ele, idx as u8);
        }
    }

    /* Construct the state vector. */
    let mut state = vec![0];

    /* Add the elements in alphabetical order. */
    for (_, ele_lvl) in &gens {
        state.push(*ele_lvl);
    }
    for (_, ele_lvl) in &mcrs {
        state.push(*ele_lvl);
    }

    return state;
}

/// Check if a configuration of generators and microchips would be safe
pub fn is_state_safe(state: &Vec<u8>) -> bool {
    let num_ele: usize = (state.len() - 1) / 2;

    /* Check the validity of each element in turn. */
    for ele_idx in 0..num_ele {
        let gen_lvl = state[1 + ele_idx];
        let mcr_lvl = state[1 + ele_idx + num_ele];

        /* If an microchip is on the same level as its generator its safe. */
        if gen_lvl == mcr_lvl {
            continue;
        };

        /* If a generator is on the level of the microchip it's not safe. */
        for gen_idx in 0..num_ele {
            let oth_gen_lvl = state[1 + gen_idx];

            if oth_gen_lvl == mcr_lvl {
                return false;
            };
        }
    }

    /* Otherwise the state is safe. */
    return true;
}

/// Find out if all the elements in a state have reached the top floor
pub fn is_state_finished(state: &Vec<u8>) -> bool {
    return state.iter().all(|&x| x == MAX_LVL);
}

/// Create a list of the next possible states from one original state
pub fn create_next_states(state: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut nxt_states = Vec::new();

    /* Move one element */
    for ele_idx in 1..state.len() {
        let lvl = state[ele_idx];

        /* Check that the elevator is on the same level */
        if lvl != state[0] {
            continue;
        };

        /* Move up */
        if lvl < MAX_LVL {
            let mut new_state = state.clone();
            new_state[ele_idx] += 1;
            new_state[0] += 1;
            if is_state_safe(&new_state) {
                nxt_states.push(new_state);
            }
        }

        /* Move down */
        if lvl > 0 {
            let mut new_state = state.clone();
            new_state[ele_idx] -= 1;
            new_state[0] -= 1;
            if is_state_safe(&new_state) {
                nxt_states.push(new_state);
            }
        }
    }

    /* Move two elements */
    for ele_idx_comb in (1..state.len()).combinations(2) {
        let lvl_0 = state[ele_idx_comb[0]];
        let lvl_1 = state[ele_idx_comb[1]];

        /* Only move if both elements are on the same level with the elevator. */
        if lvl_0 != lvl_1 || lvl_1 != state[0] {
            continue;
        };

        /* Move up */
        if lvl_0 < MAX_LVL {
            let mut new_state = state.clone();
            new_state[ele_idx_comb[0]] += 1;
            new_state[ele_idx_comb[1]] += 1;
            new_state[0] += 1;
            if is_state_safe(&new_state) {
                nxt_states.push(new_state);
            }
        }

        /* Move down */
        if lvl_0 > 0 {
            let mut new_state = state.clone();
            new_state[ele_idx_comb[0]] -= 1;
            new_state[ele_idx_comb[1]] -= 1;
            new_state[0] -= 1;
            if is_state_safe(&new_state) {
                nxt_states.push(new_state);
            }
        }
    }
    return nxt_states;
}

/// Determine how many moves it takes to move all the objects to the top floor
pub fn find_min_move_to_top(state: &Vec<u8>) -> usize {
    let mut seen_states: HashSet<Vec<u8>> = vec![state.clone()].into_iter().collect();
    let mut curr_states: HashSet<Vec<u8>> = vec![state.clone()].into_iter().collect();
    let mut num_moves = 0;

    loop {
        let mut nxt_states: HashSet<Vec<u8>> = HashSet::new();

        /* Generate all the next possible states. */
        for c_state in &curr_states {
            for n_state in create_next_states(c_state).iter() {
                if seen_states.contains(n_state) {
                    continue;
                } else {
                    seen_states.insert(n_state.clone());
                };

                /* See if the end has been reached */
                if is_state_finished(n_state) {
                    return num_moves + 1;
                };
                nxt_states.insert(n_state.clone());
            }
        }

        /* Prepare for the next loop iteration. */
        curr_states = nxt_states;
        num_moves += 1;
    }
}

fn main() {}
