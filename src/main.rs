use std::fs::File;
use std::io::Read;

fn main() {
    println!("{}", day1());
}

fn day1() -> usize {
    let mut b: usize = 0;
    let mut cur: usize = 0;
    let mut last_newline: bool = false;
    let mut e = String::from("");
    for c in readfile("src/day1.txt").chars() {
        match c {
            '\n' => {
                if e != "" {
                    cur = cur + match e.parse::<usize>() {
                        Ok(i) => i,
                        Err(error) => {
                            panic!("unable to parse string {:?} to int {:?}", e, error)
                        }
                    };
                    e = "".to_string();
                    continue
                }
                if !last_newline {
                    if cur > b {
                        b = cur;
                    }
                    cur = 0;
                    last_newline = false;
                }
            }
            _ => e.push(c),
        }
    }
    return b;
}

fn readfile(s: &str) -> String {
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
