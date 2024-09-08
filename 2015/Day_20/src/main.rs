/*
 * --- Day 20: Infinite Elves and Infinite Houses ---
 *
 * To keep the Elves busy, Santa has them deliver some presents by hand, door-
 * to-door. He sends them down a street with infinite houses numbered
 * sequentially: 1, 2, 3, 4, 5, and so on.
 *
 * Each Elf is assigned a number, too, and delivers presents to houses based on
 * that number:
 *
 *  -   The first Elf (number 1) delivers presents to every house: 1, 2, 3, 4,
 *      5, ....
 *
 *  -   The second Elf (number 2) delivers presents to every second house: 2,
 *      4, 6, 8, 10, ....
 *
 *  -   Elf number 3 delivers presents to every third house: 3, 6, 9, 12, 15,
 *      ....
 *
 * There are infinitely many Elves, numbered starting with 1. Each Elf delivers
 * presents equal to ten times his or her number at each house.
 *
 * PART 1:  What is the lowest house number of the house to get at least as
 *          many presents as the number in your puzzle input, 34000000?
 *
 * The Elves decide they don't want to visit an infinite number of houses.
 * Instead, each Elf will stop after delivering presents to 50 houses. To make
 * up for it, they decide to deliver presents equal to eleven times their number
 * at each house.
 *
 * PART 2:  With these changes, what is the new lowest house number of the house
 *          to get at least as many presents as the number in your puzzle input?
 */

/// Fill houses with presents delivered by elves
pub fn deliver_presents(num_pres: usize) -> Vec<usize> {
    let num_houses = num_pres / 10;

    /* Create the array of houses all with zero presents. */
    let mut house_pres = vec![0; num_houses];

    /* Simulate the elves delivering presents. */
    for elf in 1..num_houses {
        for idx in (0..num_houses).step_by(elf) {
            house_pres[idx] += elf * 10;
        }
    }
    return house_pres;
}

/// Find the lowest house index with the set number of presents
pub fn find_lowest_house(house_presents: &Vec<usize>, num_pres: usize) -> usize {
    let num_houses = num_pres / 10;

    /* Find the lowest house that has the specified number of presents. */
    for house_idx in 1..num_houses {
        if house_presents[house_idx] >= num_pres {
            return house_idx;
        }
    }
    panic!("House not found!");
}

fn main() {
    let min_presents = 34000000;

    let houses_part1 = deliver_presents(min_presents);
    println!(
        "Part 1 = {}",
        find_lowest_house(&houses_part1, min_presents)
    );
}
