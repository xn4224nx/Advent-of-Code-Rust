#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_04;
use day_04::{read_rooms, sum_real_rooms, Room};

#[test]
fn read_exp1_rooms() {
    assert_eq!(
        read_rooms("./data/example_01.txt"),
        vec![
            Room::new(String::from("aaaaa-bbb-z-y-x"), 123, String::from("abxyz")),
            Room::new(String::from("a-b-c-d-e-f-g-h"), 987, String::from("abcde")),
            Room::new(String::from("not-a-real-room"), 404, String::from("oarel")),
            Room::new(
                String::from("totally-real-room"),
                200,
                String::from("decoy")
            ),
        ]
    )
}

#[test]
fn verify_room_1() {
    let test_rm = Room::new(String::from("aaaaa-bbb-z-y-x"), 123, String::from("abxyz"));
    assert_eq!(test_rm.verify(), true);
}

#[test]
fn verify_room_2() {
    let test_rm = Room::new(String::from("a-b-c-d-e-f-g-h"), 987, String::from("abcde"));
    assert_eq!(test_rm.verify(), true);
}

#[test]
fn verify_room_3() {
    let test_rm = Room::new(String::from("not-a-real-room"), 404, String::from("oarel"));
    assert_eq!(test_rm.verify(), true);
}

#[test]
fn verify_room_4() {
    let test_rm = Room::new(
        String::from("totally-real-room"),
        200,
        String::from("decoy"),
    );
    assert_eq!(test_rm.verify(), false);
}

#[test]
fn sum_real__example_01_rooms() {
    let exp1_rms = read_rooms("./data/example_01.txt");
    assert_eq!(sum_real_rooms(&exp1_rms), 1514)
}
