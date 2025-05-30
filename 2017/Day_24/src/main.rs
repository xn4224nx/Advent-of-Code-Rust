/*
 * --- Day 24: Electromagnetic Moat ---
 *
 * The CPU itself is a large, black building surrounded by a bottomless pit.
 * Enormous metal tubes extend outward from the side of the building at regular
 * intervals and descend down into the void. There's no way to cross, but you
 * need to get inside.
 *
 * No way, of course, other than building a bridge out of the magnetic
 * components strewn about nearby.
 *
 * Each component has two ports, one on each end. The ports come in all
 * different types, and only matching types can be connected. You take an
 * inventory of the components by their port types (your puzzle input). Each
 * port is identified by the number of pins it uses; more pins mean a stronger
 * connection for your bridge. A 3/7 component, for example, has a type-3 port
 * on one side, and a type-7 port on the other.
 *
 * Your side of the pit is metallic; a perfect surface to connect a magnetic,
 * zero-pin port. Because of this, the first port you use must be of type 0. It
 * doesn't matter what type of port you end with; your goal is just to make the
 * bridge as strong as possible.
 *
 * The strength of a bridge is the sum of the port types in each component. For
 * example, if your bridge is made of components 0/3, 3/7, and 7/4, your bridge
 * has a strength of 0+3 + 3+7 + 7+4 = 24.
 *
 * For example, suppose you had the following components:
 *
 *      0/2
 *      2/2
 *      2/3
 *      3/4
 *      3/5
 *      0/1
 *      10/1
 *      9/10
 *
 * With them, you could make the following valid bridges:
 *
 *      0/1
 *      0/1--10/1
 *      0/1--10/1--9/10
 *      0/2
 *      0/2--2/3
 *      0/2--2/3--3/4
 *      0/2--2/3--3/5
 *      0/2--2/2
 *      0/2--2/2--2/3
 *      0/2--2/2--2/3--3/4
 *      0/2--2/2--2/3--3/5
 *
 * (Note how, as shown by 10/1, order of ports within a component doesn't
 * matter. However, you may only use each port on a component once.)
 *
 * Of these bridges, the strongest one is 0/1--10/1--9/10; it has a strength of
 * 0+1 + 1+10 + 10+9 = 31.
 *
 * PART 1:  What is the strength of the strongest bridge you can make with the
 *          components you have available?
 */

use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct BridgeBuilder {
    pub parts: Vec<(u32, u32)>,
}

impl BridgeBuilder {
    pub fn new(parts_file: &str) -> Self {
        let mut parts = Vec::new();
        let mut buffer = String::new();

        /* Open the file. */
        let file = File::open(parts_file).unwrap();
        let mut fp = BufReader::new(file);

        /* Iterate over the file line by line. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            let (first, last) = buffer.split_once("/").unwrap();
            parts.push((
                first.trim().parse::<u32>().unwrap(),
                last.trim().parse::<u32>().unwrap(),
            ));
            buffer.clear();
        }
        return BridgeBuilder { parts };
    }

    /// What is the strength of a particular bridge combination
    pub fn bridge_strength(&self, components: &Vec<usize>) -> u32 {
        return components
            .iter()
            .map(|x| self.parts[*x].0 + self.parts[*x].1)
            .sum();
    }

    /// Find the indexes of the starting components
    pub fn starting_components(&self) -> Vec<usize> {
        return (0..self.parts.len())
            .filter(|x| self.parts[*x].0 * self.parts[*x].1 == 0)
            .collect();
    }

    /// Using the index of parts extract a vector of the component magnitude
    pub fn extract_values(&self, components: &Vec<usize>) -> Vec<u32> {
        let mut bridge_mags = Vec::new();

        for (idx, comp_idx) in components.into_iter().enumerate() {
            let (val0, val1) = (self.parts[*comp_idx].0, self.parts[*comp_idx].1);

            /* The bridge must start with zero. */
            if idx == 0 {
                bridge_mags.push(min(val0, val1));
                bridge_mags.push(max(val0, val1));

            /* Otherwise the next number must be the same as the last one. */
            } else {
                bridge_mags.push(bridge_mags[bridge_mags.len() - 1]);

                if bridge_mags[bridge_mags.len() - 1] == val0 {
                    bridge_mags.push(val1);
                } else {
                    bridge_mags.push(val0);
                }
            }
        }
        return bridge_mags;
    }

    /// What combination of components makes the strongest bridge
    pub fn strongest_bridge(&self) -> u32 {
        let mut strg_value = 0;
        let mut curr_bridges: HashMap<Vec<usize>, Vec<u32>> = HashMap::new();

        /* The bridge can only start with particular values. */
        for start_idx in self.starting_components().drain(..) {
            curr_bridges.insert(vec![start_idx], self.extract_values(&vec![start_idx]));
        }
        /* Loop over existing bridges and try and create new ones. */
        while curr_bridges.len() > 0 {
            let mut new_bridges: HashMap<Vec<usize>, Vec<u32>> = HashMap::new();

            /* Check all the current bridges to see if any part can be added. */
            for (comp_idxes, elements) in curr_bridges.iter() {
                let last_ele = elements[elements.len() - 1];

                for part_idx in (0..self.parts.len()).filter(|x| !comp_idxes.contains(&x)) {
                    if last_ele == self.parts[part_idx].0 || last_ele == self.parts[part_idx].1 {
                        let new_bri = [comp_idxes.clone(), vec![part_idx]].concat();

                        /* Check if this bridge is the strongest seen so far. */
                        let bri_strg = self.bridge_strength(&new_bri);
                        if bri_strg > strg_value {
                            strg_value = bri_strg;
                        }

                        /* Save the new bridge for the next iteration. */
                        new_bridges.insert(new_bri.clone(), self.extract_values(&new_bri));
                    }
                }
            }
            curr_bridges = new_bridges;
        }
        return strg_value;
    }
}

fn main() {
    println!(
        "Part 1 = {}",
        BridgeBuilder::new("./data/input.txt").strongest_bridge()
    );
}
