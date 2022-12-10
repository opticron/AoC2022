use std::fs;
use std::env;
use std::collections::HashMap;

fn has_dups(string: Vec<char>) -> bool {
  let mut map = HashMap::<char, u32>::new();
  for mychar in &string {
    map.insert(*mychar, 0);
  }
  map.keys().len() != string.len()
}

fn find_marker_index(line: &str) -> u32 {
  let chars: Vec<char> = line.chars().collect();
  for i in 0..chars.len() {
    if has_dups(chars[i..i+14].to_vec()) {
      continue;
    }
    return i as u32 + 14;
  }
  0
}

fn main() {
  // get file name
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[1])
        .expect("Should have been able to read the file");
  dbg!(&contents);
  let trimmed_contents = contents.strip_suffix("\n")
        .expect("Failed to strip suffix?");
  dbg!(&trimmed_contents);
  let lines: Vec<&str> = trimmed_contents.split("\n").collect();

  for line in lines {
    let marker = find_marker_index(line);
    println!("Marker at index {}", marker);
  }
}
