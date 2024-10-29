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
 *
 * You would also like to know which IPs support SSL (super-secret listening).
 *
 * An IP supports SSL if it has an Area-Broadcast Accessor, or ABA, anywhere in
 * the supernet sequences (outside any square bracketed sections), and a
 * corresponding Byte Allocation Block, or BAB, anywhere in the hypernet
 * sequences. An ABA is any three-character sequence which consists of the same
 * character twice with a different character between them, such as xyx or aba.
 * A corresponding BAB is the same characters but in reversed positions: yxy and
 * bab, respectively.
 *
 * PART 2:  How many IPs in your puzzle input support SSL?
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

/// Test if an abba is outside the bracket and not inside them
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

/// Test if aba outside the brackets and a matching bab inside the brackets.
pub fn ip_support_ssl(addr: &Vec<AddrComp>) -> bool {
    let mut extr_grps = Vec::new();
    let mut intr_grps = Vec::new();

    /* Iterate over the components and assign the found groups. */
    for comp in addr.iter() {
        let mut tmp_grps: Vec<(u8, u8, u8)>;
        let mut exter: bool;

        /* Extract the groups and label the source. */
        match comp {
            AddrComp::Exter(val) => {
                tmp_grps = find_aba_groups(val, false);
                exter = true;
            }
            AddrComp::Inter(val) => {
                tmp_grps = find_aba_groups(val, true);
                exter = false;
            }
        };

        /* If no groups have been found skip the checks. */
        if tmp_grps.len() <= 0 {
            continue;
        };

        /* Detect if a aba internal group matches an external bab group. */
        for grp in tmp_grps.iter() {
            if (exter && intr_grps.contains(grp)) || (!exter && extr_grps.contains(grp)) {
                return true;
            }
        }

        /* Save the results in the right group */
        if exter {
            extr_grps = [extr_grps, tmp_grps].concat();
        } else {
            intr_grps = [intr_grps, tmp_grps].concat();
        }
    }
    /* If there have been no matches it doesn't support SSL. */
    return false;
}

/// Find aba groups in a ip address component
pub fn find_aba_groups(comp: &Vec<u8>, reverse: bool) -> Vec<(u8, u8, u8)> {
    let mut grps = Vec::new();

    /* Iterate over the string and test each position for aba. */
    for idx in 2..comp.len() {
        if comp[idx] == comp[idx - 2] && comp[idx] != comp[idx - 1] {
            grps.push(if reverse {
                (comp[idx - 1], comp[idx], comp[idx - 1])
            } else {
                (comp[idx], comp[idx - 1], comp[idx - 2])
            })
        }
    }
    return grps;
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

/// Count the number of valid ip addresses in a list
pub fn count_valid_addrs(all_addrs: &Vec<Vec<AddrComp>>, ssl: bool) -> usize {
    return if ssl {
        all_addrs.iter().filter(|x| ip_support_ssl(x)).count()
    } else {
        all_addrs.iter().filter(|x| ip_support_tls(x)).count()
    };
}

fn main() {
    let vari_ip_addrs = read_ip_addresses("./data/input.txt");
    println!("Part 1 = {}", count_valid_addrs(&vari_ip_addrs, false));
    println!("Part 2 = {}", count_valid_addrs(&vari_ip_addrs, true));
}
