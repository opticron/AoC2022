use std::fs;
use std::env;

fn main() {
  // get file name
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[1])
        .expect("Should have been able to read the file");
  let trimmed_contents = contents.strip_suffix("\n")
        .expect("Failed to strip suffix?");
  let treelists: Vec<&str> = trimmed_contents.split("\n").collect();

  let mut tree_map: Vec<Vec<i16>> = Vec::new();
  let mut seen_trees: Vec<Vec<bool>> = Vec::new();

  for treelist in treelists {
    let tree_line: Vec<i16> = treelist.chars().map(|entry| entry as i16 - '0' as i16).collect();
    let mut tree_line_seen: Vec<bool> = Vec::new();
    for _i in 0..tree_line.len() {
      tree_line_seen.push(false);
    }
    let mut highest_seen: i16 = -1;
    let mut highest_seen_rev: i16 = -1;
    for i in 0..tree_line.len() {
      let i_rev = &tree_line.len() - i - 1;
      let current = tree_line[i];
      let current_rev = tree_line[i_rev];
      if current > highest_seen {
        highest_seen = current;
        tree_line_seen[i] = true;
      }
      if current_rev > highest_seen_rev {
        highest_seen_rev = current_rev;
        tree_line_seen[i_rev] = true;
      }
    }
    tree_map.push(tree_line);
    seen_trees.push(tree_line_seen);
  }

  for j in 0..tree_map[0].len() {
    let mut highest_seen: i16 = -1;
    let mut highest_seen_rev: i16 = -1;
    for i in 0..tree_map.len() {
      let i_rev = tree_map.len() - i - 1;
      let current = tree_map[i][j];
      let current_rev = tree_map[i_rev][j];
      if current > highest_seen {
        highest_seen = current;
        seen_trees[i][j] = true;
      }
      if current_rev > highest_seen_rev {
        highest_seen_rev = current_rev;
        seen_trees[i_rev][j] = true;
      }
    }
  }

  let mut total_seen: i16 = 0;
  for a in seen_trees {
    for b in a {
      if b {
        total_seen += 1;
      }
    }
  }
  println!("Total viewable trees: {}", total_seen);
}
