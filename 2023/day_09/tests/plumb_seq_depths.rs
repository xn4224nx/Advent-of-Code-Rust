#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_09;

#[test]
fn plumb_seq_depths_00() {
    assert_eq!(
        day_09::plumb_seq_depths(&vec![0, 3, 6, 9, 12, 15]),
        vec![vec![0, 3, 6, 9, 12, 15], vec![3, 3, 3, 3, 3]]
    )
}

#[test]
fn plumb_seq_depths_01() {
    assert_eq!(
        day_09::plumb_seq_depths(&vec![1, 3, 6, 10, 15, 21]),
        vec![
            vec![1, 3, 6, 10, 15, 21],
            vec![2, 3, 4, 5, 6],
            vec![1, 1, 1, 1]
        ]
    )
}

#[test]
fn plumb_seq_depths_02() {
    assert_eq!(
        day_09::plumb_seq_depths(&vec![10, 13, 16, 21, 30, 45]),
        vec![
            vec![10, 13, 16, 21, 30, 45],
            vec![3, 3, 5, 9, 15],
            vec![0, 2, 4, 6],
            vec![2, 2, 2]
        ]
    )
}
