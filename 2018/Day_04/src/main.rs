/*
 * --- Day 4: Repose Record ---
 *
 * You've sneaked into another supply closet - this time, it's across from the
 * prototype suit manufacturing lab. You need to sneak inside and fix the issues
 * with the suit, but there's a guard stationed outside the lab, so this is as
 * close as you can safely get.
 *
 * As you search the closet for anything that might help, you discover that
 * you're not the first person to want to sneak in. Covering the walls, someone
 * has spent an hour starting every midnight for the past few months secretly
 * observing this guard post! They've been writing down the ID of the one guard
 * on duty that night - the Elves seem to have decided that one guard was enough
 * for the overnight shift - as well as when they fall asleep or wake up while
 * at their post (your puzzle input).
 *
 * For example, consider the following records, which have already been
 * organized into chronological order:
 *
 *      [1518-11-01 00:00] Guard #10 begins shift
 *      [1518-11-01 00:05] falls asleep
 *      [1518-11-01 00:25] wakes up
 *      [1518-11-01 00:30] falls asleep
 *      [1518-11-01 00:55] wakes up
 *      [1518-11-01 23:58] Guard #99 begins shift
 *      [1518-11-02 00:40] falls asleep
 *      [1518-11-02 00:50] wakes up
 *      [1518-11-03 00:05] Guard #10 begins shift
 *      [1518-11-03 00:24] falls asleep
 *      [1518-11-03 00:29] wakes up
 *      [1518-11-04 00:02] Guard #99 begins shift
 *      [1518-11-04 00:36] falls asleep
 *      [1518-11-04 00:46] wakes up
 *      [1518-11-05 00:03] Guard #99 begins shift
 *      [1518-11-05 00:45] falls asleep
 *      [1518-11-05 00:55] wakes up
 *
 * Timestamps are written using year-month-day hour:minute format. The guard
 * falling asleep or waking up is always the one whose shift most recently
 * started. Because all asleep/awake times are during the midnight hour (00:00 -
 * 00:59), only the minute portion (00 - 59) is relevant for those events.
 *
 * Visually, these records show that the guards are asleep at these times:
 *
 * Date   ID   Minute
 *            000000000011111111112222222222333333333344444444445555555555
 *            012345678901234567890123456789012345678901234567890123456789
 * 11-01  #10  .....####################.....#########################.....
 * 11-02  #99  ........................................##########..........
 * 11-03  #10  ........................#####...............................
 * 11-04  #99  ....................................##########..............
 * 11-05  #99  .............................................##########.....
 *
 * The columns are Date, which shows the month-day portion of the relevant day;
 * ID, which shows the guard on duty that day; and Minute, which shows the
 * minutes during which the guard was asleep within the midnight hour. (The
 * Minute column's header shows the minute's ten's digit in the first row and
 * the one's digit in the second row.) Awake is shown as ., and asleep is shown
 * as #.
 *
 * Note that guards count as asleep on the minute they fall asleep, and they
 * count as awake on the minute they wake up. For example, because Guard #10
 * wakes up at 00:25 on 1518-11-01, minute 25 is marked as awake.
 *
 * If you can figure out the guard most likely to be asleep at a specific time,
 * you might be able to trick that guard into working tonight so you can have
 * the best chance of sneaking in. You have two strategies for choosing the best
 * guard/minute combination.
 *
 * Strategy 1: Find the guard that has the most minutes asleep. What minute does
 * that guard spend asleep the most?
 *
 * In the example above, Guard #10 spent the most minutes asleep, a total of 50
 * minutes (20+25+5), while Guard #99 only slept for a total of 30 minutes
 * (10+10+10). Guard #10 was asleep most during minute 24 (on two days, whereas
 * any other minute the guard was asleep was only seen on one day).
 *
 * While this example listed the entries in chronological order, your entries
 * are in the order you found them. You'll need to organize them before they can
 * be analyzed.
 *
 * PART 1:  What is the ID of the guard you chose multiplied by the minute you
 *          chose? (In the above example, the answer would be 10 * 24 = 240.)
 *
 * Strategy 2: Of all guards, which guard is most frequently asleep on the same
 * minute?
 *
 * In the example above, Guard #99 spent minute 45 asleep more than any other
 * guard or minute - three times in total. (In all other cases, any guard spent
 * any minute asleep at most twice.)
 *
 * PART 2:  What is the ID of the guard you chose multiplied by the minute you
 *          chose? (In the above example, the answer would be 99 * 45 = 4455.)
 */

use chrono::NaiveDateTime;
use chrono::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct SecuritySchedule {
    pub midnight_sleeps: HashMap<usize, Vec<usize>>,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum Event {
    GuardWakes,
    GuardSleeps,
    GuardStarts(usize),
}

impl SecuritySchedule {
    pub fn new(sleep_record: &str) -> Self {
        let mut events = Vec::new();
        let mut buffer = String::new();
        let mut midnight_sleeps = HashMap::new();
        let re = Regex::new(r"\[(\d+\-\d+\-\d+ \d+:\d+)\] (.*)").unwrap();
        let num_re = Regex::new(r"(\d+)").unwrap();

        /* Open the file. */
        let file = File::open(sleep_record).unwrap();
        let mut fp = BufReader::new(file);

        /* Read the file line by line and read the events. */
        while fp.read_line(&mut buffer).unwrap() > 0 {
            let Some(caps) = re.captures(&buffer) else {
                println!("Line not parse: '{}'", buffer);
                buffer.clear();
                continue;
            };

            /* Parse the date and time of the event. */
            let event_dt = NaiveDateTime::parse_from_str(&caps[1], "%Y-%m-%d %H:%M").unwrap();

            /* Parse the nature of the event. */
            let event_nature = if caps[2].contains("wakes") {
                Event::GuardWakes
            } else if caps[2].contains("falls") {
                Event::GuardSleeps
            } else {
                let guard_id = num_re.captures(&caps[2]).unwrap()[1]
                    .parse::<usize>()
                    .unwrap();
                midnight_sleeps.insert(guard_id, vec![0; 60]);
                Event::GuardStarts(guard_id)
            };

            /* Save the event */
            events.push((event_dt, event_nature));
            buffer.clear();
        }

        /* Sort the events by the datatime it happened. */
        events.sort_by_key(|x| x.0);

        /* Iterate over the events and record the muinutes a guard was asleep. */
        let mut curr_guard = 0;
        let mut active_guard = false;
        let mut guard_asleep = false;

        for eve_idx in 0..events.len() {
            match events[eve_idx].1 {
                Event::GuardWakes => {
                    guard_asleep = false;

                    /* Determine when the guard fell asleep. */
                    let start_min = if events[eve_idx - 1].0.time().hour() != 0 {
                        0
                    } else {
                        events[eve_idx - 1].0.time().minute()
                    };

                    /* Record the minutes they where asleep */
                    if let Some(x) = midnight_sleeps.get_mut(&curr_guard) {
                        for t_idx in start_min..events[eve_idx].0.time().minute() {
                            x[t_idx as usize] += 1;
                        }
                    };
                }
                Event::GuardSleeps => {
                    guard_asleep = true;
                }
                Event::GuardStarts(g_id) => {
                    /* Has a guard ended a shift asleep? */
                    if active_guard && guard_asleep {
                        let start_min = if events[eve_idx - 1].0.time().hour() != 0 {
                            0
                        } else {
                            events[eve_idx - 1].0.time().minute()
                        };

                        /* Record the minutes they where asleep */
                        if let Some(x) = midnight_sleeps.get_mut(&curr_guard) {
                            for t_idx in start_min..events[eve_idx].0.time().minute() {
                                x[t_idx as usize] += 1;
                            }
                        };
                    }

                    /* Set the guard details. */
                    active_guard = true;
                    guard_asleep = false;
                    curr_guard = g_id;
                }
            }
        }

        /* catch the final guard being asleep on shift. */
        if guard_asleep {
            let eve_idx = events.len() - 1;
            let start_min = if events[eve_idx - 1].0.time().hour() != 0 {
                0
            } else {
                events[eve_idx - 1].0.time().minute()
            };

            /* Record the minutes they where asleep */
            if let Some(x) = midnight_sleeps.get_mut(&curr_guard) {
                for t_idx in start_min..events[eve_idx].0.time().minute() {
                    x[t_idx as usize] += 1;
                }
            };
        }

        return SecuritySchedule { midnight_sleeps };
    }

    /// What guard sleeps the most? Return the id multiplied by the minute that
    /// they are most likely to be alseep.
    pub fn sleepiest_guard_id(&self) -> usize {
        let mut sleepiest_guard = 0;
        let mut sleep_amount = 0;

        /* Find the guard that sleeps the most. */
        for (g_idx, g_sleep) in self.midnight_sleeps.iter() {
            let tmp_slp_amnt = g_sleep.iter().sum();

            if sleep_amount < tmp_slp_amnt {
                sleep_amount = tmp_slp_amnt;
                sleepiest_guard = *g_idx;
            }
        }

        /* Find what minute they are most likely to be asleep. */
        let mut most_occurances = 0;
        let mut sleepiest_min = 0;

        if let Some(mid_hour) = self.midnight_sleeps.get(&sleepiest_guard) {
            for t_idx in 0..mid_hour.len() {
                if mid_hour[t_idx] > most_occurances {
                    most_occurances = mid_hour[t_idx];
                    sleepiest_min = t_idx;
                }
            }
        };
        return sleepiest_min * sleepiest_guard;
    }

    /// Which guard is most frequently asleep on the same minute?
    pub fn guard_most_reliable_asleep(&self) -> usize {
        let mut sleepiest_guard = 0;
        let mut sleep_amount = 0;
        let mut min_of_midnight = 0;

        /* Find the guard that is asleep consistently. */
        for (g_idx, g_sleep) in self.midnight_sleeps.iter() {
            let mut curr_slp_min = 0;
            let mut curr_slp_amt = 0;

            /* Find the minute that this guard is asleep the most. */
            for t_idx in 0..g_sleep.len() {
                if curr_slp_amt < g_sleep[t_idx] {
                    curr_slp_min = t_idx;
                    curr_slp_amt = g_sleep[t_idx];
                }
            }

            if curr_slp_amt > sleep_amount {
                sleep_amount = curr_slp_amt;
                sleepiest_guard = *g_idx;
                min_of_midnight = curr_slp_min;
            }
        }
        return min_of_midnight * sleepiest_guard;
    }
}

fn main() {
    let lab_guards = SecuritySchedule::new("./data/input_0.txt");
    println!("Part 1 = {}", lab_guards.sleepiest_guard_id());
    println!("Part 2 = {}", lab_guards.guard_most_reliable_asleep());
}
