/*
 * --- Day 13: Packet Scanners ---
 *
 * You need to cross a vast firewall. The firewall consists of several layers,
 * each with a security scanner that moves back and forth across the layer. To
 * succeed, you must not be detected by a scanner.
 *
 * By studying the firewall briefly, you are able to record (in your puzzle
 * input) the depth of each layer and the range of the scanning area for the
 * scanner within it, written as depth: range. Each layer has a thickness of
 * exactly 1. A layer at depth 0 begins immediately inside the firewall; a
 * layer at depth 1 would start immediately after that.
 *
 * For example, suppose you've recorded the following:
 *
 * 0: 3
 * 1: 2
 * 4: 4
 * 6: 4
 *
 * This means that there is a layer immediately inside the firewall (with
 * range 3), a second layer immediately after that (with range 2), a third
 * layer which begins at depth 4 (with range 4), and a fourth layer which
 * begins at depth 6 (also with range 4). Visually, it might look like this:
 *
 *  0   1   2   3   4   5   6
 * [ ] [ ] ... ... [ ] ... [ ]
 * [ ] [ ]         [ ]     [ ]
 * [ ]             [ ]     [ ]
 *                 [ ]     [ ]
 *
 * Within each layer, a security scanner moves back and forth within its
 * range. Each security scanner starts at the top and moves down until it
 * reaches the bottom, then moves up until it reaches the top, and repeats. A
 * security scanner takes one picosecond to move one step.
 *
 * Your plan is to hitch a ride on a packet about to move through the
 * firewall. The packet will travel along the top of each layer, and it moves
 * at one layer per picosecond. Each picosecond, the packet moves one layer
 * forward (its first move takes it into layer 0), and then the scanners move
 * one step. If there is a scanner at the top of the layer as your packet
 * enters it, you are caught. (If a scanner moves into the top of its layer
 * while you are there, you are not caught: it doesn't have time to notice you
 * before you leave.)
 *
 * In this situation, you are caught in layers 0 and 6, because your packet
 * entered the layer when its scanner was at the top when you entered it. You
 * are not caught in layer 1, since the scanner moved into the top of the
 * layer once you were already there.
 *
 * The severity of getting caught on a layer is equal to its depth multiplied
 * by its range. (Ignore layers in which you do not get caught.) The severity
 * of the whole trip is the sum of these values. In the example above, the
 * trip severity is 0*3 + 6*4 = 24.
 *
 * PART 1:  Given the details of the firewall you've recorded, if you leave
 *          immediately, what is the severity of your whole trip?
 *
 * Now, you need to pass through the firewall without being caught - easier
 * said than done.
 *
 * You can't control the speed of the packet, but you can delay it any number
 * of picoseconds. For each picosecond you delay the packet before beginning
 * your trip, all security scanners move one step. You're not in the firewall
 * during this time; you don't enter layer 0 until you stop delaying the
 * packet.
 *
 * Because all smaller delays would get you caught, the fewest number of
 * picoseconds you would need to delay to get through safely is 10.
 *
 * PART 2:  What is the fewest number of picoseconds that you need to delay
 *          the packet to pass through the firewall without being caught?
 */

use std::collections::HashMap;
use std::fs::read_to_string;

pub struct Firewall {
    pub info: HashMap<u32, u32>,
}

impl Firewall {
    pub fn new(data_file: &str) -> Self {
        Firewall {
            info: read_to_string(data_file)
                .unwrap()
                .lines()
                .map(|x| x.split_once(": ").unwrap())
                .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
                .collect(),
        }
    }

    /// Detect if after a certain time is at the top of the stack
    pub fn scanner_at_top(&self, scanner: u32, time_passed: u32) -> bool {
        let height = *self.info.get(&scanner).unwrap();
        let offset = time_passed % ((height - 1) * 2);

        let position = if offset > height - 1 {
            2 * (height - 1) - offset
        } else {
            offset
        };
        return position == 0;
    }

    /// Calculate the total severity from a packet crossing the firewall
    pub fn trip_severity(&self) -> u32 {
        let mut total_severity = 0;

        /* Check to see if any of the scanners detect the packet. */
        for scanner_idx in self.info.keys() {
            if self.scanner_at_top(*scanner_idx, *scanner_idx) {
                total_severity += *scanner_idx * *self.info.get(&scanner_idx).unwrap();
            }
        }
        return total_severity;
    }

    /// Find the time delay that allows the packet to pass through the firewall
    /// without being detected by any of the scanner.
    pub fn find_clear_path(&self) -> u32 {
        let mut delay = 0;

        /* Loop until no packet is detected. */
        't_del: loop {
            for scanner_idx in self.info.keys() {
                if self.scanner_at_top(*scanner_idx, *scanner_idx + delay) {
                    delay += 1;
                    continue 't_del;
                }
            }
            return delay;
        }
    }
}

fn main() {
    println!(
        "Part 1 = {}\nPart 2 = {}\n",
        Firewall::new("./data/input.txt").trip_severity(),
        Firewall::new("./data/input.txt").find_clear_path()
    );
}
