use std::{io::{self, BufRead}, fs::File, path::Path, ops::Index, iter::Map, collections::HashMap};


struct Backpack {
  itemMap: HashMap<String, usize>
}

fn main() {
  let mut total = 0;
  if let Ok(lines) = read_lines("./input.txt") {
    let mut group = 0;
    let mut group_map: HashMap<String,i32> = HashMap::new();
    for (j, line) in lines.enumerate() {
      
      
      if let Ok(ip) = line {
        let len = ip.len();
        let mut backpack = Backpack {
          itemMap: HashMap::new(),
        };
        // let mut dupe:String = "".to_string();
        for i in 0..len {
          
          let ch = ip.chars().nth(i).unwrap().to_string();

          match backpack.itemMap.entry(ch.clone()) {
            std::collections::hash_map::Entry::Occupied(mut o) => {
              *o.get_mut() += 1;
            },
            std::collections::hash_map::Entry::Vacant(v) => {
              v.insert(1);
              group_map.entry(ch.clone()).and_modify(|counter| {
                *counter += 1
              }).or_insert(1);
            }
          }
        }

        group += 1;

        if group > 2 {
          group_map.iter().for_each(|(k,v)| {
            // println!("{} {} {}", k, v, j);
            if *v == 3 {
              let val = get_char_val(k.to_string());
              println!("{} {} {}", k, val, j);
              total += val;
            }
          });
          group = 0;
          group_map.clear();
        }
        
        
      }
    }


  }
  println!("{}", total);
}

fn get_char_val(ch: String) -> i32 {
  let mut b = ch.as_bytes()[0];

  if b > 64 && b < 91 {
    b -= 38;
  } else {
    b -= 96;
  }
  return b as i32;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
