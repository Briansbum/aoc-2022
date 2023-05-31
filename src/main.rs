use std::cmp::Ordering;
use std::str::FromStr;
mod utils;

fn main() {
    println!("day1_1: {}", day1_1("src/data/day1.txt"));
    println!("day1_2: {}", day1_2("src/data/day1.txt"));
    println!("day2_1: {}", day2_1("src/data/day2.txt"));
    println!("day2_2: {}", day2_2("src/data/day2.txt"));
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

fn play_game(l: Play, r: Play) -> usize {
    match l.cmp(&r) {
        Ordering::Equal => r as usize + 3,
        Ordering::Greater => r as usize,
        Ordering::Less => r as usize + 6,
    }
}

#[derive(Eq)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePlayError {
    error: String,
}

impl FromStr for Play {
    type Err = ParsePlayError;
    fn from_str(s: &str) -> Result<Play, Self::Err> {
        match s {
            "A" | "X" => Ok(Play::Rock),
            "B" | "Y" => Ok(Play::Paper),
            "C" | "Z" => Ok(Play::Scissors),
            _ => Err(ParsePlayError {
                error: format!("unexpected value for s: {}", s),
            }),
        }
    }
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Play::Rock, Play::Rock) => Ordering::Equal,
            (Play::Rock, Play::Paper) => Ordering::Less,
            (Play::Rock, Play::Scissors) => Ordering::Greater,
            (Play::Paper, Play::Rock) => Ordering::Greater,
            (Play::Paper, Play::Paper) => Ordering::Equal,
            (Play::Paper, Play::Scissors) => Ordering::Less,
            (Play::Scissors, Play::Rock) => Ordering::Less,
            (Play::Scissors, Play::Paper) => Ordering::Greater,
            (Play::Scissors, Play::Scissors) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
