mod utils;
use crate::utils::play_game;
use crate::utils::Play;
use std::str::FromStr;

fn main() {
    println!("day1_1: {}", day1_1("src/data/day1.txt"));
    println!("day1_2: {}", day1_2("src/data/day1.txt"));
    println!("day2_1: {}", day2_1("src/data/day2.txt"));
    println!("day2_2: {}", day2_2("src/data/day2.txt"));
    println!("day3_1: {}", day3_1("src/data/day3.txt"));
    println!("day3_2: {}", day3_2("src/data/day3.txt"));
    println!("day4_1: {}", day4_1("src/data/day4.txt"));
    println!("day4_2: {}", day4_2("src/data/day4.txt"));
    println!("day5_1: {:?}", day5_1("src/data/day5.txt"));
    println!("day5_2: {:?}", day5_2("src/data/day5.txt"));
    println!("day6_1: {}", day6_1("src/data/day6.txt"));
    println!("day6_2: {}", day6_2("src/data/day6.txt"));
    println!("day7_1: {}", day7_1("src/data/day7.txt"));
    println!("day7_2: {}", day7_2("src/data/day7.txt"));
    println!("day8_1: {}", day8_1("src/fixtures/day8.txt"));
}

fn day1_2(filename: &str) -> usize {
    let mut cals = utils::elf_snack_cals(filename);
    cals.sort();
    cals.reverse();
    cals.truncate(3);
    let mut ret = 0;
    cals.iter().for_each(|c| ret += c);
    return ret;
}

fn day1_1(filename: &str) -> usize {
    utils::elf_snack_cals(filename)
        .iter()
        .fold(0, |mut acc: usize, &c| {
            if c > acc {
                acc = c;
            }
            return acc;
        })
}

fn day2_2(filename: &str) -> usize {
    let f = utils::readfile(filename);
    f.lines()
        .map(|l| l.split_whitespace().collect())
        .map(|v: Vec<&str>| {
            let them = Play::from_str(v[0]).unwrap();
            let me = Play::from_str(v[1]).unwrap();
            match me {
                Play::Paper => match them {
                    Play::Paper => play_game(them, Play::Paper),
                    Play::Rock => play_game(them, Play::Rock),
                    Play::Scissors => play_game(them, Play::Scissors),
                },
                Play::Rock => match them {
                    Play::Paper => play_game(them, Play::Rock),
                    Play::Rock => play_game(them, Play::Scissors),
                    Play::Scissors => play_game(them, Play::Paper),
                },
                Play::Scissors => match them {
                    Play::Paper => play_game(them, Play::Scissors),
                    Play::Rock => play_game(them, Play::Paper),
                    Play::Scissors => play_game(them, Play::Rock),
                },
            }
        })
        .fold(0, |acc, v| acc + v)
}

fn day2_1(filename: &str) -> usize {
    let f = utils::readfile(filename);
    f.lines()
        .map(|l| l.split_whitespace().collect())
        .map(|v: Vec<&str>| play_game(Play::from_str(v[0]).unwrap(), Play::from_str(v[1]).unwrap()))
        .fold(0, |acc, v| acc + v)
}

fn day3_1(filename: &str) -> usize {
    let f = utils::readfile(filename);

    let l: Vec<usize> = f
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_at(line.len() / 2);
            let mut p = p1
                .chars()
                .filter(|c| match p2.chars().find(|v| *c == *v) {
                    Some(_) => true,
                    None => false,
                })
                .collect::<Vec<char>>();

            p.sort();
            p.dedup();

            p.iter().fold(0, |acc, v| {
                match utils::LETTER_VALS.iter().find(|lv| lv.0 == *v) {
                    Some(i) => acc + i.1,
                    None => acc,
                }
            })
        })
        .collect();
    return l.iter().fold(0, |acc, x| acc + x);
}

fn day3_2(filename: &str) -> usize {
    let f = utils::readfile(filename);

    let ff = f
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match utils::LETTER_VALS.iter().find(|lv| lv.0 == c) {
                    Some(f) => f.1,
                    None => 0,
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut counter = 0;
    let mut result = 0;
    return loop {
        if counter >= ff.len() - 1 {
            break result;
        }
        for s in &ff[counter] {
            if ff[counter + 1].contains(s) && ff[counter + 2].contains(s) {
                result += s;
                break;
            }
        }
        counter += 3;
    };
}

fn day4_1(filename: &str) -> usize {
    let f = utils::readfile(filename);

    let mut ret: usize = 0;
    f.lines()
        .filter(|line| {
            let line_split: Vec<&str> = line.split(",").collect();

            let elf1_split: Vec<&str> = line_split[0].split("-").collect();
            let elf2_split: Vec<&str> = line_split[1].split("-").collect();

            let elf1_start = elf1_split[0].parse::<usize>().unwrap();
            let elf1_end = elf1_split[1].parse::<usize>().unwrap();
            let elf2_start = elf2_split[0].parse::<usize>().unwrap();
            let elf2_end = elf2_split[1].parse::<usize>().unwrap();

            if elf1_start <= elf2_start && elf1_end >= elf2_end {
                return true;
            }
            if elf2_start <= elf1_start && elf2_end >= elf1_end {
                return true;
            }

            return false;
        })
        .map(|_| {
            ret += 1;
        })
        .last();

    return ret;
}

fn day4_2(filename: &str) -> usize {
    let f = utils::readfile(filename);

    let mut ret: usize = 0;
    f.lines()
        .filter(|line| {
            let line_split: Vec<&str> = line.split(",").collect();

            let elf1_split: Vec<&str> = line_split[0].split("-").collect();
            let elf2_split: Vec<&str> = line_split[1].split("-").collect();

            let elf1_start = elf1_split[0].parse::<usize>().unwrap();
            let elf1_end = elf1_split[1].parse::<usize>().unwrap();
            let elf2_start = elf2_split[0].parse::<usize>().unwrap();
            let elf2_end = elf2_split[1].parse::<usize>().unwrap();

            if elf1_start <= elf2_start && elf1_end >= elf2_end {
                return true;
            }
            if elf2_start <= elf1_start && elf2_end >= elf1_end {
                return true;
            }

            if elf1_start < elf2_start && elf1_end >= elf2_start && elf1_end < elf2_end {
                return true;
            }
            if elf2_start < elf1_start && elf2_end >= elf1_start && elf2_end < elf1_end {
                return true;
            }

            if elf1_end > elf2_end && elf1_start <= elf2_end && elf1_start > elf2_start {
                return true;
            }
            if elf2_end > elf1_end && elf2_start <= elf1_end && elf2_start > elf1_start {
                return true;
            }

            return false;
        })
        .map(|_| {
            ret += 1;
        })
        .last();

    return ret;
}

fn day5_1(filename: &str) -> Vec<String> {
    let f = utils::readfile(filename);

    let mut crates = utils::extract_crates(f.clone());

    for line in f.lines() {
        if !line.starts_with("move") {
            continue;
        }

        let split: Vec<&str> = line.split(" ").collect();
        let move_count = split[1].parse::<usize>().unwrap();
        let from = split[3].parse::<usize>().unwrap() - 1;
        let to = split[5].parse::<usize>().unwrap() - 1;

        utils::crate_mover_9000(move_count, &mut crates, from, to);
    }

    let mut out: Vec<String> = vec![];
    for stack in crates {
        let s = stack.last();
        match s {
            Some(s) => out.push(s.clone()),
            None => (),
        }
    }

    return out;
}

fn day5_2(filename: &str) -> Vec<String> {
    let f = utils::readfile(filename);

    let mut crates = utils::extract_crates(f.clone());

    for line in f.lines() {
        if !line.starts_with("move") {
            continue;
        }

        let split: Vec<&str> = line.split(" ").collect();
        let move_count = split[1].parse::<usize>().unwrap();
        let from = split[3].parse::<usize>().unwrap() - 1;
        let to = split[5].parse::<usize>().unwrap() - 1;

        utils::crate_mover_9001(move_count, &mut crates, from, to);
    }

    let mut out: Vec<String> = vec![];
    for stack in crates {
        let s = stack.last();
        match s {
            Some(s) => out.push(s.clone()),
            None => (),
        }
    }

    return out;
}

fn day6_1(filename: &str) -> usize {
    let f = utils::readfile(filename)
        .strip_suffix("\n")
        .unwrap()
        .to_string();

    return utils::index_of_consec_chars(f, 4).unwrap();
}

fn day6_2(filename: &str) -> usize {
    let f = utils::readfile(filename)
        .strip_suffix("\n")
        .unwrap()
        .to_string();

    return utils::index_of_consec_chars(f, 14).unwrap();
}

fn day7_1(filename: &str) -> usize {
    let f = utils::readfile(filename);

    let filetree = utils::generate_file_tree(f);

    return filetree
        .dir_sizes()
        .iter()
        .fold(0, |acc, (_, s)| match *s <= 100000 as usize {
            true => acc + s,
            false => acc,
        });
}

fn day7_2(filename: &str) -> usize {
    let f = utils::readfile(filename);

    let filetree = utils::generate_file_tree(f);

    let unused = 70000000 - filetree.arena[0].sized(&filetree.arena);

    filetree
        .arena
        .iter()
        .filter(|n| n.sized(&filetree.arena) + unused >= 30000000)
        .fold(70000000, |acc, n| {
            let n_size = n.sized(&filetree.arena);
            match n_size < acc {
                true => n_size,
                false => acc,
            }
        })
}

#[derive(Debug)]
struct TreeSquare {
    rows: Vec<Vec<u32>>,
}

impl TreeSquare {
    fn new() -> TreeSquare {
        Self { rows: vec![] }
    }

    fn visible_right(&self, x: usize, y: usize) -> bool {
        self.rows[y][(x + 1)..=(self.rows[y].len() - 1)]
            .iter()
            .fold(0, |acc, t| match self.rows[y][x].lt(t) {
                true => acc + 1,
                false => 0,
            })
            > 0
    }

    fn visible_left(&self, x: usize, y: usize) -> bool {
        self.rows[y][0..=(x - 1)]
            .iter()
            .fold(0, |acc, t| match self.rows[y][x].lt(t) {
                true => acc + 1,
                false => 0,
            })
            > 0
    }

    fn visible_updown(&self, x: usize, y: usize) -> bool {
        self.rows
            .iter()
            .fold(0, |acc, r| match self.rows[y][x].lt(&r[x]) {
                true => acc + 1,
                false => 0,
            })
            > 0
    }

    fn count_visible(self) -> usize {
        self.rows.iter().enumerate().fold(0, |acc, (y, row)| {
            acc + row.iter().enumerate().fold(0, |acc, (x, _)| {
                match x == 0 || y == 0 || x == (row.len() - 1) || y == (self.rows.len() - 1) {
                    true => {
                        println!("{}, {}", x, y);
                        acc + 1
                    }
                    false => {
                        match self.visible_updown(x, y)
                            || self.visible_left(x, y)
                            || self.visible_right(x, y)
                        {
                            true => {
                                println!("{}, {}", x, y);
                                acc + 1
                            }
                            false => acc,
                        }
                    }
                }
            })
        })
    }
}

fn day8_1(filename: &str) -> usize {
    let f = utils::readfile(filename);

    // read the file line by line and parse into a grid-like data structure.
    // list of lists sounds right
    // make it a type so that we can put methods on it
    let mut ts: TreeSquare = TreeSquare::new();

    f.lines()
        .enumerate()
        .map(|(idx, line)| {
            line.chars()
                .map(|t| {
                    if ts.rows.len() == idx {
                        ts.rows.push(vec![]);
                    }
                    ts.rows[idx].push(t.to_digit(10).unwrap())
                })
                .count();
        })
        .count();

    println!("{:?}", ts);

    return ts.count_visible();
}
