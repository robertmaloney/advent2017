use super::util;

fn parse_digits(input: &String) -> Vec<u32> {
  input.chars().filter_map(|c| c.to_digit(10)).collect()
}

pub fn solve() -> u32 {
  let input = util::string_from_file("./src/day_one/input.txt").expect("Couldn't find input.txt");
  let digits = parse_digits(&input);
  let skip_val = digits.len() / 2;
  let pairs = digits.iter().zip(digits.iter().cycle().skip(skip_val));
  pairs.fold(
    0,
    |acc, (d0, d1)| {
      if d0 == d1 {
        acc + d0
      } else {
        acc
      }
    }
  )
}
