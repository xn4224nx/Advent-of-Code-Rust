#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_18;
use day_18::TrapRoom;

#[test]
fn read_data_file() {
    assert_eq!(
        TrapRoom::new("./data/example_01.txt").rows,
        vec![vec![true, true, false, false, true,]]
    );
    assert_eq!(
        TrapRoom::new("./data/example_02.txt").rows,
        vec![vec![
            true, false, false, true, false, true, false, false, false, false,
        ]]
    );
}

#[test]
fn show_rows() {
    assert_eq!(
        TrapRoom::new("./data/example_01.txt").show(),
        String::from("..^^.")
    );
    assert_eq!(
        TrapRoom::new("./data/example_02.txt").show(),
        String::from(".^^.^.^^^^")
    );
}

#[test]
fn predict_further_rows_exp1() {
    let mut test = TrapRoom::new("./data/example_01.txt");
    assert_eq!(test.rows, vec![vec![true, true, false, false, true,]]);

    test.predict_rows(3);

    assert_eq!(
        test.rows,
        vec![
            vec![true, true, false, false, true,],
            vec![true, false, false, false, false,],
            vec![false, false, true, true, false,],
        ]
    );
}

#[test]
fn predict_further_rows_exp2() {
    let mut test = TrapRoom::new("./data/example_02.txt");
    assert_eq!(test.rows, vec![vec![true, true, false, false, true,]]);

    test.predict_rows(10);

    assert_eq!(
        test.rows,
        vec![
            vec![true, false, false, true, false, true, false, false, false, false,],
            vec![false, false, false, true, true, true, false, true, true, false,],
            vec![false, true, false, false, true, false, true, false, false, true,],
            vec![true, true, false, false, true, true, true, false, false, false,],
            vec![true, false, false, false, false, true, false, false, true, false,],
            vec![false, false, true, true, false, true, false, false, true, true,],
            vec![false, false, false, false, true, true, false, false, false, true,],
            vec![false, true, true, false, false, false, false, true, false, false,],
            vec![true, false, false, false, true, true, false, true, false, false,],
            vec![false, false, true, false, false, false, true, true, false, false,],
        ]
    );
}

#[test]
fn num_safe_tiles() {
    let mut test = TrapRoom::new("./data/example_01.txt");

    test.rows = vec![
        vec![true, true, false, false, true],
        vec![true, false, false, false, false],
        vec![false, false, true, true, false],
    ];
    assert_eq!(test.num_safe_tiles(), 6);

    test.rows = vec![
        vec![
            true, false, false, true, false, true, false, false, false, false,
        ],
        vec![
            false, false, false, true, true, true, false, true, true, false,
        ],
        vec![
            false, true, false, false, true, false, true, false, false, true,
        ],
        vec![
            true, true, false, false, true, true, true, false, false, false,
        ],
        vec![
            true, false, false, false, false, true, false, false, true, false,
        ],
        vec![
            false, false, true, true, false, true, false, false, true, true,
        ],
        vec![
            false, false, false, false, true, true, false, false, false, true,
        ],
        vec![
            false, true, true, false, false, false, false, true, false, false,
        ],
        vec![
            true, false, false, false, true, true, false, true, false, false,
        ],
        vec![
            false, false, true, false, false, false, true, true, false, false,
        ],
    ];
    assert_eq!(test.num_safe_tiles(), 38);
}
