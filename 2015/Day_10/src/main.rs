/*
 * --- Day 10: Elves Look, Elves Say ---
 *
 * Today, the Elves are playing a game called look-and-say. They take turns
 * making sequences by reading aloud the previous sequence and using that
 * reading as the next sequence. For example, 211 is read as "one two, two
 * ones", which becomes 1221 (1 2, 2 1s).
 *
 * Look-and-say sequences are generated iteratively, using the previous value as
 * input for the next step. For each step, take the previous value, and replace
 * each run of digits (like 111) with the number of digits (3) followed by the
 * digit itself (1).
 *
 * PART 1:  Starting with the digits in your puzzle input, apply this process 40
 *          times. What is the length of the result?
 */

/// Create the new version of the input based on the look & say game
pub fn look_and_say(input: String) -> String {
    let mut output = String::new();
    let b_input = input.into_bytes();

    let mut grp_srt = 0;
    let mut idx = 1;

    while idx < b_input.len() {
        /* Detect a change in chars */
        if b_input[idx] != b_input[idx - 1] {
            output.push_str(format!("{}{}", idx - grp_srt, b_input[idx - 1] as char).as_str());
            grp_srt = idx;
        }
        idx += 1;
    }

    /* Catch the final group and single char strings */
    output.push_str(format!("{}{}", idx - grp_srt, b_input[idx - 1] as char).as_str());

    return output;
}

fn main() {
    let mut in_seq = String::from("1113122113");

    for _ in 0..40 {
        in_seq = look_and_say(in_seq);
    }

    println!("Part 1 = {}", in_seq.len());
}
