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

    return index_of_consec_chars(f, 4).unwrap();
}

fn day6_2(filename: &str) -> usize {
    let f = utils::readfile(filename)
        .strip_suffix("\n")
        .unwrap()
        .to_string();

    return index_of_consec_chars(f, 14).unwrap();
}

fn index_of_consec_chars(str: String, count: usize) -> Option<usize> {
    let mut s: Vec<char> = vec![];

    for (i, c) in str.char_indices() {
        match s.contains(&c) {
            true => {
                match s[0] == c {
                    true => {
                        s.remove(0);
                    }
                    false => {
                        let mut iter = s.split_inclusive(|ch| ch == &c);
                        iter.next();
                        match iter.next() {
                            Some(n) => s = n.into(),
                            None => s = vec![],
                        }
                    }
                }
                s.push(c);
            }
            false => {
                s.push(c);
                if s.len() == count {
                    return Some(i + 1);
                }
            }
        }
    }

    return None;
}

#[derive(Default, Debug, Clone)]
struct ArenaTree<T>
where
    T: PartialEq + Clone,
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq + Clone,
{
    fn node(&mut self, val: T, parent: usize) -> usize {
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val, parent));
        idx
    }

    fn child(&mut self, idx: usize, child_idx: usize) {
        self.arena[idx].children.append(&mut vec![child_idx]);
    }

    fn dir_sizes(self) -> Vec<(T, usize)> {
        let mut out: Vec<(T, usize)> = vec![];
        for node in self.arena.as_slice() {
            out.push((node.val.clone(), node.sized(&self.arena)));
        }
        out
    }
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
struct Node<T>
where
    T: PartialEq + Clone,
{
    idx: usize,
    val: T,
    parent: usize,
    children: Vec<usize>,
    files: Vec<File>,
}

impl<T> Node<T>
where
    T: PartialEq + Clone,
{
    fn new(idx: usize, val: T, parent: usize) -> Self {
        Self {
            idx,
            val,
            parent,
            children: vec![],
            files: vec![],
        }
    }

    fn file(&mut self, name: String, size: usize) {
        self.files.append(&mut vec![File { name, size }]);
    }

    fn sized(&self, arena: &Vec<Node<T>>) -> usize {
        let files = self.files.clone();
        let mut ret = files.into_iter().fold(0, |acc, f| acc + f.size);

        for c in self.children.clone() {
            ret += arena[c].sized(arena);
        }

        ret
    }
}

fn day7_1(filename: &str) -> usize {
    let f = utils::readfile(filename);

    let filetree = generate_file_tree(f);

    return filetree.dir_sizes().into_iter().fold(0, |acc, (_, s)| match s <= 100000 {
        true => acc + s,
        false => acc,
    });
}

fn generate_file_tree(f: String) -> ArenaTree<String> {
    let mut filetree: ArenaTree<String> = ArenaTree::default();

    let mut curr_dir = 0;

    f.lines()
        .map(|line| match line.starts_with("$") {
            true => {
                let mut l = line.split_whitespace();
                l.next();
                match l.next().unwrap() {
                    "cd" => {
                        let dir = l.next().unwrap();
                        if dir == ".." {
                            curr_dir = filetree.arena[curr_dir].parent;
                        } else {
                            curr_dir = filetree.node(dir.to_string(), curr_dir);
                        }
                    }
                    &_ => {}
                }
            }
            false => match line.starts_with("dir") {
                true => {
                    let new_child =
                        filetree.node(line.split_whitespace().last().unwrap().to_string(), curr_dir);
                    filetree.child(curr_dir, new_child);
                }
                false => {
                    let mut fileline = line.split_whitespace();
                    let size = fileline.next().unwrap().parse::<usize>().unwrap();
                    let name = fileline.next().unwrap().to_string();
                    filetree.arena[curr_dir].file(name, size);
                }
            },
        })
        .count();

    filetree
}
