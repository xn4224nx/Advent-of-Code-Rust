#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_16;
use day_16::BinaryBlob;

#[test]
fn create_the_blob() {
    let test = BinaryBlob::new("10111011111001111");
    assert_eq!(test.data, vec![0b1011_1011, 0b1110_0111, 0b1000_0000]);
    assert_eq!(test.bit_cnt, 17);

    let test = BinaryBlob::new("1");
    assert_eq!(test.data, vec![0b1000_0000]);
    assert_eq!(test.bit_cnt, 1);

    let test = BinaryBlob::new("001");
    assert_eq!(test.data, vec![0b0010_0000]);
    assert_eq!(test.bit_cnt, 3);

    let test = BinaryBlob::new("11111000000");
    assert_eq!(test.data, vec![0b1111_1000, 0b0000_0000]);
    assert_eq!(test.bit_cnt, 10);

    let test = BinaryBlob::new("00000000");
    assert_eq!(test.data, vec![0b0000_0000]);
    assert_eq!(test.bit_cnt, 8);
}

#[test]
fn reverse_binary_data() {
    let mut test = BinaryBlob::new("10111011111001111");
    test.reverse();
    assert_eq!(test.data, vec![0b1111_0011, 0b1110_1110, 0b1000_0000]);

    let mut test = BinaryBlob::new("1");
    test.reverse();
    assert_eq!(test.data, vec![0b1000_0000]);

    let mut test = BinaryBlob::new("001");
    test.reverse();
    assert_eq!(test.data, vec![0b1000_0000]);

    let mut test = BinaryBlob::new("11111000000");
    test.reverse();
    assert_eq!(test.data, vec![0b0000_0011, 0b1110_0000]);

    let mut test = BinaryBlob::new("10101010");
    test.reverse();
    assert_eq!(test.data, vec![0b0101_0101]);
}

#[test]
fn invert_binary_data() {
    let mut test = BinaryBlob::new("10111011111001111");
    test.invert();
    assert_eq!(test.data, vec![0b1011_1011, 0b1110_0111, 0b1000_0000]);

    let mut test = BinaryBlob::new("1");
    test.invert();
    assert_eq!(test.data, vec![0b0000_0000]);

    let mut test = BinaryBlob::new("001");
    test.invert();
    assert_eq!(test.data, vec![0b1100_0000]);

    let mut test = BinaryBlob::new("11111000000");
    test.invert();
    assert_eq!(test.data, vec![0b0000_0111, 0b1110_0000]);

    let mut test = BinaryBlob::new("00000000");
    test.invert();
    assert_eq!(test.data, vec![0b1111_1111]);
}

#[test]
fn dragon_curve_the_data() {
    let mut test = BinaryBlob::new("1");
    test.dragon_curve();
    assert_eq!(test.data, vec![0b1000_0000]);
    assert_eq!(test.bit_cnt, 3);

    let mut test = BinaryBlob::new("0");
    test.dragon_curve();
    assert_eq!(test.data, vec![0b0010_0000]);
    assert_eq!(test.bit_cnt, 3);

    let mut test = BinaryBlob::new("11111");
    test.dragon_curve();
    assert_eq!(test.data, vec![0b1111_1000, 0b0000_0000]);
    assert_eq!(test.bit_cnt, 11);

    let mut test = BinaryBlob::new("111100001010");
    test.dragon_curve();
    assert_eq!(
        test.data,
        vec![0b1111_0000, 0b1010_0101, 0b0111_1000, 0b0000_0000]
    );
    assert_eq!(test.bit_cnt, 31);
}

#[test]
fn generate_checksum() {
    let mut test = BinaryBlob::new("110010110100");
    assert_eq!(test.checksum(), "100");

    let mut test = BinaryBlob::new("10000011110010000111");
    assert_eq!(test.checksum(), "01100");
}

#[test]
fn expand_the_data() {
    let mut test = BinaryBlob::new("10000011110010000111");
    test.expand(20);
    assert_eq!(test.data, vec![0b1000_0011, 0b1100_1000, 0b0111_0000]);
    assert_eq!(test.bit_cnt, 20);

    let mut test = BinaryBlob::new("10000011110010000111");
    test.expand(23);
    assert_eq!(test.data, vec![0b1000_0011, 0b1100_1000, 0b0111_1100]);
    assert_eq!(test.bit_cnt, 23);
}

#[test]
fn show_the_data() {
    let test = BinaryBlob::new("10111011111001111");
    assert_eq!(test.show(), "10111011111001111");

    let test = BinaryBlob::new("1");
    assert_eq!(test.show(), "1");

    let test = BinaryBlob::new("001");
    assert_eq!(test.show(), "001");

    let test = BinaryBlob::new("11111000000");
    assert_eq!(test.show(), "11111000000");

    let test = BinaryBlob::new("00000000");
    assert_eq!(test.show(), "00000000");
}

#[test]
fn expanded_and_check() {
    let mut test = BinaryBlob::new("10000011110010000111");
    assert_eq!(test.expanded_check(20), "01100")
}
