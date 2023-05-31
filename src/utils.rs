use core::panic;
use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;
use std::str::FromStr;

pub fn elf_snack_cals(filename: &str) -> Vec<usize> {
  let f = readfile(filename);
  let mut ret: Vec<usize> = vec![];
  let mut cur: usize = 0;
  f.lines().for_each(|l| match l {
    "" => {
      ret.push(cur);
      cur = 0;
    },
    _ => {
      cur = cur + match l.parse::<usize>() {
          Ok(i) => i,
          Err(error) => {
            panic!("unable to parse string {:?} as usize: {:?}", l, error);
          }
      }
    }
  });
  ret.push(cur);
  return ret
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