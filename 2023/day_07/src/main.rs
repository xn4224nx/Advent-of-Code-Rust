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

fn main() {
    println!("Hello, world!");
}
