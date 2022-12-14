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

  let mut tree_map: Vec<Vec<u32>> = Vec::new();
  let mut view_score: Vec<Vec<u32>> = Vec::new();

  for treelist in treelists {
    let tree_line: Vec<u32> = treelist.chars().map(|entry| entry as u32 - '0' as u32).collect();
    let mut view_score_line: Vec<u32> = Vec::new();
    for _i in 0..tree_line.len() {
      view_score_line.push(0);
    }
    tree_map.push(tree_line);
    view_score.push(view_score_line);
  }

  let mut max_view_score: u32 = 0;
  for i in 0..tree_map.len() {
    if i == 0 || i == tree_map.len() - 1 {
      continue;
    }
    for j in 0..tree_map[0].len() {
      if j == 0 || j == tree_map[0].len() - 1 {
        continue;
      }
      view_score[i][j] = find_view_score(&tree_map, i, j);
      if view_score[i][j] > max_view_score {
        max_view_score = view_score[i][j];
      }
    }
  }
  println!("Max viewable: {}", max_view_score);
}

fn find_view_score(tree_map: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
  let height: u32 = tree_map[i][j];
  let mut udist: usize = 0;
  let mut ddist: usize = 0;
  let mut ldist: usize = 0;
  let mut rdist: usize = 0;
  let mut utrack: bool = true;
  let mut dtrack: bool = true;
  let mut ltrack: bool = true;
  let mut rtrack: bool = true;
  let mut offset: usize = 0;
  loop {
    offset += 1;
    let mut still_checking: bool = false;
    let ucheck: i64 = i as i64 - offset as i64;
    if ucheck >= 0 && utrack {
      // check up
      still_checking = true;
      let target_height = tree_map[ucheck as usize][j];
      udist = offset;
      if target_height >= height {
        utrack = false;
      }
    }
    let dcheck: i64 = i as i64 + offset as i64;
    if dcheck < tree_map.len() as i64 && dtrack {
      // check down
      still_checking = true;
      let target_height = tree_map[dcheck as usize][j];
      ddist = offset;
      if target_height >= height {
        dtrack = false;
      }
    }
    let lcheck: i64 = j as i64 - offset as i64;
    if lcheck >= 0 && ltrack {
      // check left
      still_checking = true;
      let target_height = tree_map[i][lcheck as usize];
      ldist = offset;
      if target_height >= height {
        ltrack = false;
      }
    }
    let rcheck: i64 = j as i64 + offset as i64;
    if rcheck < tree_map[0].len() as i64 && rtrack {
      // check right
      still_checking = true;
      let target_height = tree_map[i][rcheck as usize];
      rdist = offset;
      if target_height >= height {
        rtrack = false;
      }
    }

    if !still_checking {
      break;
    }
  }

  // distances calculated, get view score
  let score = udist * ddist * ldist * rdist;
  score as u32
}
