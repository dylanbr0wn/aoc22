use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Elves {
  elves: Vec<i32>,
  max_len: usize,
  min_elf: usize,
}

impl Elves {
  fn new(max_len: i32) -> Elves {
    Elves { elves: vec![0; 3], max_len: max_len as usize, min_elf:0 }
  }

  fn add(&mut self, elf: i32) {
    if elf <= self.elves[self.min_elf] {
      return;
    }
    self.elves[self.min_elf] = elf;
    self.min_elf = 0;
    for i in 1..self.max_len {
      if self.elves[i] < self.elves[self.min_elf] {
        self.min_elf = i;
      }
    }
  }
  fn get_elves_total(&self) -> i32 {
    let mut total = 0;
    for i in 0..self.max_len {
      total += self.elves[i];
    }
    total
  }
}

fn main() {
  let max_len = 3;
  let mut elves = Elves::new(max_len);

  if let Ok(mut lines) = read_lines("./input.txt") {
    'outer: loop {
      let mut elf = 0;
      'inner: loop {
        match lines.next() {
          Some(Ok(line)) => {
            if line.len() == 0 {
              elves.add(elf);
              break 'inner;
            }
            if let Ok(item) = line.parse::<i32>() {
              elf += item;
            }
          },
          Some(Err(_)) => {
            println!("Error reading line");
          },
          None => {
            elves.add(elf);
            break 'outer;
          }
        }
      }
    }
  }
  let total_cals = elves.get_elves_total();

  println!("total cals is {}", total_cals);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}