#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_19;
use day_19::find_final_elf;

#[test]
fn final_elf_expl_01() {
    assert_eq!(find_final_elf(5), 3);
}
