use std::fs;
use std::env;
use std::collections::HashMap;

fn main() {
  // get file name
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[1])
        .expect("Should have been able to read the file");
  let trimmed_contents = contents.strip_suffix("\n")
        .expect("Failed to strip suffix?");
  let moves: Vec<&str> = trimmed_contents.split("\n").collect();

  let mut used_coords: HashMap<Vec<i32>, bool> = HashMap::new();
  let mut knots: Vec<Vec<i32>> = Vec::new();
  for i in 0..10 {
    knots.push(Vec::new());
    knots[i].push(0);
    knots[i].push(0);
  }

  used_coords.insert(knots[9].clone(), true);

  for mov in moves {
    let mov_parts: Vec<&str> = mov.split(" ").collect();
    let i_dir: i32 = match mov_parts[0] {
    "U" => 1,
    "D" => -1,
    _ => 0,
    };
    let j_dir: i32 = match mov_parts[0] {
    "R" => 1,
    "L" => -1,
    _ => 0,
    };
    // for each move request
    println!("Executing move: {}", mov);
    for _i in 0..(mov_parts[1].parse::<u32>().expect("parse blew")) {
      // move head
      knots[0][0] += i_dir;
      knots[0][1] += j_dir;
      println!("Moved head to {:?}", knots[0]);
      // check remaining knots
      for i in 0..(knots.len()-1) {
        // does knot need to move?
        println!("evaluating knot {:?} vs follower {:?}", &knots[i], &knots[i+1]);
        let x_too_far = (knots[i][0] - knots[i+1][0]).abs() > 1;
        let y_too_far = (knots[i][1] - knots[i+1][1]).abs() > 1;
        if x_too_far && y_too_far {
          println!("X and Y dist too far");
          let xoffset = knots[i][0] - knots[i+1][0];
          let xincrement = xoffset/xoffset.abs();
          knots[i+1][0] = knots[i][0] - xincrement;

          let yoffset = knots[i][1] - knots[i+1][1];
          let yincrement = yoffset/yoffset.abs();
          knots[i+1][1] = knots[i][1] - yincrement;
        } else if x_too_far {
          println!("X dist too far");
          let offset = knots[i][0] - knots[i+1][0];
          let increment = offset/offset.abs();
          knots[i+1][0] = knots[i][0] - increment;
          knots[i+1][1] = knots[i][1];
        } else if y_too_far {
          println!("Y dist too far");
          let offset = knots[i][1] - knots[i+1][1];
          let increment = offset/offset.abs();
          knots[i+1][0] = knots[i][0];
          knots[i+1][1] = knots[i][1] - increment;
        } else {
          // if nothing moved, ignore the rest of knots for this move
          println!("close enough, no further changes necessary");
          break;
        }
      }
      used_coords.insert(knots[9].clone(), true);
    }
    dbg!(&knots);
  }
  println!("spaces occupied: {}", used_coords.keys().len());
}
