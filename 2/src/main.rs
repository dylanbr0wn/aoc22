use std::{io::{self, BufRead}, path::Path, fs::File};

enum Move {
  Rock =1,
  Paper =2,
  Scissors =3,
}

enum MatchResult {
  Win = 6,
  Lose = 0,
  Draw = 3,
}

fn main() {
  let mut total = 0;
  if let Ok(lines) = read_lines("./input.txt") {
    

    for line in lines {
      if let Ok(ip) = line {

        let moves: Vec<&str> = ip.trim().split(" ").collect();

        total += get_winner(parse_opponent_move(moves[0]), parse_result(moves[1]));


      }
    }


  }
  println!("{}", total);
}

fn parse_opponent_move(o_move: &str) -> Move {
  if o_move == "A" {
    return Move::Rock;
  } else if o_move == "B" {
    return Move::Paper;
  } else if o_move == "C" {
    return Move::Scissors;
  } else {
    panic!("Invalid move");
  }
}

fn parse_my_move(o_move: &str) -> Move {
  if o_move == "X" {
    return Move::Rock;
  } else if o_move == "Y" {
    return Move::Paper;
  } else if o_move == "Z" {
    return Move::Scissors;
  } else {
    panic!("Invalid move");
  }
}

fn parse_result(result: &str) -> MatchResult {
  if result == "X" {
    return MatchResult::Lose;
  } else if result == "Y" {
    return MatchResult::Draw;
  } else if result == "Z" {
    return MatchResult::Win;
  } else {
    panic!("Invalid move");
  }
}

fn calc_result(my_move: Move, win: MatchResult) -> i32 {
  match win {
    MatchResult::Win => {
      return 6 + (my_move as i32);
    }
    MatchResult::Lose => {
      return 0 + (my_move as i32);
    }
    MatchResult::Draw => {
      return 3 + (my_move as i32);
    }
  }
}

fn get_winner(o_move: Move, result: MatchResult) -> i32 {
  match o_move {
    Move::Rock => {
      match result {
        MatchResult::Lose => {
          return calc_result(Move::Scissors, MatchResult::Lose);
        }
        MatchResult::Draw => {
          return calc_result(Move::Rock, MatchResult::Draw);
        }
        MatchResult::Win => {
          return calc_result(Move::Paper, MatchResult::Win);
        }
      }
    }
    Move::Paper => {
      match result {
        MatchResult::Lose => {
          return calc_result(Move::Rock, MatchResult::Lose);
        }
        MatchResult::Draw => {
          return calc_result(Move::Paper, MatchResult::Draw);
        }
        MatchResult::Win => {
          return calc_result(Move::Scissors, MatchResult::Win);
        }
      }
    }
    Move::Scissors => {
      match result {
        MatchResult::Lose => {
          return calc_result(Move::Paper, MatchResult::Lose);
        }
        MatchResult::Draw => {
          return calc_result(Move::Scissors, MatchResult::Draw);
        }
        MatchResult::Win => {
          return calc_result(Move::Rock, MatchResult::Win);
        }
      }
    }
  }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}