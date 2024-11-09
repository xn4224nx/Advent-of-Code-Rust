#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_09;
use day_09::{decompressed_len, read_compressed_data};

#[test]
fn read_exp_data_1() {
    assert_eq!(
        read_compressed_data("./data/example_01.txt"),
        "ADVENT".as_bytes().to_vec()
    );
}

#[test]
fn read_exp_data_2() {
    assert_eq!(
        read_compressed_data("./data/example_02.txt"),
        "A(1x5)BC".as_bytes().to_vec()
    );
}

#[test]
fn read_exp_data_3() {
    assert_eq!(
        read_compressed_data("./data/example_03.txt"),
        "(3x3)XYZ".as_bytes().to_vec()
    );
}

#[test]
fn read_exp_data_4() {
    assert_eq!(
        read_compressed_data("./data/example_04.txt"),
        "A(2x2)BCD(2x2)EFG".as_bytes().to_vec()
    );
}

#[test]
fn read_exp_data_5() {
    assert_eq!(
        read_compressed_data("./data/example_05.txt"),
        "(6x1)(1x3)A".as_bytes().to_vec()
    );
}

#[test]
fn read_exp_data_6() {
    assert_eq!(
        read_compressed_data("./data/example_06.txt"),
        "X(8x2)(3x3)ABCY".as_bytes().to_vec()
    );
}

#[test]
fn read_exp_data_7() {
    assert_eq!(
        read_compressed_data("./data/example_07.txt"),
        "(27x12)(20x12)(13x14)(7x10)(1x12)A".as_bytes().to_vec()
    );
}

#[test]
fn read_exp_data_8() {
    assert_eq!(
        read_compressed_data("./data/example_08.txt"),
        "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"
            .as_bytes()
            .to_vec()
    );
}

#[test]
fn decomp_len_exp_1() {
    assert_eq!(decompressed_len(&"ADVENT".as_bytes().to_vec()), 6)
}

#[test]
fn decomp_len_exp_2() {
    assert_eq!(decompressed_len(&"A(1x5)BC".as_bytes().to_vec()), 7)
}

#[test]
fn decomp_len_exp_3() {
    assert_eq!(decompressed_len(&"(3x3)XYZ".as_bytes().to_vec()), 9)
}

#[test]
fn decomp_len_exp_4() {
    assert_eq!(
        decompressed_len(&"A(2x2)BCD(2x2)EFG".as_bytes().to_vec()),
        11
    )
}

#[test]
fn decomp_len_exp_5() {
    assert_eq!(decompressed_len(&"(6x1)(1x3)A".as_bytes().to_vec()), 6)
}

#[test]
fn decomp_len_exp_6() {
    assert_eq!(decompressed_len(&"X(8x2)(3x3)ABCY".as_bytes().to_vec()), 18)
}