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
 */

/// Find the lowest house number to get a certain number of presents
fn find_lowest_house(num_pres: usize) -> usize {
    let num_houses = num_pres / 10;

    /* Create the array of houses all with zero presents. */
    let mut house_pres = vec![0; num_houses];

    /* Simulate the elves delivering presents. */
    for elf in 1..num_houses {
        for idx in (0..num_houses).step_by(elf) {
            house_pres[idx] += elf * 10;
        }
    }

    /* Find the lowest house that has the specified number of presents. */
    for house_idx in 1..num_houses {
        if house_pres[house_idx] >= num_pres {
            return house_idx;
        }
    }
    panic!("House not found!");
}

fn main() {
    println!("Part 1 = {}", find_lowest_house(34000000));
}
