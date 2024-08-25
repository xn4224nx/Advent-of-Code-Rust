/*
 * --- Day 11: Corporate Policy ---
 *
 * Santa's previous password expired, and he needs help choosing a new one.
 *
 * To help him remember his new password after the old one expires, Santa has
 * devised a method of coming up with a password based on the previous one.
 * Corporate policy dictates that passwords must be exactly eight lowercase
 * letters (for security reasons), so he finds his new password by incrementing
 * his old password string repeatedly until it is valid.
 *
 * Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so
 * on. Increase the rightmost letter one step; if it was z, it wraps around to
 * a, and repeat with the next letter to the left until one doesn't wrap
 * around.
 *
 * Unfortunately for Santa, a new Security-Elf recently started, and he has
 * imposed some additional password requirements:
 *
 *      -   Passwords must include one increasing straight of at least three
 *          letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip
 *          letters; abd doesn't count.
 *
 *      -   Passwords may not contain the letters i, o, or l, as these letters
 *          can be mistaken for other characters and are therefore confusing.
 *
 *      -   Passwords must contain at least two different, non-overlapping pairs
 *          of letters, like aa, bb, or zz.
 *
 * PART 1:  Given Santa's current password (your puzzle input), what should his
 *          next password be?
 *
 * PART 2: Santa's password expired again. What's the next one?
 */

use std::collections::HashSet;

/// Increment a string although it is a number.
pub fn increm_str(pass: &mut Vec<u8>) {
    for idx in (0..pass.len()).rev() {
        /* If the letter is a 'z' change it to a 'a' */
        if pass[idx] == 122 {
            pass[idx] = 97;

        /* Otherwise increment the next letter on and then stop. */
        } else {
            pass[idx] += 1;
            break;
        }
    }
}

/// Determine if a password passes all the rules.
pub fn is_pass_valid(pass: &Vec<u8>) -> bool {
    let mut inc_triple = false;
    let mut pairs = HashSet::new();

    for idx in 0..pass.len() {
        /* Password may not contain i, o or l. */
        if pass[idx] == 105 || pass[idx] == 111 || pass[idx] == 108 {
            return false;
        };

        /* Passwords must include an increasing range of three letters. */
        if idx > 1 && pass[idx - 2] + 2 == pass[idx] && pass[idx - 1] + 1 == pass[idx] {
            inc_triple = true;
        };

        /* Passwords must contain at least two different, pairs of letters. */
        if idx > 1 && pass[idx] == pass[idx - 1] {
            pairs.insert(pass[idx]);
        };
    }

    return inc_triple && pairs.len() > 1;
}

/// Change the given password to the next valid password
pub fn next_valid_pass(pass: &mut Vec<u8>) {
    loop {
        increm_str(pass);

        if is_pass_valid(pass) {
            return;
        }
    }
}

fn main() {
    let mut pass_0 = "hxbxwxba".as_bytes().to_vec();
    next_valid_pass(&mut pass_0);
    println!("Part 1 = {}", std::str::from_utf8(&pass_0).unwrap());
    next_valid_pass(&mut pass_0);
    println!("Part 2 = {}", std::str::from_utf8(&pass_0).unwrap());
}
