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
  let mut signal_strength_acc: i128 = 0;
  let mut cycle: u64 = 0;
  for ins in instructions {
    if ins == "noop" {
      bump_cycle(&mut cycle, &x, &mut signal_strength_acc);
      continue;
    }
    // only other instruction is addx
    bump_cycle(&mut cycle, &x, &mut signal_strength_acc);
    let modif: i128 = ins.strip_prefix("addx ").expect("missing prefix").parse::<i128>().expect("bad input");
    bump_cycle(&mut cycle, &x, &mut signal_strength_acc);
    x += modif;
  }
  println!("final signal strength accum: {}", signal_strength_acc);
}

fn bump_cycle(cycle: &mut u64, x: &i128, signal_strength_acc: &mut i128) {
  *cycle += 1;
  if (*cycle as i128 - 20) % 40 == 0 {
    let sig_strength: i128 = x*((*cycle) as i128);
    println!("sig strength at cycle {}: {}", *cycle, sig_strength);
    *signal_strength_acc += sig_strength;
  }
}
