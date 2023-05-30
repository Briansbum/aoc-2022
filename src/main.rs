mod utils;

fn main() {
    println!("day1_1: {}", day1_1("src/data/day1.txt"));
    println!("day1_2: {}", day1_2("src/data/day1.txt"));
}

fn day1_2(filename: &str) -> usize {
    let mut cals = utils::elf_snack_cals(filename);
    cals.sort();
    cals.reverse();
    cals.truncate(3);
    let mut ret = 0;
    cals.iter().for_each(|c| ret += c);
    return ret
}

fn day1_1(filename: &str) -> usize {
    let mut d = &0;
    let cals = utils::elf_snack_cals(filename);
    for (_, c) in cals.iter().enumerate() {
        if c > d {
            d = c;
        }
    }
    return *d;
}


