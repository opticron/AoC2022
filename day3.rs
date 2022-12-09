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

fn get_priority(entrystring: Vec<&str>) -> u32 {
  let priorities1: Vec<u32> = entrystring[0].chars().map(|entry| get_item_priority(entry)).collect();
  let priorities2: Vec<u32> = entrystring[1].chars().map(|entry| get_item_priority(entry)).collect();
  let priorities3: Vec<u32> = entrystring[2].chars().map(|entry| get_item_priority(entry)).collect();
  // trivial bit, find common item...
  for p1item in priorities1.iter() {
    for p2item in priorities2.iter() {
      if p1item != p2item {
        continue;
      }
      for p3item in priorities3.iter() {
        if p3item != p2item {
          continue;
        }
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
  let rucks = iterentries.collect::<Vec<&str>>();
  let ruckchunks = rucks.chunks(3);
  //dbg!(ruckchunks.collect::<Vec<&[&str]>>());
  let priorities = ruckchunks.map(|ruckstrings| get_priority(ruckstrings.to_vec()));
  //dbg!(priorities.collect::<Vec<u32>>());
  let prioritysum: u32 = priorities.sum();
  dbg!(prioritysum);
}
