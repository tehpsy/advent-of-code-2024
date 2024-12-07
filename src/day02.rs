type Level = i32;
type Report = Vec<Level>;
type Reports = Vec<Report>;

pub fn run(input: String) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

pub fn parse_reports(input: &str) -> Reports {
    input.lines().map(|line| parse_report(line)).collect()
}

pub fn parse_report(input: &str) -> Report {
    input
        .split_whitespace()
        .map(|text| text.parse::<Level>().expect("Invalid number"))
        .collect()
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Increasing,
    Decreasing,
}

pub fn is_safe(report: &Report) -> bool {
    if report.is_empty() {
        return true;
    }

    let mut direction: Option<Direction> = None;

    for (index, level) in report.iter().enumerate().skip(1) {
        let previous_level = report[index - 1];
        if direction == None && *level != previous_level {
            if *level > previous_level {
                direction = Some(Direction::Increasing);
            } else {
                direction = Some(Direction::Decreasing);
            };
        }

        if previous_level == *level {
            return false;
        }

        if !check(previous_level, *level, direction.unwrap()) {
            return false;
        }
    }

    true
}

pub fn check(first: Level, second: Level, direction: Direction) -> bool {
    let threshold = 3;

    match direction {
        Direction::Increasing => return first < second && (first - second).abs() <= threshold,
        Direction::Decreasing => return first > second && (first - second).abs() <= threshold,
    }
}

pub fn part1(input: &str) -> u32 {
    parse_reports(input)
        .into_iter()
        .map(|report| is_safe(&report))
        .filter(|&safe| safe)
        .count() as u32
}

pub fn part2(input: &str) -> u32 {
    parse_reports(input)
        .into_iter()
        .map(|report| {
            for index in 0..report.len() {
                let new_report: Report = report
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != index)
                    .map(|(_, &value)| value)
                    .collect();

                if is_safe(&new_report) {
                    return true;
                }
            }

            false
        })
        .filter(|&safe| safe)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 4);
    }
}
