#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_16;
use day_16::BinaryBlob;

#[test]
fn create_the_blob() {
    let test = BinaryBlob::new("10111011111001111");
    assert_eq!(
        test.data,
        vec![1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1]
    );

    let test = BinaryBlob::new("1");
    assert_eq!(test.data, vec![1]);

    let test = BinaryBlob::new("001");
    assert_eq!(test.data, vec![0, 0, 1]);

    let test = BinaryBlob::new("11111000000");
    assert_eq!(test.data, vec![1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0]);

    let test = BinaryBlob::new("00000000");
    assert_eq!(test.data, vec![0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn reverse_binary_data() {
    let mut test = BinaryBlob::new("10111011111001111");
    test.reverse();
    assert_eq!(
        test.data,
        vec![1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1]
    );

    let mut test = BinaryBlob::new("1");
    test.reverse();
    assert_eq!(test.data, vec![1]);

    let mut test = BinaryBlob::new("001");
    test.reverse();
    assert_eq!(test.data, vec![1, 0, 0]);

    let mut test = BinaryBlob::new("11111000000");
    test.reverse();
    assert_eq!(test.data, vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1]);

    let mut test = BinaryBlob::new("10101010");
    test.reverse();
    assert_eq!(test.data, vec![0, 1, 0, 1, 0, 1, 0, 1]);
}

#[test]
fn invert_binary_data() {
    let mut test = BinaryBlob::new("10111011111001111");
    test.invert();
    assert_eq!(
        test.data,
        vec![0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
    );

    let mut test = BinaryBlob::new("1");
    test.invert();
    assert_eq!(test.data, vec![0]);

    let mut test = BinaryBlob::new("001");
    test.invert();
    assert_eq!(test.data, vec![1, 1, 0]);

    let mut test = BinaryBlob::new("11111000000");
    test.invert();
    assert_eq!(test.data, vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1]);

    let mut test = BinaryBlob::new("00000000");
    test.invert();
    assert_eq!(test.data, vec![1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn dragon_curve_the_data() {
    let mut test = BinaryBlob::new("1");
    test.dragon_curve();
    assert_eq!(test.data, vec![1, 0, 0]);

    let mut test = BinaryBlob::new("0");
    test.dragon_curve();
    assert_eq!(test.data, vec![0, 0, 1]);

    let mut test = BinaryBlob::new("11111");
    test.dragon_curve();
    assert_eq!(test.data, vec![1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0]);

    let mut test = BinaryBlob::new("111100001010");
    test.dragon_curve();
    assert_eq!(
        test.data,
        vec![1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0]
    );
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
    let mut test = BinaryBlob::new("10000");
    test.expand(20);
    assert_eq!(
        test.data,
        vec![1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1]
    );

    let mut test = BinaryBlob::new("10000");
    test.expand(23);
    assert_eq!(
        test.data,
        vec![1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0]
    );
}

#[test]
fn show_the_data() {
    let test = BinaryBlob::new("10111011111001111");
    assert_eq!(test.show(&test.data), "10111011111001111");

    let test = BinaryBlob::new("1");
    assert_eq!(test.show(&test.data), "1");

    let test = BinaryBlob::new("001");
    assert_eq!(test.show(&test.data), "001");

    let test = BinaryBlob::new("11111000000");
    assert_eq!(test.show(&test.data), "11111000000");

    let test = BinaryBlob::new("00000000");
    assert_eq!(test.show(&test.data), "00000000");
}

#[test]
fn expanded_and_check() {
    let mut test = BinaryBlob::new("10000011110010000111");
    assert_eq!(test.expanded_check(20), "01100")
}
