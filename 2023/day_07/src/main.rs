/*
* --- Day 7: Camel Cards ---
*
* In Camel Cards, you get a list of hands, and your goal is to order them based
* on the strength of each hand. A hand consists of five cards labeled one of A,
* K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of each card
* follows this order, where A is the highest and 2 is the lowest.
*
* Every hand is exactly one type. From strongest to weakest, they are:
*
*   - Five of a kind, where all five cards have the same label: AAAAA
*
*   - Four of a kind, where four cards have the same label and one card has a
*       different label: AA8AA
*
*   - Full house, where three cards have the same label, and the remaining two
*       cards share a different label: 23332
*
*   - Three of a kind, where three cards have the same label, and the remaining
*       two cards are each different from any other card in the hand: TTT98
*
*   - Two pair, where two cards share one label, two other cards share a second
*       label, and the remaining card has a third label: 23432
*
*   - One pair, where two cards share one label, and the other three cards have
*       a different label from the pair and each other: A23A4
*
*   - High card, where all cards' labels are distinct: 23456
*
* Hands are primarily ordered based on type; for example, every full house is
* stronger than any three of a kind.
*
* If two hands have the same type, a second ordering rule takes effect. Start
* by comparing the first card in each hand. If these cards are different, the
* hand with the stronger first card is considered stronger. If the first card in
* each hand have the same label, however, then move on to considering the second
* card in each hand. If they differ, the hand with the higher second card wins;
* otherwise, continue with the third card in each hand, then the fourth, then
* the fifth.
*
* To play Camel Cards, you are given a list of hands and their corresponding
* bid (your puzzle input).
*
* Each hand is followed by its bid amount. Each hand wins an amount equal to its
* bid multiplied by its rank, where the weakest hand gets rank 1, the
* second-weakest hand gets rank 2, and so on up to the strongest hand. Because
* there are five hands in this example, the strongest hand will have rank 5 and
* its bid will be multiplied by 5.
*
* So, the first step is to put the hands in order of strength. Now, you can
* determine the total winnings of this set of hands by adding up the result of
* multiplying each hand's bid with its rank.
*
* Part 1 - Find the rank of every hand in your set. What are the total winnings?
*
* Now, J cards are jokers - wildcards that can act like whatever card would make
* the hand the strongest type possible. To balance this, J cards are now the
* weakest individual cards, weaker even than 2.
*
* Part 2 - Using the new joker rule, find the rank of every hand in your set.
*          What are the new total winnings?
*/

static CARD_TYPES: u32 = 13;

use std::cmp::min;
use std::collections::HashMap;
use std::fs;

/// Read a file with hands and bids
fn parse_card_data(file_path: &str) -> Vec<(String, u32)> {
    let mut card_data = Vec::new();

    /* Read the entire file into a string. */
    let raw_file = fs::read_to_string(file_path).expect("Could not open file!");

    /* Iterate over the read file and parse the data */
    for raw_line in raw_file.lines() {
        let mut rl_split = raw_line.split_whitespace();
        let cards = rl_split.next().unwrap().to_string();
        let bid = rl_split.next().unwrap().parse::<u32>().unwrap();
        card_data.push((cards, bid));
    }
    return card_data;
}

/// Determine the unique card counts in a hand
fn card_count(hand: &String) -> Vec<u32> {
    let mut card_cnt = HashMap::new();

    /* Convert the string to a vector of chars */
    let hand_vec: Vec<char> = hand.chars().collect();

    /* Count the occurance of each card */
    for card in hand_vec {
        *card_cnt.entry(card).or_insert(0) += 1;
    }

    let mut counts: Vec<u32> = card_cnt.into_values().collect();
    counts.sort();
    counts.reverse();

    return counts;
}

/// Determine the Type of Hand
fn classif_hand_type(hand: &String, joker: bool) -> u32 {
    /* Get a vector count of each card in the hand. */
    let card_cnt = card_count(hand);

    if !joker || !hand.contains("J") {
        /* Five of a kind */
        if card_cnt.len() == 1 {
            return 6;

        /* One Pair */
        } else if card_cnt.len() == 4 {
            return 1;

        /* High Card */
        } else if card_cnt.len() == 5 {
            return 0;

        /* Four of a Kind */
        } else if card_cnt.contains(&4) {
            return 5;

        /* Full House */
        } else if card_cnt.len() == 2 {
            return 4;

        /* Three of a Kind */
        } else if card_cnt.contains(&3) {
            return 3;

        /* Two Pair */
        } else {
            return 2;
        }
    } else {
        let joker_cnt = hand.chars().filter(|c| *c == 'J').count() as u32;
        let mut non_j_cnts = card_count(&hand.chars().filter(|c| *c != 'J').collect());

        /* Catch all the cards being jokers. */
        if non_j_cnts.len() == 0 {
            non_j_cnts.push(0);
        }

        /* Five of a kind */
        if joker_cnt + non_j_cnts[0] == 5 {
            return 6;

        /* Four of a Kind */
        } else if joker_cnt + non_j_cnts[0] == 4 {
            return 5;

        /* Full House */
        } else if ((joker_cnt + non_j_cnts[0] == 3) && non_j_cnts[1] >= 2)
            || (non_j_cnts[0] == 3 && (non_j_cnts[1] + joker_cnt >= 2))
        {
            return 4;

        /* Three of a Kind */
        } else if non_j_cnts[0] + joker_cnt == 3 {
            return 3;

        /* Two Pair */
        } else if non_j_cnts[0] == 2 && non_j_cnts[1] + min(joker_cnt, 1) == 2 {
            return 2;

        /* One Pair */
        } else if non_j_cnts[0] + joker_cnt == 2 {
            return 1;

        /* High Card */
        } else {
            return 0;
        }
    }
}

/// Determine the value of a card
fn card_value(card: char, joker: bool) -> u32 {
    if card == '2' {
        if joker {
            return 1;
        } else {
            return 0;
        }
    } else if card == '3' {
        if joker {
            return 2;
        } else {
            return 1;
        }
    } else if card == '4' {
        if joker {
            return 3;
        } else {
            return 2;
        }
    } else if card == '5' {
        if joker {
            return 4;
        } else {
            return 3;
        }
    } else if card == '6' {
        if joker {
            return 5;
        } else {
            return 4;
        }
    } else if card == '7' {
        if joker {
            return 6;
        } else {
            return 5;
        }
    } else if card == '8' {
        if joker {
            return 7;
        } else {
            return 6;
        }
    } else if card == '9' {
        if joker {
            return 8;
        } else {
            return 7;
        }
    } else if card == 'T' {
        if joker {
            return 9;
        } else {
            return 8;
        }
    } else if card == 'J' {
        if joker {
            return 0;
        } else {
            return 9;
        }
    } else if card == 'Q' {
        return 10;
    } else if card == 'K' {
        return 11;
    } else if card == 'A' {
        return 12;
    } else {
        panic!("Unknown card!");
    }
}

/// Give a hand a numerical score related to its rank
fn calc_hand_value(hand: &String, joker: bool) -> u32 {
    let mut score = 0;
    let mut idx = 0;

    /* Determine a hand score to differentiate between hands of the same type.
    add to score in reverse order based on the value of the card. */
    for card in hand.chars().rev() {
        score += card_value(card, joker) * u32::pow(CARD_TYPES, idx);
        idx += 1;
    }

    /* The most determinant thing is a hands classifcation. */
    return score + classif_hand_type(hand, joker) * u32::pow(CARD_TYPES, idx + 1);
}

/// Rank the hands and sum the hand bid multiplied by the ranking
fn calc_total_winnings(card_data: &mut Vec<(String, u32)>, joker: bool) -> u32 {
    let mut winnings = 0;

    /* Sort the vector of hands based on the card value. */
    card_data.sort_by_cached_key(|x| calc_hand_value(&x.0, joker));

    /* Calculate the total winnings. */
    for (idx, (_hand, bid)) in card_data.iter().enumerate() {
        winnings += bid * (idx as u32 + 1)
    }

    return winnings;
}

fn main() {
    let mut card_data = parse_card_data("./data/input.txt");
    println!(
        "Part 1 answer = {}",
        calc_total_winnings(&mut card_data, false)
    );
    println!(
        "Part 2 answer = {}",
        calc_total_winnings(&mut card_data, true)
    );
}
