#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_17;
use day_17::Maze;

#[test]
fn find_viable_dirs() {
    let mut test = Maze::new("hijkl");

    assert_eq!(test.viable_dirs(&Vec::new()), vec!['D']);
    assert_eq!(test.viable_dirs(&vec!['D']), vec!['U', 'R']);
    assert_eq!(test.viable_dirs(&vec!['D', 'R']), Vec::new());
    assert_eq!(test.viable_dirs(&vec!['D', 'U']), vec!['R']);
    assert_eq!(test.viable_dirs(&vec!['D', 'U', 'R']), Vec::new());
}

#[test]
fn converting_direction_to_final_coord() {
    let mut test = Maze::new("hijkl");

    assert_eq!(test.directs_2_coords(&Vec::new()), test.strt_pnt);
    assert_eq!(test.directs_2_coords(&vec!['D', 'U']), test.strt_pnt);
    assert_eq!(
        test.directs_2_coords(&vec!['D', 'D', 'D']),
        (test.strt_pnt.0, test.strt_pnt.1 + 3)
    );
    assert_eq!(
        test.directs_2_coords(&vec!['R', 'R', 'R']),
        (test.strt_pnt.0 + 3, test.strt_pnt.1)
    );
    assert_eq!(
        test.directs_2_coords(&vec!['D', 'U', 'D']),
        (test.strt_pnt.0, test.strt_pnt.1 + 1)
    );
    assert_eq!(
        test.directs_2_coords(&vec!['D', 'D', 'D', 'R', 'R', 'R']),
        test.end_pnt
    );
}

#[test]
fn find_the_shortest_path() {
    assert_eq!(Maze::new("ihgpwlah").find_shortest_path(), "DDRRRD");
    assert_eq!(Maze::new("kglvqrro").find_shortest_path(), "DDUDRLRRUDRD");
    assert_eq!(
        Maze::new("ulqzkmiv").find_shortest_path(),
        "DRURDRUDDLLDLUURRDULRLDUUDDDRR"
    );
}
