
pub fn run() {
  use std::fs;

  let input = fs::read_to_string("inputs/day1.txt")
      .expect("Failed to read input file");

  println!("Distance sum: {:?}", solve(&input));
}

pub fn solve(_input: &str) -> u8 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solve() {
      let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";

      assert_eq!(solve(&test_input), 11);
  }
}