use std::fs;
use std::env;

fn sum_elves(elfstring: &str) -> i32 {
  let mut elfsum: i32 = 0;
  for entry in elfstring.split("\n") {
    elfsum += entry.parse::<i32>().unwrap();
  }
  return elfsum;
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
  let iterelves = trimmed_contents.split("\n\n");
  dbg!(&iterelves);
  let mut elfsums: Vec<i32> = iterelves.map(|elfstring| sum_elves(elfstring)).collect();
  elfsums.sort();
  let topiter = elfsums.into_iter().rev().take(3);
  let topsum: i32 = topiter.sum();
  dbg!(topsum);
}
