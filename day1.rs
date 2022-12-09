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
  let elfsums = iterelves.map(|elfstring| sum_elves(elfstring));
  let max: i32 = elfsums.fold(std::i32::MIN, |a,b| a.max(b));
  dbg!(max);
}
