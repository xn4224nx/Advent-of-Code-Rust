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
 */

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

fn main() {
    println!("Part 1 = {}", moves_to_exit(325489));
}
