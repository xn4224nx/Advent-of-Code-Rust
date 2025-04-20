/*
 * --- Day 14: Disk Defragmentation ---
 *
 * Suddenly, a scheduled job activates the system's disk defragmenter. Were the
 * situation different, you might sit and watch it for a while, but today, you
 * just don't have that kind of time. It's soaking up valuable system resources
 * that are needed elsewhere, and so the only option is to help it finish its
 * task as soon as possible.
 *
 * The disk in question consists of a 128x128 grid; each square of the grid is
 * either free or used. On this disk, the state of the grid is tracked by the
 * bits in a sequence of knot hashes.
 *
 * A total of 128 knot hashes are calculated, each corresponding to a single
 * row in the grid; each hash contains 128 bits which correspond to individual
 * grid squares. Each bit of a hash indicates whether that square is free (0)
 * or used (1).
 *
 * The hash inputs are a key string (your puzzle input), a dash, and a number
 * from 0 to 127 corresponding to the row. For example, if your key string were
 * flqrgnkx, then the first row would be given by the bits of the knot hash of
 * flqrgnkx-0, the second row from the bits of the knot hash of flqrgnkx-1, and
 * so on until the last row, flqrgnkx-127.
 *
 * The output of a knot hash is traditionally represented by 32 hexadecimal
 * digits; each of these digits correspond to 4 bits, for a total of 4 * 32 =
 * 128 bits. To convert to bits, turn each hexadecimal digit to its equivalent
 * binary value, high-bit first: 0 becomes 0000, 1 becomes 0001, e becomes
 * 1110, f becomes 1111, and so on; a hash that begins with a0c2017... in
 * hexadecimal would begin with 10100000110000100000000101110000... in binary.
 *
 * Continuing this process, the first 8 rows and columns for key flqrgnkx
 * appear as follows, using # to denote used squares, and . to denote free
 * ones:
 *
 * ##.#.#..-->
 * .#.#.#.#
 * ....#.#.
 * #.#.##.#
 * .##.#...
 * ##..#..#
 * .#...#..
 * ##.#.##.-->
 * |      |
 * V      V
 *
 * In this example, 8108 squares are used across the entire 128x128 grid.
 *
 * PART 1:  Given your actual key string, how many squares are used?
 */

mod hasher;
use std::collections::HashSet;

pub struct DiskDefrag {
    pub used: HashSet<(u8, u8)>,
}

impl DiskDefrag {
    pub fn new(seed: &str) -> Self {
        let mut used = HashSet::new();

        /* Generate the disk, row by row. */
        for row_idx in 0..128 {
            let message = format!("{}-{}", seed, row_idx);
            let digest =
                u128::from_str_radix(&hasher::KnotHash::new(&message).digest(), 16).unwrap();

            /* Iterate over the binaric number and add the used coordinates. */
            for (col_idx, bit) in (0..128).rev().map(|n| (digest >> n) & 1).enumerate() {
                if bit == 1 {
                    used.insert((col_idx as u8, row_idx as u8));
                }
            }
        }
        return DiskDefrag { used };
    }

    /// Get the coordinates of potentially adjacent points.
    pub fn adj_coords(&self, coord: &(u8, u8)) -> HashSet<(u8, u8)> {
        return HashSet::from([
            (coord.0.saturating_add(1), coord.1),
            (coord.0, coord.1.saturating_add(1)),
            (coord.0.saturating_sub(1), coord.1),
            (coord.0, coord.1.saturating_sub(1)),
        ]);
    }

    /// Count the number of connected regions in the disk defrag
    pub fn region_count(&self) -> u32 {
        let mut region_cnt = 0;
        let mut seen_sqrs: HashSet<(u8, u8)> = HashSet::new();

        for coord in &self.used {
            if seen_sqrs.contains(&coord) {
                continue;
            } else {
                seen_sqrs.insert(*coord);
                region_cnt += 1;
            };

            /* Find all the points that the original is connected to. */
            let mut next_coords: HashSet<(u8, u8)> = HashSet::from([*coord]);
            while !next_coords.is_empty() {
                let mut adj_coords: HashSet<(u8, u8)> = HashSet::new();

                for n_coord in next_coords {
                    /* Find all coordinates this one is connected to. */
                    for a_coord in &self.adj_coords(&n_coord) {
                        /* If the coordinate is used but unseen so far. */
                        if self.used.contains(&a_coord) && !seen_sqrs.contains(&a_coord) {
                            adj_coords.insert(*a_coord);
                            seen_sqrs.insert(*a_coord);
                        }
                    }
                }
                next_coords = adj_coords;
            }
        }
        return region_cnt;
    }
}

fn main() {
    let sys_disk = DiskDefrag::new("ffayrhll");
    println!("Part 1 = {}", sys_disk.used.len());
    println!("Part 2 = {}", sys_disk.region_count());
}
