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
 *
 * Apparently, the file actually uses version two of the format.
 *
 * In version two, the only difference is that markers within decompressed data
 * are decompressed. This, the documentation explains, provides much more
 * substantial compression capabilities, allowing many-gigabyte files to be
 * stored in only a few kilobytes.
 *
 * Unfortunately, the computer you brought probably doesn't have enough memory
 * to actually decompress the file; you'll have to come up with another way to
 * get its decompressed length.
 *
 * PART 2:  What is the decompressed length of the file using this improved
 *          format?
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
pub fn find_markers(data: &Vec<u8>) -> Vec<(u64, u64, u64, u64)> {
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

            srt_idx = idx as u64;
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
                .parse::<u64>()
                .unwrap();
            part_num.clear();

            /* Save the completed marker. */
            markers.push((srt_idx, idx as u64, range_num, rep_num));

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
                .parse::<u64>()
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

/// Measure the character length of the decompressed data file.
pub fn decompressed_len(data: &Vec<u8>) -> u64 {
    let mut decom_len: u64 = 0;
    let mut skipped_mkrs = Vec::new();
    let mut valid_mkrs = Vec::new();

    /* Generate the marker metadata */
    let markers = find_markers(data);

    /* If there are no markers mearly return the data length. */
    if markers.is_empty() {
        return data.len() as u64;
    }

    /* Determine the valid markers. */
    for mk_idx in 0..markers.len() {
        /* The first marker is always valid but other markers are only executed
         * if a previous ones have not skipped it. */
        if mk_idx == 0 || !skipped_mkrs.contains(&mk_idx) {
            valid_mkrs.push(mk_idx);
            let nxt_mkr_idx = mk_idx + 1;

            /* Iterate over all the other markers determine the skipped ones. */
            for n_idx in nxt_mkr_idx..markers.len() {
                /* What is the index in the data that the current goes up to. */
                let final_mkr_reach = markers[mk_idx].1 + markers[mk_idx].2;

                /* Does this marker start in the range of the current one? */
                if final_mkr_reach > markers[n_idx].0 {
                    skipped_mkrs.push(n_idx);
                } else {
                    break;
                }
            }
        }
    }

    /* Add the length before the first marker */
    decom_len += markers[0].0;

    /* Iterate over the valid markers and determine the uncompressed length. */
    for valid_mk_idx in 0..valid_mkrs.len() {
        let mk_idx = valid_mkrs[valid_mk_idx];

        /* Determine the expansion due to this marker */
        decom_len += markers[mk_idx].2 * markers[mk_idx].3;

        /* Determine the unexpanded data between this marker and the next. */
        if valid_mk_idx != valid_mkrs.len() - 1 {
            let nxt_mk_idx = valid_mkrs[valid_mk_idx + 1];
            decom_len += markers[nxt_mk_idx].0 - (markers[mk_idx].1 + markers[mk_idx].2 + 1);
        }
    }

    /* Add in uncompressed data beyond the final marker. */
    let final_mk_idx = valid_mkrs[valid_mkrs.len() - 1];
    decom_len += (data.len() - 1) as u64 - (markers[final_mk_idx].1 + markers[final_mk_idx].2);

    return decom_len;
}

/// Measure the character length from a recursive decompresion of a data file
pub fn rec_decomp_len(data: &Vec<u8>) -> u64 {
    let mut sparse_data: Vec<(usize, u64)> = Vec::new();
    let markers = find_markers(data);

    /* If there are no markers mearly return the data length. */
    if markers.is_empty() {
        return data.len() as u64;
    }

    /* Record the distance between markers or the end of the data. */
    for mk_idx in 0..markers.len() {
        let start_idx = markers[mk_idx].1 + 1;
        let end_idx = if mk_idx != markers.len() - 1 {
            markers[mk_idx + 1].0 - 1
        } else {
            data.len() as u64 - 1
        };

        /* Only save it if there is data between markers. */
        if start_idx > end_idx {
            continue;
        };

        /* Transform the data start and end indexes into a sparse array. */
        for idx in start_idx..end_idx + 1 {
            sparse_data.push((idx as usize, 1));
        }
    }

    /* Iterate over the markers and determine marker data that gets changed. */
    for mk_idx in 0..markers.len() {
        let mk_reach = markers[mk_idx].1 + markers[mk_idx].2;

        /* Determine the non-marker data it covers. */
        for data_idx in 0..sparse_data.len() {
            let inter_pos = sparse_data[data_idx].0 as u64;

            /* Check this data is in reach of the current marker. */
            if inter_pos <= mk_reach && inter_pos > markers[mk_idx].0 {
                sparse_data[data_idx].1 *= markers[mk_idx].3;
            }
        }
    }
    return markers[0].0 as u64 + sparse_data.iter().map(|x| x.1).sum::<u64>();
}

fn main() {
    let uncomp_data = read_compressed_data("./data/input.txt");
    println!("Part 1 = {}", decompressed_len(&uncomp_data));
    println!("Part 2 = {}", rec_decomp_len(&uncomp_data));
}
