use std::fs;
use std::env;

fn get_item_priority(item: char) -> u32 {
  if item >= 'A' && item <= 'Z' {
    return item as u32 - 'A' as u32 + 27;
  }
  if item >= 'a' && item <= 'z' {
    return item as u32 - 'a' as u32 + 1;
  }
  return 0;
}

fn get_priority(entrystring: &str) -> u32 {
  let priorities: Vec<u32> = entrystring.chars().map(|entry| get_item_priority(entry)).collect();
  let partition1 = &priorities[0..priorities.len()/2];
  let partition2 = &priorities[priorities.len()/2..priorities.len()];
  // trivial bit, find common item...
  for p1item in partition1.iter() {
    for p2item in partition2.iter() {
      if p1item == p2item {
        println!("Found shared priority {}", *p1item);
        return *p1item;
      }
    }
  }
  println!("Found no shared priorities");
  return 0;
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
  let iterentries = trimmed_contents.split("\n");
  dbg!(&iterentries);
  let priorities = iterentries.map(|ruckstring| get_priority(ruckstring));
  dbg!(&priorities);
  let prioritysum: u32 = priorities.sum();
  dbg!(prioritysum);
}
