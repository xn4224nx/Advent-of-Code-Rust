#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod main;
use main::Rocket;

#[test]
fn new_rocket_exp0() {
    let test = Rocket::new("./data/example_1.txt");

    assert_eq!(test.module_masses, vec![12, 14, 1969, 100756]);
}

#[test]
fn module_fuel_exp0() {
    assert_eq!(
        Rocket {
            module_masses: vec![12]
        }
        .fuel_reqs(),
        2
    );
}

#[test]
fn module_fuel_exp1() {
    assert_eq!(
        Rocket {
            module_masses: vec![14]
        }
        .fuel_reqs(),
        2
    );
}

#[test]
fn module_fuel_exp2() {
    assert_eq!(
        Rocket {
            module_masses: vec![1969]
        }
        .fuel_reqs(),
        654
    );
}

#[test]
fn module_fuel_exp3() {
    assert_eq!(
        Rocket {
            module_masses: vec![100756]
        }
        .fuel_reqs(),
        33583
    );
}

#[test]
fn module_fuel_exp4() {
    assert_eq!(Rocket::new("./data/example_1.txt").fuel_reqs(), 34241);
}
