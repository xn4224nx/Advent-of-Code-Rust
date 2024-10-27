#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_07;
use day_07::{comp_has_abba, ip_support_tls, read_ip_addresses, AddrComp};

#[test]
fn read_ip_addresses_exp1() {
    assert_eq!(
        read_ip_addresses("./data/example_01.txt"),
        vec![
            vec![
                AddrComp::Exter(vec![97, 98, 98, 97]),
                AddrComp::Inter(vec![109, 110, 111, 112,]),
                AddrComp::Exter(vec![113, 114, 115, 116])
            ],
            vec![
                AddrComp::Exter(vec![97, 98, 99, 100]),
                AddrComp::Inter(vec![98, 100, 100, 98,]),
                AddrComp::Exter(vec![120, 121, 121, 120,])
            ],
            vec![
                AddrComp::Exter(vec![97, 97, 97, 97]),
                AddrComp::Inter(vec![113, 119, 101, 114,]),
                AddrComp::Exter(vec![116, 121, 117, 105,])
            ],
            vec![
                AddrComp::Exter(vec![105, 111, 120, 120, 111, 106]),
                AddrComp::Inter(vec![97, 115, 100, 102, 103, 104,]),
                AddrComp::Exter(vec![122, 120, 99, 118, 98, 110,])
            ],
        ]
    )
}

#[test]
fn read_ip_addresses_exp2() {
    assert_eq!(
        read_ip_addresses("./data/example_02.txt"),
        vec![
            vec![
                AddrComp::Exter(vec![97, 98, 97,]),
                AddrComp::Inter(vec![98, 97, 98,]),
                AddrComp::Exter(vec![120, 121, 122,])
            ],
            vec![
                AddrComp::Exter(vec![120, 121, 120,]),
                AddrComp::Inter(vec![120, 121, 120,]),
                AddrComp::Exter(vec![120, 121, 120,])
            ],
            vec![
                AddrComp::Exter(vec![97, 97, 97,]),
                AddrComp::Inter(vec![107, 101, 107,]),
                AddrComp::Exter(vec![101, 107, 101,])
            ],
            vec![
                AddrComp::Exter(vec![122, 97, 122, 98, 122,]),
                AddrComp::Inter(vec![98, 122, 98,]),
                AddrComp::Exter(vec![99, 100, 98,])
            ],
        ]
    )
}

#[test]
fn ip_support_tls_exp1() {
    assert_eq!(
        ip_support_tls(vec![
            AddrComp::Exter(vec![97, 98, 98, 97]),
            AddrComp::Inter(vec![109, 110, 111, 112,]),
            AddrComp::Exter(vec![113, 114, 115, 116])
        ]),
        true
    )
}

#[test]
fn ip_support_tls_exp2() {
    assert_eq!(
        ip_support_tls(vec![
            AddrComp::Exter(vec![97, 98, 99, 100]),
            AddrComp::Inter(vec![98, 100, 100, 98,]),
            AddrComp::Exter(vec![120, 121, 121, 120,])
        ]),
        false
    )
}

#[test]
fn ip_support_tls_exp3() {
    assert_eq!(
        ip_support_tls(vec![
            AddrComp::Exter(vec![97, 97, 97, 97]),
            AddrComp::Inter(vec![113, 119, 101, 114,]),
            AddrComp::Exter(vec![116, 121, 117, 105,])
        ]),
        false
    )
}

#[test]
fn ip_support_tls_exp4() {
    assert_eq!(
        ip_support_tls(vec![
            AddrComp::Exter(vec![105, 111, 120, 120, 111, 106]),
            AddrComp::Inter(vec![97, 115, 100, 102, 103, 104,]),
            AddrComp::Exter(vec![122, 120, 99, 118, 98, 110,])
        ]),
        true
    )
}

#[test]
fn comp_has_abba_exp1() {
    assert_eq!(comp_has_abba(vec![97, 98, 98, 97]), true)
}

#[test]
fn comp_has_abba_exp2() {
    assert_eq!(comp_has_abba(vec![109, 110, 111, 112,]), false)
}

#[test]
fn comp_has_abba_exp3() {
    assert_eq!(comp_has_abba(vec![113, 114, 115, 116]), false)
}

#[test]
fn comp_has_abba_exp4() {
    assert_eq!(comp_has_abba(vec![97, 98, 99, 100]), false)
}

#[test]
fn comp_has_abba_exp5() {
    assert_eq!(comp_has_abba(vec![98, 100, 100, 98,]), true)
}

#[test]
fn comp_has_abba_exp6() {
    assert_eq!(comp_has_abba(vec![120, 121, 121, 120,]), true)
}

#[test]
fn comp_has_abba_exp7() {
    assert_eq!(comp_has_abba(vec![97, 97, 97, 97]), false)
}

#[test]
fn comp_has_abba_exp8() {
    assert_eq!(comp_has_abba(vec![113, 119, 101, 114,]), false)
}

#[test]
fn comp_has_abba_exp9() {
    assert_eq!(comp_has_abba(vec![116, 121, 117, 105,]), false)
}

#[test]
fn comp_has_abba_exp10() {
    assert_eq!(comp_has_abba(vec![105, 111, 120, 120, 111, 106]), true)
}

#[test]
fn comp_has_abba_exp11() {
    assert_eq!(comp_has_abba(vec![97, 115, 100, 102, 103, 104,]), false)
}

#[test]
fn comp_has_abba_exp12() {
    assert_eq!(comp_has_abba(vec![122, 120, 99, 118, 98, 110,]), false)
}
