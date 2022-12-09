use std::fs;
use std::env;

fn get_value(play: &str) -> i32 {
  match play {
  "A"|"X" => return 1,
  "B"|"Y" => return 2,
  "C"|"Z" => return 3,
  _ => return 0
  }
}

fn get_points(entrystring: &str) -> i32 {
  let plays = entrystring.split(" ").collect::<Vec<&str>>();
  let opponent_play = get_value(plays[0]);
  let self_play = get_value(plays[1]);
  // 1 == opponent wins, 0 == tie, -1 == self wins
  // edge cases: 3 - 1 == 2 self wins, 1 - 3 == -2 oppo wins
  // +2, %3 to yield 0=oppo, 1=self, 2=tie
  let winner = (opponent_play - self_play + 2) % 3;
  match winner {
  0 => return 0+self_play,
  1 => return 6+self_play,
  2 => return 3+self_play,
  _ => return 0
  }
}

fn main() {
  println!("day1p1");
  // get file name
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[1])
        .expect("Should have been able to read the file");
  dbg!(&contents);
  let trimmed_contents = contents.strip_suffix("\n")
        .expect("Failed to strip suffix?");
  dbg!(&trimmed_contents);
  let iterentries = trimmed_contents.split("\n");
  dbg!(&iterentries);
  let roundpoints: Vec<i32> = iterentries.map(|roundstring| get_points(roundstring)).collect();
  dbg!(&roundpoints);
  let pointsum: i32 = roundpoints.into_iter().sum();
  dbg!(pointsum);
}
