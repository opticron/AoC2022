use std::fs;
use std::env;

fn main() {
  // get file name
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[1])
        .expect("Should have been able to read the file");
  let trimmed_contents = contents.strip_suffix("\n")
        .expect("Failed to strip suffix?");
  let instructions: Vec<&str> = trimmed_contents.split("\n").collect();

  let mut x: i128 = 1;
  let mut screen: Vec<char> = Vec::new();
  let mut cycle: u64 = 0;
  for ins in instructions {
    if ins == "noop" {
      bump_cycle(&mut cycle, &x, &mut screen);
      continue;
    }
    // only other instruction is addx
    bump_cycle(&mut cycle, &x, &mut screen);
    let modif: i128 = ins.strip_prefix("addx ").expect("missing prefix").parse::<i128>().expect("bad input");
    bump_cycle(&mut cycle, &x, &mut screen);
    x += modif;
  }

  for i in 0..6 {
    for j in 0..40 {
      print!("{}", screen[i*40+j]);
    }
    println!("");
  }
}

fn bump_cycle(cycle: &mut u64, x: &i128, screen: &mut Vec<char>) {
  let diff = x - (*cycle as i128 % 40);
  if diff.abs() < 2 {
    screen.push('#');
  } else {
    screen.push('.');
  }
  *cycle += 1;
}
