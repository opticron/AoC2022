use std::fs;
use std::env;

fn find_marker_index(line: &str) -> u32 {
  let chars: Vec<char> = line.chars().collect();
  for i in 0..chars.len() {
    if chars[i] == chars[i+1] {
      continue;
    }
    if chars[i] == chars[i+2] {
      continue;
    }
    if chars[i] == chars[i+3] {
      continue;
    }
    if chars[i+1] == chars[i+2] {
      continue;
    }
    if chars[i+1] == chars[i+3] {
      continue;
    }
    if chars[i+2] == chars[i+3] {
      continue;
    }
    return i as u32 + 4;
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
