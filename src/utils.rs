use core::panic;
use std::fs::File;
use std::io::Read;

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