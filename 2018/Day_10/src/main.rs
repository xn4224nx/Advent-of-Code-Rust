/*
 * --- Day 10: The Stars Align ---
 *
 * It's no use; your navigation system simply isn't capable of providing walking
 * directions in the arctic circle, and certainly not in 1018.
 *
 * The Elves suggest an alternative. In times like these, North Pole rescue
 * operations will arrange points of light in the sky to guide missing Elves
 * back to base. Unfortunately, the message is easy to miss: the points move
 * slowly enough that it takes hours to align them, but have so much momentum
 * that they only stay aligned for a second. If you blink at the wrong time, it
 * might be hours before another message appears.
 *
 * You can see these points of light floating in the distance, and record their
 * position in the sky and their velocity, the relative change in position per
 * second (your puzzle input). The coordinates are all given from your
 * perspective; given enough time, those positions and velocities will move the
 * points into a cohesive message!
 *
 * Rather than wait, you decide to fast-forward the process and calculate what
 * the points will eventually spell.
 *
 * For example, suppose you note the following points:
 *
 *      position=< 9,  1> velocity=< 0,  2>
 *      position=< 7,  0> velocity=<-1,  0>
 *      position=< 3, -2> velocity=<-1,  1>
 *      position=< 6, 10> velocity=<-2, -1>
 *      position=< 2, -4> velocity=< 2,  2>
 *      position=<-6, 10> velocity=< 2, -2>
 *      position=< 1,  8> velocity=< 1, -1>
 *      position=< 1,  7> velocity=< 1,  0>
 *      position=<-3, 11> velocity=< 1, -2>
 *      position=< 7,  6> velocity=<-1, -1>
 *      position=<-2,  3> velocity=< 1,  0>
 *      position=<-4,  3> velocity=< 2,  0>
 *      position=<10, -3> velocity=<-1,  1>
 *      position=< 5, 11> velocity=< 1, -2>
 *      position=< 4,  7> velocity=< 0, -1>
 *      position=< 8, -2> velocity=< 0,  1>
 *      position=<15,  0> velocity=<-2,  0>
 *      position=< 1,  6> velocity=< 1,  0>
 *      position=< 8,  9> velocity=< 0, -1>
 *      position=< 3,  3> velocity=<-1,  1>
 *      position=< 0,  5> velocity=< 0, -1>
 *      position=<-2,  2> velocity=< 2,  0>
 *      position=< 5, -2> velocity=< 1,  2>
 *      position=< 1,  4> velocity=< 2,  1>
 *      position=<-2,  7> velocity=< 2, -2>
 *      position=< 3,  6> velocity=<-1, -1>
 *      position=< 5,  0> velocity=< 1,  0>
 *      position=<-6,  0> velocity=< 2,  0>
 *      position=< 5,  9> velocity=< 1, -2>
 *      position=<14,  7> velocity=<-2,  0>
 *      position=<-3,  6> velocity=< 2, -1>
 *
 * Each line represents one point. Positions are given as <X, Y> pairs: X
 * represents how far left (negative) or right (positive) the point appears,
 * while Y represents how far up (negative) or down (positive) the point
 * appears.
 *
 * At 0 seconds, each point has the position given. Each second, each point's
 * velocity is added to its position. So, a point with velocity <1, -2> is
 * moving to the right, but is moving upward twice as quickly. If this point's
 * initial position were <3, 9>, after 3 seconds, its position would become
 * <6, 3>.
 *
 * After 3 seconds, the message appeared briefly: HI. Of course, your message
 * will be much longer and will take many more seconds to appear.
 *
 * PART 1:  What message will eventually appear in the sky?
 *
 * Good thing you didn't have to wait, because that would have taken a long
 * time - much longer than the 3 seconds in the example above.
 *
 * PART 2:  Impressed by your sub-hour communication capabilities, the Elves are
 *          curious: exactly how many seconds would they have needed to wait for
 *          that message to appear?
 */

pub struct NightSky {
    pub star_position: Vec<(usize, usize)>,
    pub star_velocity: Vec<(usize, usize)>,
}

impl NightSky {
    pub fn new(data_file: &str) -> Self {
        NightSky {
            star_position: Vec::new(),
            star_velocity: Vec::new(),
        }
    }

    /// Calculate a measure of how close all the stars are to each other at a
    /// specific point int time
    pub fn star_cluster_size(&self, time: usize) -> usize {
        0
    }

    /// Create a visual representation of the night sky at a point in time.
    pub fn show(&self, time: usize) -> &str {
        ""
    }

    /// Show what the night sky looks like when its stars are closest together.
    /// Then return the time it takes for that pattern to appear.
    pub fn message(&self) -> usize {
        0
    }
}

fn main() {}
