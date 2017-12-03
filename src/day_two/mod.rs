use super::util;
use std::cmp;
use std::u32;

fn max_min_difference<T: Iterator<Item=u32>>(iter: T) -> u32 {
  let values = iter.fold(
    (u32::min_value(), u32::max_value()),
    |(max, min), elem| (cmp::max(max, elem), cmp::min(min, elem))
  );

  values.0 - values.1
}

fn evenly_divisible_division<T: Iterator<Item=u32> + Clone>(iter: T) -> Option<u32> {
  for (x_i, x) in iter.clone().enumerate() {
    for (y_i, y) in iter.clone().enumerate() {
      if y % x == 0 && y_i != x_i {
        return Some(y / x);
      }
    }
  }
  None
}

pub fn solve() {
  let input = util::string_from_file("./src/day_two/input.txt").expect("Couldn't find day_two/input.txt");
  let checksum: u32 = input.lines().filter_map(|line| {
    let parse = |num: &str| num.parse::<u32>().ok();
    let line_numbers = line
      .split_whitespace()
      .filter_map(&parse);

    evenly_divisible_division(line_numbers)
  }).sum();

  println!("Day Two: {}", checksum);
}
