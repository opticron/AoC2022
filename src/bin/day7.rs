use std::fs;
use std::env;
use std::collections::HashMap;
use std::collections::VecDeque;

struct DirEnt<'a> {
  files: HashMap<&'a str, u64>,
  dirs: HashMap<&'a str, DirEnt<'a>>
}

impl<'a> DirEnt<'a> {
  fn new() -> DirEnt<'a> {
    Self {
      files: HashMap::<&'a str, u64>::new(),
      dirs: HashMap::<&'a str, DirEnt<'a>>::new()
    }
  }
}

fn main() {
  // get file name
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[1])
        .expect("Should have been able to read the file");
  dbg!(&contents);
  let trimmed_contents = contents.strip_suffix("\n")
        .expect("Failed to strip suffix?").strip_prefix("$ ").expect("Failed to strip prefix");
  dbg!(&trimmed_contents);
  let commands: Vec<&str> = trimmed_contents.split("\n$ ").collect();

  let mut pwd: Vec<&str> = Vec::<&str>::new();
  let mut root: DirEnt = DirEnt::new();
  
  for total_command in commands {
    let mut lines: VecDeque<&str> = total_command.split("\n").collect();
    let command = lines.pop_front().expect("must have at least one line per command");
    if command.starts_with("cd ") {
      println!("command: {}", command);
      println!("pwd state: {:?}", pwd);
      // deal with cd
      let cdtarget = command.strip_prefix("cd ").expect("verified cd had bad prefix");
      if cdtarget == ".." {
        pwd.pop();
      println!("post pwd state: {:?}", pwd);
        continue;
      }
      if cdtarget == "/" {
        pwd.clear();
      println!("post pwd state: {:?}", pwd);
        continue;
      }
      // deal with subdir
      pwd.push(cdtarget);
      println!("post pwd state: {:?}", pwd);
      continue;
    }
    // deal with ls output
    let mut target_dirent = &mut root;
    for dir in &pwd {
      if !target_dirent.dirs.contains_key(dir) {
        // create dir
        target_dirent.dirs.insert(dir, DirEnt::new());
      }
      target_dirent = target_dirent.dirs.get_mut(dir).expect("target dir already verified");
    }
    // target dirent is where file data goes
    for line in lines {
      if line.starts_with("dir ") {
        // ignore dirs in ls output
        continue;
      }
      let lineparts: Vec<&str> = line.split(" ").collect();
      target_dirent.files.insert(lineparts[1], lineparts[0].parse::<u64>().expect("bad conversion"));
    }
  }

  // scan the structure recursively to find dirs smaller than 100000
  let mut all_totals: Vec<u64> = Vec::new();
  let all_used = find_sizes(&mut all_totals, &root, &"");
  dbg!(&all_totals);
  let current_free = 70000000 - all_used;
  let target_free = 30000000;
  let deficit_space = target_free - current_free;
  let smallest_deletion = all_totals.iter().filter(|x| x > &&deficit_space).min().expect("no elements?");
  println!("smallest deletion: {}", smallest_deletion);
}

fn find_sizes<'a>(all_totals: &mut Vec<u64>, dirent: &DirEnt<'a>, _curdir: &'a str) -> u64 {
  let mut total: u64 = 0;
  for size in dirent.files.values() {
    total += size;
  }
  for (name, subdir) in &dirent.dirs {
    total += find_sizes(all_totals, &subdir, &name);
  }
  all_totals.push(total);
  total
}
