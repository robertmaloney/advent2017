use super::util;
use std::collections::HashSet;
use std::iter::FromIterator;

fn sort_phrase(input: &str) -> String {
  let mut chars: Vec<char> = input.chars().collect();
  chars.sort();
  String::from_iter(chars.iter())
}

pub fn solve() {
  let input = util::string_from_file("./src/day_four/input.txt").expect("Couldn't find day_three/input.txt");

  let result: u32 = input.lines().map(|line| {
    let dup = line
      .split_whitespace()
      .map(|phrase| sort_phrase(&phrase))
      .fold(
        (HashSet::new(), false),
        |(mut s, dup), phrase| {
          if dup {
            (s, dup)
          } else if s.contains(&phrase) {
            (s, true)
          } else {
            s.insert(phrase);
            (s, false)
          }
        }
      ).1;
    if dup { 0 } else { 1 }
  }).sum();

  println!("Day Four: {}", result);
}
