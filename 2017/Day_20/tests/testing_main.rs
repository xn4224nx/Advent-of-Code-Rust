#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_20;
use day_20::{Particle, Throng};

#[test]
fn new_throng_exp01() {
    assert_eq!(
        Throng::new("./data/example_01.txt"),
        Throng {
            contents: vec![
                Particle {
                    position: vec![3, 0, 0],
                    velocity: vec![2, 0, 0],
                    acceleration: vec![-1, 0, 0],
                },
                Particle {
                    position: vec![4, 0, 0],
                    velocity: vec![0, 0, 0],
                    acceleration: vec![-2, 0, 0],
                },
            ],
        }
    );
}

#[test]
fn new_throng_exp02() {
    assert_eq!(
        Throng::new("./data/example_02.txt"),
        Throng {
            contents: vec![
                Particle {
                    position: vec![4, 0, 0],
                    velocity: vec![1, 0, 0],
                    acceleration: vec![-1, 0, 0],
                },
                Particle {
                    position: vec![2, 0, 0],
                    velocity: vec![-2, 0, 0],
                    acceleration: vec![-2, 0, 0],
                },
            ],
        }
    );
}

#[test]
fn new_throng_exp03() {
    assert_eq!(
        Throng::new("./data/example_03.txt"),
        Throng {
            contents: vec![
                Particle {
                    position: vec![4, 0, 0],
                    velocity: vec![0, 0, 0],
                    acceleration: vec![-1, 0, 0],
                },
                Particle {
                    position: vec![-2, 0, 0],
                    velocity: vec![-4, 0, 0],
                    acceleration: vec![-2, 0, 0],
                },
            ],
        }
    );
}

#[test]
fn new_throng_exp04() {
    assert_eq!(
        Throng::new("./data/example_04.txt"),
        Throng {
            contents: vec![
                Particle {
                    position: vec![3, 0, 0],
                    velocity: vec![-1, 0, 0],
                    acceleration: vec![-1, 0, 0],
                },
                Particle {
                    position: vec![-8, 0, 0],
                    velocity: vec![-6, 0, 0],
                    acceleration: vec![-2, 0, 0],
                },
            ],
        }
    );
}

#[test]
fn new_throng_exp05() {
    assert_eq!(
        Throng::new("./data/example_05.txt"),
        Throng {
            contents: vec![
                Particle {
                    position: vec![-6, 0, 0],
                    velocity: vec![3, 0, 0],
                    acceleration: vec![0, 0, 0],
                },
                Particle {
                    position: vec![-4, 0, 0],
                    velocity: vec![2, 0, 0],
                    acceleration: vec![0, 0, 0],
                },
                Particle {
                    position: vec![-2, 0, 0],
                    velocity: vec![1, 0, 0],
                    acceleration: vec![0, 0, 0],
                },
                Particle {
                    position: vec![3, 0, 0],
                    velocity: vec![-1, 0, 0],
                    acceleration: vec![0, 0, 0],
                },
            ],
        }
    );
}

#[test]
fn new_particle_exp01() {
    assert_eq!(
        Particle::new("p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>"),
        Particle {
            position: vec![3, 0, 0],
            velocity: vec![2, 0, 0],
            acceleration: vec![-1, 0, 0],
        }
    );
}

#[test]
fn new_particle_exp02() {
    assert_eq!(
        Particle::new("p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>"),
        Particle {
            position: vec![4, 0, 0],
            velocity: vec![0, 0, 0],
            acceleration: vec![-2, 0, 0],
        }
    );
}

#[test]
fn new_particle_exp03() {
    assert_eq!(
        Particle::new("p=< 4,0,0>, v=< 1,0,0>, a=<-1,0,0>"),
        Particle {
            position: vec![4, 0, 0],
            velocity: vec![1, 0, 0],
            acceleration: vec![-1, 0, 0],
        }
    );
}

#[test]
fn new_particle_exp04() {
    assert_eq!(
        Particle::new("p=< 2,0,0>, v=<-2,0,0>, a=<-2,0,0>"),
        Particle {
            position: vec![2, 0, 0],
            velocity: vec![-2, 0, 0],
            acceleration: vec![-2, 0, 0],
        }
    );
}

#[test]
fn new_particle_exp05() {
    assert_eq!(
        Particle::new("p=< 4,0,0>, v=< 0,0,0>, a=<-1,0,0>"),
        Particle {
            position: vec![4, 0, 0],
            velocity: vec![0, 0, 0],
            acceleration: vec![-1, 0, 0],
        }
    );
}

#[test]
fn new_particle_exp06() {
    assert_eq!(
        Particle::new("p=<-2,0,0>, v=<-4,0,0>, a=<-2,0,0>"),
        Particle {
            position: vec![-2, 0, 0],
            velocity: vec![-4, 0, 0],
            acceleration: vec![-2, 0, 0],
        }
    );
}

#[test]
fn new_particle_exp07() {
    assert_eq!(
        Particle::new("p=< 3,0,0>, v=<-1,0,0>, a=<-1,0,0>"),
        Particle {
            position: vec![3, 0, 0],
            velocity: vec![-1, 0, 0],
            acceleration: vec![-1, 0, 0],
        }
    );
}

#[test]
fn new_particle_exp08() {
    assert_eq!(
        Particle::new("p=<-8,0,0>, v=<-6,0,0>, a=<-2,0,0>"),
        Particle {
            position: vec![-8, 0, 0],
            velocity: vec![-6, 0, 0],
            acceleration: vec![-2, 0, 0],
        }
    );
}

#[test]
fn new_particle_exp09() {
    assert_eq!(
        Particle::new("p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>"),
        Particle {
            position: vec![-6, 0, 0],
            velocity: vec![3, 0, 0],
            acceleration: vec![0, 0, 0],
        }
    );
}

#[test]
fn new_particle_exp10() {
    assert_eq!(
        Particle::new("p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>"),
        Particle {
            position: vec![-4, 0, 0],
            velocity: vec![2, 0, 0],
            acceleration: vec![0, 0, 0],
        }
    );
}

#[test]
fn new_particle_exp11() {
    assert_eq!(
        Particle::new("p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>"),
        Particle {
            position: vec![-2, 0, 0],
            velocity: vec![1, 0, 0],
            acceleration: vec![0, 0, 0],
        }
    );
}

#[test]
fn new_particle_exp12() {
    assert_eq!(
        Particle::new("p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>"),
        Particle {
            position: vec![3, 0, 0],
            velocity: vec![-1, 0, 0],
            acceleration: vec![0, 0, 0],
        }
    );
}

#[test]
fn step_exp01() {
    let mut test = Particle::new("p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>");
    test.step();
    assert_eq!(test, Particle::new("p=< 4,0,0>, v=< 1,0,0>, a=<-1,0,0>"));
}

#[test]
fn step_exp02() {
    let mut test = Particle::new("p=< 4,0,0>, v=< 1,0,0>, a=<-1,0,0>");
    test.step();
    assert_eq!(test, Particle::new("p=< 4,0,0>, v=< 0,0,0>, a=<-1,0,0>"));
}

#[test]
fn step_exp03() {
    let mut test = Particle::new("p=< 4,0,0>, v=< 0,0,0>, a=<-1,0,0>");
    test.step();
    assert_eq!(test, Particle::new("p=< 3,0,0>, v=<-1,0,0>, a=<-1,0,0>"));
}

#[test]
fn step_exp04() {
    let mut test = Particle::new("p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>");
    test.step();
    assert_eq!(test, Particle::new("p=< 2,0,0>, v=<-2,0,0>, a=<-2,0,0>"));
}

#[test]
fn step_exp05() {
    let mut test = Particle::new("p=< 2,0,0>, v=<-2,0,0>, a=<-2,0,0>");
    test.step();
    assert_eq!(test, Particle::new("p=<-2,0,0>, v=<-4,0,0>, a=<-2,0,0> "));
}

#[test]
fn step_exp06() {
    let mut test = Particle::new("p=<-2,0,0>, v=<-4,0,0>, a=<-2,0,0> ");
    test.step();
    assert_eq!(test, Particle::new("p=<-8,0,0>, v=<-6,0,0>, a=<-2,0,0>"));
}

#[test]
fn dist_from_origin_exp01() {
    assert_eq!(
        Particle::new("p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>").dist_from_origin(),
        3
    );
}

#[test]
fn dist_from_origin_exp02() {
    assert_eq!(
        Particle::new("p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>").dist_from_origin(),
        4
    );
}

#[test]
fn dist_from_origin_exp03() {
    assert_eq!(
        Particle::new("p=< 4,0,0>, v=< 1,0,0>, a=<-1,0,0>").dist_from_origin(),
        4
    );
}

#[test]
fn dist_from_origin_exp04() {
    assert_eq!(
        Particle::new("p=< 2,0,0>, v=<-2,0,0>, a=<-2,0,0>").dist_from_origin(),
        2
    );
}

#[test]
fn dist_from_origin_exp05() {
    assert_eq!(
        Particle::new("p=< 4,0,0>, v=< 0,0,0>, a=<-1,0,0>").dist_from_origin(),
        4
    );
}

#[test]
fn dist_from_origin_exp06() {
    assert_eq!(
        Particle::new("p=<-2,0,0>, v=<-4,0,0>, a=<-2,0,0>").dist_from_origin(),
        2
    );
}

#[test]
fn dist_from_origin_exp07() {
    assert_eq!(
        Particle::new("p=< 3,0,0>, v=<-1,0,0>, a=<-1,0,0>").dist_from_origin(),
        3
    );
}

#[test]
fn dist_from_origin_exp08() {
    assert_eq!(
        Particle::new("p=<-8,0,0>, v=<-6,0,0>, a=<-2,0,0>").dist_from_origin(),
        8
    );
}

#[test]
fn long_term_closest_to_origin_exp01() {
    assert_eq!(
        Throng::new("./data/example_01.txt").long_term_closest_to_origin(),
        0
    );
}

#[test]
fn remaining_particles_exp01() {
    assert_eq!(
        Throng::new("./data/example_05.txt").remaining_particles(),
        1
    );
}
