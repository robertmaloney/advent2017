use super::util;

struct SpiralGrid {
  ring_index: i32,
  coordinates: (i32, i32),
  value: i32,
}

impl SpiralGrid {
  fn new() -> SpiralGrid {
    SpiralGrid {
      ring_index: 0,
      coordinates: (0, 0),
      value: 1,
    }
  }
}

impl Iterator for SpiralGrid {
  type Item = (i32, i32);

  fn next(&mut self) -> Option<<Self as Iterator>::Item> {
    let (x, y) = self.coordinates;
    let side = self.ring_index;

    if y == -side {
      if x == side {
        self.ring_index = self.ring_index + 1;
      }
      self.coordinates = (x + 1, y);
    } else if x == -side {
      self.coordinates = (x, y - 1);
    } else if y == side {
      self.coordinates = (x - 1, y);
    } else {
      self.coordinates = (x, y + 1);
    }

    Some((x, y))
  }
}

pub fn solve() {
  let input = util::string_from_file("./src/day_three/input.txt").expect("Couldn't find day_three/input.txt");
  let number = input.trim().parse::<usize>().expect("Can't parse number");
  let (x, y) = SpiralGrid::new().nth(number - 1).expect("INFINITE");
  let result = x.abs() + y.abs();
  println!("Day Three: {}", result);
}
