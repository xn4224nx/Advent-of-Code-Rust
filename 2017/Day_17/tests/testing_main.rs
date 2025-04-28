#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_17;
use day_17::Tornado;

#[test]
fn new_class_exp01() {
    let test = Tornado::new(3);
    assert_eq!(test.buffer, vec![0]);
    assert_eq!(test.step, 3);
    assert_eq!(test.position, 1);
}

#[test]
fn spin_exp01() {
    let mut test = Tornado::new(3);
    test.buffer = vec![0];
    test.position = 1;
    test.spin();
    assert_eq!(test.buffer, vec![0, 1]);
    assert_eq!(test.position, 2);
}

#[test]
fn spin_exp02() {
    let mut test = Tornado::new(3);
    test.buffer = vec![0, 1];
    test.position = 2;
    test.spin();
    assert_eq!(test.buffer, vec![1, 0, 2]);
    assert_eq!(test.position, 3);
}

#[test]
fn spin_exp03() {
    let mut test = Tornado::new(3);
    test.buffer = vec![1, 0, 2];
    test.position = 3;
    test.spin();
    assert_eq!(test.buffer, vec![1, 0, 2, 3]);
    assert_eq!(test.position, 4);
}

#[test]
fn spin_exp04() {
    let mut test = Tornado::new(3);
    test.buffer = vec![1, 0, 2, 3];
    test.position = 4;
    test.spin();
    assert_eq!(test.buffer, vec![3, 1, 0, 2, 4]);
    assert_eq!(test.position, 5);
}

#[test]
fn spin_exp05() {
    let mut test = Tornado::new(3);
    test.buffer = vec![3, 1, 0, 2, 4];
    test.position = 5;
    test.spin();
    assert_eq!(test.buffer, vec![2, 4, 3, 1, 0, 5]);
    assert_eq!(test.position, 6);
}

#[test]
fn spin_exp06() {
    let mut test = Tornado::new(3);
    test.buffer = vec![2, 4, 3, 1, 0, 5];
    test.position = 6;
    test.spin();
    assert_eq!(test.buffer, vec![1, 0, 5, 2, 4, 3, 6]);
    assert_eq!(test.position, 7);
}

#[test]
fn spin_exp07() {
    let mut test = Tornado::new(3);
    test.buffer = vec![1, 0, 5, 2, 4, 3, 6];
    test.position = 7;
    test.spin();
    assert_eq!(test.buffer, vec![2, 4, 3, 6, 1, 0, 5, 7]);
    assert_eq!(test.position, 8);
}

#[test]
fn spin_exp08() {
    let mut test = Tornado::new(3);
    test.buffer = vec![2, 4, 3, 6, 1, 0, 5, 7];
    test.position = 8;
    test.spin();
    assert_eq!(test.buffer, vec![6, 1, 0, 5, 7, 2, 4, 3, 8]);
    assert_eq!(test.position, 9);
}

#[test]
fn spin_exp09() {
    let mut test = Tornado::new(3);
    test.buffer = vec![6, 1, 0, 5, 7, 2, 4, 3, 8];
    test.position = 9;
    test.spin();
    assert_eq!(test.buffer, vec![5, 7, 2, 4, 3, 8, 6, 1, 0, 9]);
    assert_eq!(test.position, 10);
}

#[test]
fn spin_exp10() {
    let mut test = Tornado::new(3);
    for _ in 0..9 {
        test.spin();
    }

    assert_eq!(test.buffer, vec![5, 7, 2, 4, 3, 8, 6, 1, 0, 9]);
    assert_eq!(test.position, 10);
}

#[test]
fn value_after_exp01() {
    assert_eq!(Tornado::new(3).value_after(8, 8), 6);
}

#[test]
fn value_after_exp02() {
    assert_eq!(Tornado::new(3).value_after(9, 9), 5);
}

#[test]
fn value_after_exp03() {
    //assert_eq!(Tornado::new(3).value_after(2017, 2017), 638);
}
