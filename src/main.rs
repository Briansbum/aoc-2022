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
    let letter_vals: [(char, usize); 52] = [('a', 1), ('b', 2), ('c', 3), ('d', 4), ('e', 5), ('f', 6),
                                        ('g', 7), ('h', 8), ('i', 9), ('j', 10), ('k', 11), ('l', 12),
                                        ('m', 13), ('n', 14), ('o', 15), ('p', 16), ('q', 17), ('r', 18),
                                        ('s', 19), ('t', 20), ('u', 21), ('v', 22), ('w', 23), ('x', 24),
                                        ('y', 25), ('z', 26), ('A', 27), ('B', 28), ('C', 29), ('D', 30),
                                        ('E', 31), ('F', 32), ('G', 33), ('H', 34), ('I', 35), ('J', 36),
                                        ('K', 37), ('L', 38), ('M', 39), ('N', 40), ('O', 41), ('P', 42),
                                        ('Q', 43), ('R', 44), ('S', 45), ('T', 46), ('U', 47), ('V', 48),
                                        ('W', 49), ('X', 50), ('Y', 51), ('Z', 52)];
    let f = utils::readfile(filename);

    let l: Vec<usize> = f.lines().map(|line| {
        let (p1, p2) = line.split_at(line.len() / 2);
        let mut p = p1.chars().filter(|c| match p2.chars().find(|v| *c == *v) {
            Some(_) => true,
            None => false,
        }).collect::<Vec<char>>();
        
        p.sort();
        p.dedup();
        
        p.iter().fold(0, |acc, v| {
            match letter_vals.iter().find(|lv| lv.0 == *v) {
                Some(i) => acc + i.1,
                None => acc,
            }
        })
    }).collect();
    return l.iter().fold(0, |acc, x| acc + x)
}