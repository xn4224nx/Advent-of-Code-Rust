/*
 * --- Day 9: Mirage Maintenance ---
 *
 * The OASIS produces a report of many values and how they are changing
 * over time (your puzzle input). Each line in the report contains the
 * history of a single value.
 *
 * To best protect the oasis, your environmental report should include a
 * prediction of the next value in each history. To do this, start by
 * making a new sequence from the difference at each step of your
 * history. If that sequence is not all zeroes, repeat this process,
 * using the sequence you just generated as the input sequence. Once all
 * of the values in your latest sequence are zeroes, you can extrapolate
 * what the next value of the original history should be.
 *
 * Part 1 - Analyze your OASIS report and extrapolate the next value for
 *          each history. What is the sum of these extrapolated values?
 */

/// Determine the common difference and the level its at
pub fn common_diff_level(seq: Vec<i32>) -> (i32, i32) {
    let mut level = 1;

    /* Calculate the common difference between each element. */
    let mut diff: Vec<i32> = seq.windows(2).map(|s| s[1] - s[0]).collect();

    loop {
        /* If all values are the same there is a common difference */
        if diff.iter().all(|&x| x == diff[0]) {
            break;
        }

        /* Calculate the common difference between common differences.*/
        diff = diff.windows(2).map(|s| s[1] - s[0]).collect();

        level += 1;
    }

    return (diff[0], level);
}

fn main() {
    println!("{:?}", common_diff_level(vec![10, 13, 16, 21, 30, 45]));
}
