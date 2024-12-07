pub fn run(input: String) {
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

pub fn solve_part1(_input: &str) -> u32 {
    0
}

pub fn solve_part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 0);
    }
}
