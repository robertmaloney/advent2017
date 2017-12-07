use super::util;

struct Jumper {
  state: Vec<i32>,
  index: i32,
}

impl Jumper {
  fn new(initial_state: Vec<i32>) -> Jumper {
    Jumper {
      state: initial_state,
      index: 0,
    }
  }
}

impl Iterator for Jumper {
  type Item = ();

  fn next(&mut self) -> Option<<Self as Iterator>::Item> {
    if self.index < 0 || self.index > self.state.len() as i32 - 1 {
      return None;
    }

    let jump_by = self.state[self.index as usize];
    self.state[self.index as usize] += if jump_by > 2 { -1 } else { 1 };
    self.index += jump_by;
    Some(())
  }
}

pub fn solve() {
  let input = util::string_from_file("./src/day_five/input.txt").expect("Couldn't find day_five/input.txt");

  let jumps = Jumper::new(
    input
      .lines()
      .filter_map(|line| i32::from_str_radix(line, 10).ok())
      .collect()
  ).count();

  println!("Day Five: {}", jumps);
}
