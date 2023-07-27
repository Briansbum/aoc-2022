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

    let ff = f.lines().map(|line| {
        line.chars()
            .map(|c| match utils::LETTER_VALS.iter().find(|lv| lv.0 == c) {
                Some(f) => f.1,
                None => 0,
            }).collect::<Vec<usize>>()
    }).collect::<Vec<Vec<usize>>>();

    let mut counter = 0;
    let mut result = 0;
    return loop {
        if counter >= ff.len()-1 {
            break result
        }
        for s in &ff[counter] {
            if ff[counter+1].contains(s) && ff[counter+2].contains(s) {
                result += s;
                break
            }
        }
        counter += 3;
    };
}

fn day4_1(filename: &str) -> usize {
    let f = utils::readfile(filename);

    let mut ret: usize = 0;
    f.lines().filter(|line| {
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
    }).map(|_| {
        ret += 1;
    }).last();

    return ret;
}

fn day4_2(filename: &str) -> usize {
    let f = utils::readfile(filename);

    let mut ret: usize = 0;
    f.lines().filter(|line| {
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
            return true
        }
        if elf2_start < elf1_start && elf2_end >= elf1_start && elf2_end < elf1_end {
            return true
        }

        if elf1_end > elf2_end && elf1_start <= elf2_end && elf1_start > elf2_start {
            return true
        }
        if elf2_end > elf1_end && elf2_start <= elf1_end && elf2_start > elf1_start {
            return true
        }

        return false;
    }).map(|_| {
        ret += 1;
    }).last();

    return ret;
}

