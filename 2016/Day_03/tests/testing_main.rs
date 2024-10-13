#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_03;
use day_03::{count_valid_triangles, is_valid_triangle, read_triangles};

#[test]
fn test_read_triangles_01() {
    assert_eq!(
        read_triangles("./data/example_01.txt"),
        vec![
            (330, 143, 338),
            (769, 547, 83),
            (930, 625, 317),
            (669, 866, 147),
            (15, 881, 210),
        ]
    );
}

#[test]
fn test_read_triangles_02() {
    assert_eq!(
        read_triangles("./data/example_02.txt"),
        vec![
            (101, 301, 501),
            (102, 302, 502),
            (103, 303, 503),
            (201, 401, 601),
            (202, 402, 602),
            (203, 403, 603),
        ]
    );
}

#[test]
fn test_is_valid_triangle_01() {
    assert_eq!(is_valid_triangle((330, 143, 338)), true);
}

#[test]
fn test_is_valid_triangle_02() {
    assert_eq!(is_valid_triangle((769, 547, 83)), false);
}

#[test]
fn test_is_valid_triangle_03() {
    assert_eq!(is_valid_triangle((930, 625, 317)), true);
}

#[test]
fn test_is_valid_triangle_04() {
    assert_eq!(is_valid_triangle((669, 866, 147)), false);
}

#[test]
fn test_is_valid_triangle_05() {
    assert_eq!(is_valid_triangle((15, 881, 210)), false);
}

#[test]
fn test_is_valid_triangle_06() {
    assert_eq!(is_valid_triangle((5, 10, 25)), false);
}

#[test]
fn test_count_valid_triangles_01() {
    let tri = read_triangles("./data/example_01.txt");
    assert_eq!(count_valid_triangles(&tri), 2);
}

#[test]
fn test_count_valid_triangles_02() {
    let tri = read_triangles("./data/example_02.txt");
    assert_eq!(count_valid_triangles(&tri), 3);
}
