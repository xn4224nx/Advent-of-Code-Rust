/*
 * --- Day 9: Explosives in Cyberspace ---
 *
 * Wandering around a secure area, you come across a datalink port to a new part
 * of the network. After briefly scanning it for interesting files, you find one
 * file in particular that catches your attention. It's compressed with an
 * experimental format, but fortunately, the documentation for the format is
 * nearby.
 *
 * The format compresses a sequence of characters. Whitespace is ignored. To
 * indicate that some sequence should be repeated, a marker is added to the
 * file, like (10x2). To decompress this marker, take the subsequent 10
 * characters and repeat them 2 times. Then, continue reading the file after the
 * repeated data. The marker itself is not included in the decompressed output.
 *
 * If parentheses or other characters appear within the data referenced by a
 * marker, that's okay - treat it like normal data, not a marker, and then
 * resume looking for markers after the decompressed section.
 *
 * PART 1:  What is the decompressed length of the file (your puzzle input)?
 *          Don't count whitespace.
 */

use std::fs;

pub fn read_compressed_data(file_path: &str) -> Vec<u8> {
    let mut whole_file = fs::read(file_path).unwrap();

    /* Remove the newline character from the vector. */
    whole_file.pop();

    return whole_file;
}

/// Determine the positions and values for the markers in a compressed data
/// file. Each marker is represented by four integers, the marker starter index,
/// the marker end index, the char range and the char repitition.
pub fn find_markers(data: &Vec<u8>) -> Vec<(u32, u32, u32, u32)> {
    let mut markers = Vec::new();
    let mut in_marker = false;

    /* Track the internal marker numbers. */
    let mut part_num = Vec::new();
    let mut range_num = 0;
    let mut rep_num = 0;

    /* Record the start and end index of the marker. */
    let mut srt_idx = 0;

    /* Iterate over every character. */
    for (idx, d_char) in data.iter().enumerate() {
        /* Detect the start of a marker, ie the '(' */
        if *d_char == 40 {
            if in_marker {
                panic!("Nested markers are not supported!")
            };

            srt_idx = idx as u32;
            in_marker = true;
        }
        /* Detect the end of the marker, ie the ')' */
        else if *d_char == 41 {
            if !in_marker {
                panic!("Unmatched markers are not supported!")
            };

            /* Parse the second number of the marker. */
            rep_num = String::from_utf8(part_num.clone())
                .unwrap()
                .parse::<u32>()
                .unwrap();
            part_num.clear();

            /* Save the completed marker. */
            markers.push((srt_idx, idx as u32, range_num, rep_num));

            /* Reset the statistics ready for the next marker. */
            range_num = 0;
            rep_num = 0;
            srt_idx = 0;

            in_marker = false;
        }
        /* Detect the change in numbers of the marker, ie 'x' */
        else if *d_char == 120 && in_marker {
            range_num = String::from_utf8(part_num.clone())
                .unwrap()
                .parse::<u32>()
                .unwrap();
            part_num.clear();
        }
        /* Collect the numbers in the marker. */
        else if in_marker && *d_char >= 48 && *d_char <= 57 {
            part_num.push(*d_char)
        }
    }
    return markers;
}

pub fn decompressed_len(data: &Vec<u8>) -> usize {
    0
}

fn main() {}
