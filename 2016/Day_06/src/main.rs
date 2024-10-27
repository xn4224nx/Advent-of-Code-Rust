/*
 * --- Day 6: Signals and Noise ---
 *
 * Something is jamming your communications with Santa. Fortunately, your signal
 * is only partially jammed, and protocol in situations like this is to switch
 * to a simple repetition code to get the message through.
 *
 * In this model, the same message is sent repeatedly. You've recorded the
 * repeating message signal (your puzzle input), but the data seems quite
 * corrupted - almost too badly to recover. Almost.
 *
 * All you need to do is figure out which character is most frequent for each
 * position.
 *
 * PART 1:  Given the recording in your puzzle input, what is the error-
 *          corrected version of the message being sent?
 *
 * Of course, that would be the message - if you hadn't agreed to use a modified
 * repetition code instead.
 *
 * In this modified code, the sender instead transmits what looks like random
 * data, but for each character, the character they actually want to send is
 * slightly less likely than the others. Even after signal-jamming noise, you
 * can look at the letter distributions in each column and choose the least
 * common letter to reconstruct the original message.
 *
 * PART 2:  Given the recording in your puzzle input and this new decoding
 *          methodology, what is the original message that Santa is trying to
 *          send?
 */

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Read the signal data file and parse into a 2D vector of numbers.
pub fn read_signal_data(data_file: &str) -> Vec<Vec<u8>> {
    let mut sigs = Vec::new();
    let mut buffer = Vec::new();

    /* Open the file. */
    let file = File::open(data_file).unwrap();
    let mut fp = BufReader::new(file);

    /* Read the file line by line. */
    while fp.read_until(b'\n', &mut buffer).unwrap() > 0 {
        sigs.push(buffer[..buffer.len() - 1].to_vec());
        buffer.clear();
    }
    return sigs;
}

/// Using frequency analysis find the hidden message
pub fn find_freq_msg(data: &Vec<Vec<u8>>, minimal: bool) -> String {
    let mut raw_msg = Vec::new();

    /* For each column of the 2D vector determine the frequency dist */
    for col_idx in 0..data[0].len() {
        let mut char_cnts = HashMap::new();

        /* Iterate over each character and record the frequencies */
        for line in data.iter() {
            char_cnts
                .entry(line[col_idx])
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        /* Find the correct char based on frequency. */
        let freq_char = if minimal {
            char_cnts
                .iter()
                .min_by(|a, b| a.1.cmp(&b.1))
                .map(|(key, _val)| key)
                .unwrap()
        } else {
            char_cnts
                .iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .map(|(key, _val)| key)
                .unwrap()
        };

        raw_msg.push(*freq_char);
    }

    /* Convert the number vector to a string. */
    return String::from_utf8(raw_msg).unwrap();
}

fn main() {
    let sig = read_signal_data("./data/input.txt");
    println!("Part 1 = {}", find_freq_msg(&sig, false));
    println!("Part 2 = {}", find_freq_msg(&sig, true));
}
