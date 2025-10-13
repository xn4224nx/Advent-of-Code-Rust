/*
 * --- Day 4: Secure Container ---
 *
 * You arrive at the Venus fuel depot only to discover it's protected by a
 * password. The Elves had written the password on a sticky note, but someone
 * threw it out.
 *
 * However, they do remember a few key facts about the password:
 *
 *      -   It is a six-digit number.
 *
 *      -   The value is within the range given in your puzzle input.
 *
 *      -   Two adjacent digits are the same (like 22 in 122345).
 *
 *      -   Going from left to right, the digits never decrease; they only ever
 *          increase or stay the same (like 111123 or 135679).
 *
 * Other than the range rule, the following are true:
 *
 *      -   111111 meets these criteria (double 11, never decreases).
 *
 *      -   223450 does not meet these criteria (decreasing pair of digits 50).
 *
 *      -   123789 does not meet these criteria (no double).
 *
 * PART 1:  How many different passwords within the range given in your puzzle
 *          input meet these criteria?
 */

/// Does the provided number meet the password rules
pub fn is_valid_num(number: &Vec<usize>) -> bool {
    let mut double_seen: bool = false;

    /* It is a six-digit number. */
    if number.len() != 6 {
        return false;
    }

    for dig_idx in 1..number.len() {
        /* Look for any doubles. */
        if !double_seen && number[dig_idx] == number[dig_idx - 1] {
            double_seen = true;
        }

        /* Ensure the values are not decreasing. */
        if number[dig_idx] < number[dig_idx - 1] {
            return false;
        }
    }
    return double_seen;
}

/// How many valid passwords are in the supplied range (inclusive)?
pub fn count_valid_nums(lower_lim: usize, upper_lim: usize) -> usize {
    return (lower_lim..=upper_lim)
        .filter(|x| {
            is_valid_num(
                &x.to_string()
                    .chars()
                    .map(|y| y.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>(),
            )
        })
        .count();
}

fn main() {
    println!("Part 1 = {}", count_valid_nums(254032, 789860));
}
