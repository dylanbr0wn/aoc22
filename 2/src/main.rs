use std::{io::{self, BufRead}, path::Path, fs::File};

fn main() {
  let mut total = 0;
  if let Ok(lines) = read_lines("./input.txt") {
    for line in lines {
      if let Ok(ip) = line {
        let moves: Vec<&str> = ip.trim().split(" ").collect();
        total += get_winner_from_moves(parse_move("ABC", moves[0]), parse_move("XYZ", moves[1]));
        // total += get_winner_from_result(parse_move("ABC", moves[0]), parse_result(moves[1]));
      }
    }
  }
  println!("{}", total);
}

fn parse_move(lookup: &str, o_move: &str) -> usize {
  let res = lookup.match_indices(o_move).next().unwrap().0 + 1;
  return res;
}

fn parse_result(result: &str) -> usize {
  let items = "XYZ";
  let res = items.match_indices(result).next().unwrap().0 * 3;
  return res;
}
fn get_winner_from_result(o_move: usize, result: usize ) -> usize {
  let mut score = result;

  if score == 3 {
    score += o_move
  } else if result == 0 {
    let mut res = o_move - 1;
    if res < 1 {
      res +=3;
    }
    score += res;
  } else {
    let mut res = o_move + 1;
    if res > 3 {
      res -=3;
    }
    score += res;
  }

  return score
}

fn get_winner_from_moves(o_move: usize, m_move: usize) -> usize {
  let mut score: usize = m_move ;

  if o_move == m_move {
    score += 3;
  }else {
    let mut res: usize = 0;
    if m_move < o_move {
      res = (3 + m_move - o_move) % 3;
    }else {
      res = (m_move - o_move) % 3;
    }
    if res == 1 {
      score += 6;
    }
  }
  return score;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}