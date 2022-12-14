use std::fs;
use std::env;
use std::collections::HashMap;

fn main() {
  // get file name
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[1])
        .expect("Should have been able to read the file");
  let trimmed_contents = contents.strip_suffix("\n")
        .expect("Failed to strip suffix?");
  let moves: Vec<&str> = trimmed_contents.split("\n").collect();

  let mut used_coords: HashMap<Vec<i32>, bool> = HashMap::new();
  let mut head: Vec<i32> = Vec::new();
  let mut tail: Vec<i32> = Vec::new();

  head.push(0);
  head.push(0);
  tail.push(0);
  tail.push(0);

  used_coords.insert(tail.clone(), true);

  for mov in moves {
    let mov_parts: Vec<&str> = mov.split(" ").collect();
    let i_dir: i32 = match mov_parts[0] {
    "U" => 1,
    "D" => -1,
    _ => 0,
    };
    let j_dir: i32 = match mov_parts[0] {
    "R" => 1,
    "L" => -1,
    _ => 0,
    };
    for _i in 0..(mov_parts[1].parse::<u32>().expect("parse blew")) {
      // move head
      head[0] += i_dir;
      head[1] += j_dir;
      // does tail need to move?
      if (head[0] - tail[0]).abs() > 1 {
        tail[1] = head[1];
        tail[0] = head[0] - i_dir;
      }
      if (head[1] - tail[1]).abs() > 1 {
        tail[0] = head[0];
        tail[1] = head[1] - j_dir;
      }
      used_coords.insert(tail.clone(), true);
    }
  }
  println!("spaces occupied: {}", used_coords.keys().len());
}
