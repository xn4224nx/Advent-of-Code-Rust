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
*/

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

    return card_cnt.into_values().collect();
}

/// Determine the Type of Hand
fn classif_hand_type(hand: &String) -> u32 {
    /* Get a vector count of each card in the hand. */
    let card_cnt = card_count(hand);

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
}

/// Determine the value of a card
fn card_value(card: char) -> u32 {
    return match card {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!(),
    };
}

/// Give a hand a numerical score related to its rank
fn calc_hand_value(hand: &String) -> u32 {
    let mut score = 0;
    let mut idx = 0;

    /* Determine a hand score to differentiate between hands of the same type.
    add to score in reverse order based on the value of the card. */
    for card in hand.chars().rev() {
        score += card_value(card) * u32::pow(12, idx);
        idx += 1;
    }

    /* The most determinant thing is a hands classifcation. */
    return score + classif_hand_type(hand) * u32::pow(12, idx + 1);
}

fn main() {
    let card_data = parse_card_data("./data/example.txt");
}
