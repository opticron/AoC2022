use std::fs;
use std::env;

fn perform_state_import(state: &mut Vec<Vec<char>>, stateline: &str) {
  let chars: Vec<char> = stateline.chars().collect();
  let charchunks = chars.chunks(4);
  dbg!(&charchunks);
  for (i, stack) in charchunks.enumerate() {
    let item = stack[1];
    if item != ' ' {
      // update state
      state[i].push(item);
    }
  }
}

fn get_alteration_details(stateline: &str) -> (u32, usize, usize) {
  let lineparts: Vec<&str> = stateline.split(" ").collect();
  (
    lineparts[1].parse::<u32>().expect("Bad count"),
    lineparts[3].parse::<usize>().expect("Bad source") - 1,
    lineparts[5].parse::<usize>().expect("Bad destination") - 1
  )
}

fn perform_state_alteration(state: &mut Vec<Vec<char>>, stateline: &str) {
  let (count, source, dest) = get_alteration_details(stateline);
  let slen = state[source].len();
  let sslice = slen - count as usize;
  let mut items_moved: Vec<char> = (&state[source][sslice..slen]).to_vec();
  state[source].truncate(sslice);
  state[dest].append(&mut items_moved);
  dbg!(state);
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
  let parts: Vec<&str> = trimmed_contents.split("\n\n").collect();
  let statestr = parts[0];
  let instructionstr = parts[1];
  let instructions = instructionstr.split("\n");

  // pull numbering off of the bottom of state string to get width
  let stateparts: Vec<&str> = statestr.split("\n ").collect();
  let actualstate = stateparts[0];
  dbg!(&stateparts[1]);
  let columncountstr: Vec<&str> = stateparts[1].strip_suffix(" ")
        .expect("Failed to strip suffix again?").split("   ").collect();
  let stackcount = columncountstr.len();
  dbg!(stackcount);
  let mut state: Vec<Vec<char>> = Vec::new();
  for _i in 0..stackcount {
    state.push(Vec::new());
  }
  let rawstateparse: Vec<&str> = actualstate.split("\n").collect();
  let stateparse = rawstateparse.iter().rev();
  for stateline in stateparse {
    perform_state_import(&mut state, stateline);
  }
  
  for instruction in instructions {
    perform_state_alteration(&mut state, instruction);
  }

  for stack in state {
    print!("{}", stack.last().expect("No elements left?"));
  }
  println!("");
}
