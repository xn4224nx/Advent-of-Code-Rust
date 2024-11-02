#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_08;
use day_08::{Instruc, Screen};
use ndarray::Array2;

#[test]
fn new_structure() {
    let mut test_src = Screen::new("./data/example_01.txt");
    assert_eq!(
        test_src.com_file_path,
        String::from("./data/example_01.txt")
    );
    assert_eq!(test_src.size, (0, 0));
    assert_eq!(test_src.pixels, Array2::from(Vec::<[bool; 2]>::new()));
    assert_eq!(test_src.commands, Vec::new());
}

#[test]
fn parsing_commands() {
    let mut test_src = Screen::new("./data/example_01.txt");
    test_src.load_commands();
    assert_eq!(
        test_src.commands,
        vec![
            Instruc::Rect(3, 2),
            Instruc::RotCol(1, 1),
            Instruc::RotRow(0, 4),
            Instruc::RotCol(1, 1)
        ]
    );
}

#[test]
fn blank_screen() {
    let mut test_src = Screen::new("./data/example_01.txt");
    test_src.load_commands();
    assert_eq!(
        test_src.render(),
        vec![
            String::from("......."),
            String::from("......."),
            String::from(".......")
        ]
    );
}

#[test]
fn set_rec_3x2() {
    let mut test_src = Screen::new("./data/example_01.txt");
    test_src.load_commands();

    test_src.set_rect(3, 2);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("###...."),
            String::from("###...."),
            String::from(".......")
        ]
    );
}

#[test]
fn set_rec_2x2() {
    let mut test_src = Screen::new("./data/example_01.txt");
    test_src.load_commands();

    test_src.set_rect(2, 2);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("##....."),
            String::from("##....."),
            String::from(".......")
        ]
    );
}

#[test]
fn set_rec_2x3() {
    let mut test_src = Screen::new("./data/example_01.txt");
    test_src.load_commands();

    test_src.set_rect(2, 3);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("##....."),
            String::from("##....."),
            String::from("##.....")
        ]
    );
}

#[test]
fn rot_col_1x1() {
    let mut test_src = Screen::new("./data/example_01.txt");
    test_src.load_commands();

    test_src.set_rect(3, 2);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("###...."),
            String::from("###...."),
            String::from(".......")
        ]
    );

    test_src.rotate_col(1, 1);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("#.#...."),
            String::from("###...."),
            String::from(".#.....")
        ]
    );
}

#[test]
fn rot_col_0x1() {
    let mut test_src = Screen::new("./data/example_01.txt");
    test_src.load_commands();

    test_src.set_rect(3, 2);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("###...."),
            String::from("###...."),
            String::from(".......")
        ]
    );

    test_src.rotate_col(0, 1);
    assert_eq!(
        test_src.render(),
        vec![
            String::from(".##...."),
            String::from("###...."),
            String::from("#......")
        ]
    );
}

#[test]
fn rot_col_2x2() {
    let mut test_src = Screen::new("./data/example_01.txt");
    test_src.load_commands();

    test_src.set_rect(3, 2);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("###...."),
            String::from("###...."),
            String::from(".......")
        ]
    );

    test_src.rotate_col(2, 2);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("###...."),
            String::from("##....."),
            String::from("..#....")
        ]
    );
}

#[test]
fn rot_col_2x3() {
    let mut test_src = Screen::new("./data/example_01.txt");
    test_src.load_commands();

    test_src.set_rect(3, 2);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("###...."),
            String::from("###...."),
            String::from(".......")
        ]
    );

    test_src.rotate_col(2, 3);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("###...."),
            String::from("###...."),
            String::from(".......")
        ]
    );
}

#[test]
fn rot_row_0x4() {
    let mut test_src = Screen::new("./data/example_01.txt");
    test_src.load_commands();

    test_src.set_rect(2, 3);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("##....."),
            String::from("##....."),
            String::from("##.....")
        ]
    );

    test_src.rotate_row(0, 4);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("....##."),
            String::from("##....."),
            String::from("##.....")
        ]
    );
}

#[test]
fn rot_row_1x2() {
    let mut test_src = Screen::new("./data/example_01.txt");
    test_src.load_commands();

    test_src.set_rect(2, 3);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("##....."),
            String::from("##....."),
            String::from("##.....")
        ]
    );

    test_src.rotate_row(1, 2);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("##....."),
            String::from(".##...."),
            String::from("##.....")
        ]
    );
}

#[test]
fn rot_row_3x6() {
    let mut test_src = Screen::new("./data/example_01.txt");
    test_src.load_commands();

    test_src.set_rect(2, 3);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("##....."),
            String::from("##....."),
            String::from("##.....")
        ]
    );

    test_src.rotate_row(3, 6);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("##....."),
            String::from("##....."),
            String::from("#.....#")
        ]
    );
}

#[test]
fn rot_row_0x8() {
    let mut test_src = Screen::new("./data/example_01.txt");
    test_src.load_commands();

    test_src.set_rect(2, 3);
    assert_eq!(
        test_src.render(),
        vec![
            String::from("##....."),
            String::from("##....."),
            String::from("##.....")
        ]
    );

    test_src.rotate_row(0, 8);
    assert_eq!(
        test_src.render(),
        vec![
            String::from(".##...."),
            String::from("##....."),
            String::from("#.....#")
        ]
    );
}

#[test]
fn run_all_commands() {
    let mut test_src = Screen::new("./data/example_01.txt");
    test_src.load_commands();

    test_src.execute_commands();
    assert_eq!(
        test_src.render(),
        vec![
            String::from(".#..#.#"),
            String::from("#.#...."),
            String::from(".#.....")
        ]
    );
}

#[test]
fn count_on_in_exp1() {
    let mut test_src = Screen::new("./data/example_01.txt");
    test_src.load_commands();
    test_src.execute_commands();
    assert_eq!(test_src.on_pixels(), 6);
}
