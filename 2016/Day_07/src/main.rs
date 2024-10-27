/*
 * --- Day 7: Internet Protocol Version 7 ---
 *
 * While snooping around the local network of EBHQ, you compile a list of IP
 * addresses (they're IPv7, of course; IPv6 is much too limited). You'd like to
 * figure out which IPs support TLS (transport-layer snooping).
 *
 * An IP supports TLS if it has an Autonomous Bridge Bypass Annotation, or ABBA.
 * An ABBA is any four-character sequence which consists of a pair of two
 * different characters followed by the reverse of that pair, such as xyyx or
 * abba. However, the IP also must not have an ABBA within any hypernet
 * sequences, which are contained by square brackets.
 *
 * PART 1:  How many IPs in your puzzle input support TLS?
 */

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub enum AddrComp {
    Inter(Vec<u8>),
    Exter(Vec<u8>),
}

/// Read the addresses and parse into a structured format.
pub fn read_ip_addresses(file_path: &str) -> Vec<Vec<AddrComp>> {
    let mut results = Vec::new();
    let mut buffer = Vec::new();

    /* Open the file */
    let file = File::open(file_path).unwrap();
    let mut fp = BufReader::new(file);

    /* Iterate over the file line by line. */
    while fp.read_until(b'\n', &mut buffer).unwrap() > 0 {
        let mut line = Vec::new();

        /* Record if the current char is in or out of square brackets. */
        let mut in_brack = false;
        let mut temp = Vec::new();

        /* Iterate over the characters in the line */
        for bchar in &buffer {
            /* A [ has been encountered. */
            if *bchar == 91 {
                in_brack = true;

                /* Save the buffer at the change of state and reset it */
                if temp.len() > 0 {
                    line.push(AddrComp::Exter(temp.clone()));
                    temp.clear();
                };

            /* A ] has been encountered. */
            } else if *bchar == 93 {
                in_brack = false;

                /* Save the buffer at the change of state and reset it */
                if temp.len() > 0 {
                    line.push(AddrComp::Inter(temp.clone()));
                    temp.clear();
                };

            /* The end of the line has been encountered. */
            } else if *bchar == 10 {
                /* Save the buffer at the end of the line. */
                if temp.len() > 0 {
                    line.push(AddrComp::Exter(temp.clone()));
                    temp.clear();
                };
                break;

            /* Otherwise save the current char to a buffer */
            } else {
                temp.push(*bchar);
            }
        }
        results.push(line);
        buffer.clear();
    }
    return results;
}

/// Test if an abba in outside the bracket and not inside them
pub fn ip_support_tls(addr: &Vec<AddrComp>) -> bool {
    let mut external_abba = false;

    /* Iterate over the address components. */
    for comp in addr.iter() {
        match comp {
            AddrComp::Inter(val) => {
                /* Any internal abba means its not supported no matter what. */
                if comp_has_abba(val) {
                    return false;
                }
            }
            AddrComp::Exter(val) => {
                if comp_has_abba(val) {
                    external_abba = true;
                }
            }
        }
    }
    return external_abba;
}

/// Determine if a vector has an abba pattern within
pub fn comp_has_abba(comp: &Vec<u8>) -> bool {
    /* Compare the four characters together in a group. */
    for idx in 3..comp.len() {
        /* Test for an abba pattern. */
        if comp[idx] == comp[idx - 3]
            && comp[idx - 1] == comp[idx - 2]
            && comp[idx - 1] != comp[idx]
        {
            return true;
        }
    }
    return false;
}

fn main() {}
