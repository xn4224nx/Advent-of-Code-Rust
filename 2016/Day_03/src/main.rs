/*
 * --- Day 3: Squares With Three Sides ---
 *
 * Now that you can think clearly, you move deeper into the labyrinth of
 * hallways and office furniture that makes up this part of Easter Bunny HQ.
 * This must be a graphic design department; the walls are covered in
 * specifications for triangles.
 *
 * Or are they?
 *
 * The design document gives the side lengths of each triangle it describes,
 * but... 5 10 25? Some of these aren't triangles. You can't help but mark the
 * impossible ones.
 *
 * In a valid triangle, the sum of any two sides must be larger than the
 * remaining side. For example, the "triangle" given above is impossible,
 * because 5 + 10 is not larger than 25.
 *
 * PART 1:  In your puzzle input, how many of the listed triangles are possible?
 */

/// Read the triangles sizes from file
pub fn read_triangles(file_path: &str) -> Vec<(u32, u32, u32)> {
    Vec::new()
}

/// Determine if a triangle is valid.
pub fn is_valid_triangle(triangle: (u32, u32, u32)) -> bool {
    true
}

/// Count the number of valid triangles in a list
pub fn count_valid_triangles(all_tri: &Vec<(u32, u32, u32)>) -> u32 {
    0
}

fn main() {}
