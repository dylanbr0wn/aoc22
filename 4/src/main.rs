
use std::{io::{self, BufRead}, fs::File, path::Path};

struct Elf {
  max: usize,
  min: usize,
}


fn main() {
  let mut total = 0;
  if let Ok(lines) = read_lines("./input.txt") {

    for line in lines{
      if let Ok(ip) = line {
        let groups = ip.split(",").map(|x| {
          let group = x.split("-").collect::<Vec<&str>>();
          let min = group[0].parse::<usize>().unwrap();
          let max = group[1].parse::<usize>().unwrap();
          Elf { min, max }
        }).collect::<Vec<Elf>>();

        let elf1 = groups.get(0).unwrap();

        let elf2 = groups.get(1).unwrap();
        if elf1.min <= elf2.min && elf1.max >= elf2.min {
          total += 1;
        } else if elf2.min <= elf1.min && elf2.max >= elf1.min {
          total +=1
        }
      }
    }
  }
  println!("total: {}", total);
    
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
