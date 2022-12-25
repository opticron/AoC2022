use std::fs;
use std::env;
use std::collections::VecDeque;
extern crate rug;
use rug::Integer;
use crate::rug::ops::Pow;

#[derive(Debug)]
enum Operation {
  Multiply,
  Add
}

#[derive(Debug)]
struct Monkey {
  items: VecDeque<Integer>,
  operation: Operation,
  operand: i32,
  test_operand: u32,
  true_target: usize,
  false_target: usize,
  items_inspected: u32
}

impl Monkey {
  fn new(index: u32, monkey_desc: &str) -> Self {
    let monkey_desc_parts: Vec<&str> = monkey_desc.split("\n").collect();
    // extract number and verify
    let header_index = monkey_desc_parts[0].strip_suffix(":").expect("missing colon").strip_prefix("Monkey ").expect("missing header").parse::<u32>().expect("bad monkey number");
    assert!(header_index == index, "index offset");
    let mitems: VecDeque<Integer> = monkey_desc_parts[1].strip_prefix("  Starting items: ").expect("missing item header").split(", ").map(|a| a.parse::<Integer>().expect("bad item number")).collect();
    let ops_parts: Vec<&str> = monkey_desc_parts[2].strip_prefix("  Operation: new = old ").expect("missing operation header").split(" ").collect();
    let test_op = monkey_desc_parts[3].strip_prefix("  Test: divisible by ").expect("missing test header").parse::<u32>().expect("bad test number");
    let test_true = monkey_desc_parts[4].strip_prefix("    If true: throw to monkey ").expect("missing true header").parse::<usize>().expect("bad true number");
    let test_false = monkey_desc_parts[5].strip_prefix("    If false: throw to monkey ").expect("missing false header").parse::<usize>().expect("bad false number");
    Self {
      items: mitems,
      operation: match ops_parts[0] {"+"=>Operation::Add,_=>Operation::Multiply},
      operand: match ops_parts[1] {"old"=>-1,_=>ops_parts[1].parse::<u32>().expect("bad operand") as i32},
      test_operand: test_op,
      true_target: test_true,
      false_target: test_false,
      items_inspected: 0
    }
  }
}

fn main() {
  // get file name
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[1])
        .expect("Should have been able to read the file");
  let trimmed_contents = contents.strip_suffix("\n")
        .expect("Failed to strip suffix?");
  let monkeys_desc: Vec<&str> = trimmed_contents.split("\n\n").collect();
  let mut monkeys: Vec<Monkey> = Vec::new();

  for monkey_desc in monkeys_desc {
    monkeys.push(Monkey::new(monkeys.len() as u32, monkey_desc));
  }

  dbg!(&monkeys);
  let mut lcd: u32 = 1;
  for monkey in &monkeys {
    lcd *= monkey.test_operand;
    println!("test operand: {}", monkey.test_operand);
  }
  println!("lcd: {}", lcd);

  for _i in 0..10000 {
    for i in 0..monkeys.len() {
      while monkeys[i].items.len() > 0 {
        let mut worry = monkeys[i].items.pop_front().expect("guaranteed to exist");
        let absop = Integer::from(monkeys[i].operand.abs() as u32);
        if monkeys[i].operand == -1 {
          match monkeys[i].operation {
          Operation::Add => worry *= Integer::from(2 as u32),
          _ => worry = worry.pow(2)
          };
        } else {
          match monkeys[i].operation {
          Operation::Add => worry += absop,
          _ => worry *= absop
          };
        }
        worry %= Integer::from(lcd);
        let mut push_target = monkeys[i].false_target;
        if &worry % Integer::from(monkeys[i].test_operand) == Integer::from(0 as u32) {
          push_target = monkeys[i].true_target;
        }
        monkeys[push_target].items.push_back(worry);
        monkeys[i].items_inspected += 1;
      }
    }
    if _i % 1000 == 0 {
      println!("Finished round {}", _i);
    }
  }
  let mut scores: Vec<u32> = monkeys.iter().map(|a| a.items_inspected).collect();
  scores.sort();
  dbg!(&scores);
  let top: Vec<u32> = scores.into_iter().rev().take(2).collect();
  dbg!(&top);
  println!("Monkey business: {}", (top[0] as u64)*(top[1] as u64));
}
