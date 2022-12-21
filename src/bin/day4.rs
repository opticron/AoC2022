use std::fs;
use std::env;

fn get_range(range: &str) -> Vec<u32> {
  return range.split("-").map(|rpoint| rpoint.parse::<u32>().expect("Bad conversion")).collect::<Vec<u32>>();
}

fn target_in_range(target: u32, range: &Vec<u32>) -> bool {
  if target >= range[0] && target <= range[1] {
    return true;
  }
  return false;
}

fn get_contained(entrystring: &str) -> u32 {
  let entries: Vec<&str> = entrystring.split(",").collect();
  let entry1r: Vec<u32> = get_range(entries[0]);
  let entry2r: Vec<u32> = get_range(entries[1]);

  if target_in_range(entry2r[0], &entry1r) {
    return 1;
  }
  if target_in_range(entry2r[1], &entry1r) {
    return 1;
  }
  if target_in_range(entry1r[0], &entry2r) {
    return 1;
  }
  if target_in_range(entry1r[1], &entry2r) {
    return 1;
  }
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
  let fully_contained = iterentries.map(|pair| get_contained(pair));
  let prioritysum: u32 = fully_contained.sum();
  dbg!(prioritysum);
}
