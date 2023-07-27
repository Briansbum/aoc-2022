use core::panic;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub fn extract_crates(f: String) -> Vec<Vec<String>> {
    let mut crates: Vec<Vec<String>> = vec!{};
    for line in f.lines() {
        if line.starts_with(" 1") {
            break
        }
        line.char_indices().map(|(i, c)| {
            if c.is_alphabetic() {
                while crates.len() <= i/4 {
                    crates.push(vec!{});
                }
                crates[i/4].push(c.to_string());
            }
        }).last();
    }

    crates.iter_mut().map(|cs| {
        cs.reverse()
    }).last();
    
    return crates;
}

pub fn crate_mover_9001(count: usize, crates: &mut Vec<Vec<String>>, from: usize, to:usize) {
    let mut i = 0;
    let mut pickup: Vec<String> = vec!{};
    while i < count {
        let c = crates[from].pop();
        match c {
            Some(c) => pickup.push(c),
            None => panic!("uh oh: {:?}", crates[from]),
        }

        i += 1;
    }
    pickup.reverse();
    crates[to].append(&mut pickup);
}

pub fn crate_mover_9000(count: usize, crates: &mut Vec<Vec<String>>, from: usize, to:usize) {
    let mut i = 0;
    while i < count {
        let c = crates[from].pop();
        match c {
            Some(c) => crates[to].push(c),
            None => panic!("uh oh: {:?}", crates[from]),
        }

        i += 1;
    }
}


pub fn elf_snack_cals(filename: &str) -> Vec<usize> {
    let f = readfile(filename);
    let mut ret: Vec<usize> = vec![];
    let mut cur: usize = 0;
    f.lines().for_each(|l| match l {
        "" => {
            ret.push(cur);
            cur = 0;
        }
        _ => {
            cur = cur
                + match l.parse::<usize>() {
                    Ok(i) => i,
                    Err(error) => {
                        panic!("unable to parse string {:?} as usize: {:?}", l, error);
                    }
                }
        }
    });
    ret.push(cur);
    return ret;
}

#[cfg(test)]
mod tests {
    use super::elf_snack_cals;

    #[test]
    fn test_elf_snack_cals() {
        let c = elf_snack_cals("src/fixtures/day1.txt");
        assert_eq!(c, vec![6000, 4000, 11000, 24000, 10000]);
    }
}

pub fn readfile(s: &str) -> String {
    let file_result = File::open(s);

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("problem opening file: {:?}", error),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(error) => panic!("cannot read file into string: {:?}", error),
        _ => (),
    };

    return contents;
}

pub fn play_game(l: Play, r: Play) -> usize {
    match l.cmp(&r) {
        Ordering::Equal => r as usize + 3,
        Ordering::Greater => r as usize,
        Ordering::Less => r as usize + 6,
    }
}

#[derive(Eq)]
pub enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePlayError {
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

pub const LETTER_VALS: [(char, usize); 52] = [
    ('a', 1),
    ('b', 2),
    ('c', 3),
    ('d', 4),
    ('e', 5),
    ('f', 6),
    ('g', 7),
    ('h', 8),
    ('i', 9),
    ('j', 10),
    ('k', 11),
    ('l', 12),
    ('m', 13),
    ('n', 14),
    ('o', 15),
    ('p', 16),
    ('q', 17),
    ('r', 18),
    ('s', 19),
    ('t', 20),
    ('u', 21),
    ('v', 22),
    ('w', 23),
    ('x', 24),
    ('y', 25),
    ('z', 26),
    ('A', 27),
    ('B', 28),
    ('C', 29),
    ('D', 30),
    ('E', 31),
    ('F', 32),
    ('G', 33),
    ('H', 34),
    ('I', 35),
    ('J', 36),
    ('K', 37),
    ('L', 38),
    ('M', 39),
    ('N', 40),
    ('O', 41),
    ('P', 42),
    ('Q', 43),
    ('R', 44),
    ('S', 45),
    ('T', 46),
    ('U', 47),
    ('V', 48),
    ('W', 49),
    ('X', 50),
    ('Y', 51),
    ('Z', 52),
];
