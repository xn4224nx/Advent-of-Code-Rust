/*
 * --- Day 3: Spiral Memory ---
 *
 * You come across an experimental new kind of memory stored on an infinite two-
 * dimensional grid.
 *
 * Each square on the grid is allocated in a spiral pattern starting at a
 * location marked 1 and then counting up while spiraling outward. For example,
 * the first few squares are allocated like this:
 *
 *      17  16  15  14  13
 *      18   5   4   3  12
 *      19   6   1   2  11
 *      20   7   8   9  10
 *      21  22  23---> ...
 *
 * While this is very space-efficient (no squares are skipped), requested data
 * must be carried back to square 1 (the location of the only access port for
 * this memory system) by programs that can only move up, down, left, or right.
 * They always take the shortest path: the Manhattan Distance between the
 * location of the data and square 1.
 *
 * For example:
 *
 *      -   Data from square 1 is carried 0 steps, since it's at the access
 *          port.
 *
 *      -   Data from square 12 is carried 3 steps, such as: down, left, left.
 *
 *      -   Data from square 23 is carried only 2 steps: up twice.
 *
 *      -   Data from square 1024 must be carried 31 steps.
 *
 * PART 1:  How many steps are required to carry the data from the square
 *          identified in your puzzle input all the way to the access port?
 *
 * As a stress test on the system, the programs here clear the grid and then
 * store the value 1 in square 1. Then, in the same allocation order as shown
 * above, they store the sum of the values in all adjacent squares, including
 * diagonals.
 *
 * So, the first few squares' values are chosen as follows:
 *
 *      -   Square 1 starts with the value 1.
 *
 *      -   Square 2 has only one adjacent filled square (with value 1), so it
 *          also stores 1.
 *
 *      -   Square 3 has both of the above squares as neighbors and stores the
 *          sum of their values, 2.
 *
 *      -   Square 4 has all three of the aforementioned squares as neighbors
 *          and stores the sum of their values, 4.
 *
 *      -   Square 5 only has the first and fourth squares as neighbors, so it
 *          gets the value 5.
 *
 * Once a square is written, its value does not change. Therefore, the first
 * few squares would receive the following values:
 *
 *      147  142  133  122   59
 *      304    5    4    2   57
 *      330   10    1    1   54
 *      351   11   23   25   26
 *      362  747  806--->   ...
 *
 * PART 2:  What is the first value written that is larger than your puzzle
 *          input?
 */

use std::collections::HashMap;

/// Calculate the coordinates of step in the spiral
pub fn coords_in_spiral(step: i32) -> (i32, i32) {
    /* Calculate the ring characteristics. */
    let ring = (step as f64).sqrt().ceil() as i32 / 2;
    let ring_side_len = 2 * ring + 1;
    let ring_final_val = ring_side_len.pow(2);

    /* Calculate the value at each of the corners of the ring */
    let bot_left = ring_final_val - (ring_side_len - 1);
    let top_left = ring_final_val - 2 * (ring_side_len - 1);
    let top_rght = ring_final_val - 3 * (ring_side_len - 1);

    /* Test to see if this step is on a corner */
    return if step == ring_final_val {
        (ring, -ring)
    } else if step == bot_left {
        (-ring, -ring)
    } else if step == top_left {
        (-ring, ring)
    } else if step == top_rght {
        (ring, ring)

    /* Bottow row points */
    } else if ring_final_val > step && step > bot_left {
        (step - bot_left - ring_side_len / 2, -ring)

    /* Top row points */
    } else if top_left > step && step > top_rght {
        ((top_rght + ring_side_len / 2) - step, ring)

    /* Left column points */
    } else if bot_left > step && step > top_left {
        (-ring, (top_left + ring_side_len / 2) - step)

    /* Right column points */
    } else if top_rght > step {
        (ring, step - top_rght + ring_side_len / 2)
    } else {
        panic!("Step {} could not be placed at a point.", step);
    };
}

/// Count the manhattan distance from this step to the centre
pub fn moves_to_exit(step: i32) -> i32 {
    let (x_coord, y_coord) = coords_in_spiral(step);
    return x_coord.abs() + y_coord.abs();
}

/// Assuming the squares start at one and have their value decided by the sum
/// of already created adjacent squares. What is the first value created larger
/// than the max value? This corresponds to the OEIS sequence A141481.
/// https://oeis.org/A141481
pub fn find_first_gt_max_val(max_value: i32) -> i32 {
    let mut step_idx = 2;
    let mut pnt_vals = HashMap::from([((0, 0), 1)]);

    /* Generate the values for each step. */
    loop {
        let pnt = coords_in_spiral(step_idx);

        /* Define the possible adjacent points to the current one. */
        let possible_adj_pnts = vec![
            (pnt.0 + 1, pnt.1 + 1),
            (pnt.0, pnt.1 + 1),
            (pnt.0 - 1, pnt.1 + 1),
            (pnt.0 + 1, pnt.1),
            (pnt.0 - 1, pnt.1),
            (pnt.0 + 1, pnt.1 - 1),
            (pnt.0, pnt.1 - 1),
            (pnt.0 - 1, pnt.1 - 1),
        ];

        /* Calculate the value for this point by summing all adjacent steps. */
        let mut pnt_sum = 0;
        for adj_pnt in possible_adj_pnts.iter() {
            match pnt_vals.get(adj_pnt) {
                Some(value) => pnt_sum += value,
                None => {}
            };
        }

        /* Check for a solution */
        if pnt_sum > max_value {
            return pnt_sum;
        } else {
            pnt_vals.insert(pnt, pnt_sum);
            step_idx += 1;
        }
    }
}

fn main() {
    println!("Part 1 = {}", moves_to_exit(325489));
    println!("Part 2 = {}", find_first_gt_max_val(325489));
}
