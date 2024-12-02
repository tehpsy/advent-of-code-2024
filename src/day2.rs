type Level = i32;
type Report = Vec<Level>;
type Reports = Vec<Report>;

fn read_input(file: &str) -> String {
    std::fs::read_to_string(file).expect("Failed to read input file")
}

pub fn run() {
    let input = read_input("inputs/day2.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
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

pub fn parse_report(input: &str) -> Report {
    input
        .split_whitespace()
        .map(|text| text.parse::<Level>().expect("Invalid number"))
        .collect()
}

pub fn parse_reports(input: &str) -> Reports {
    input.lines().map(|line| parse_report(line)).collect()
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

    let mut previous_level: Option<Level> = None;
    let mut direction: Option<Direction> = None;

    for level in report {
        if direction == None && previous_level != None && *level != previous_level.unwrap() {
            if *level > previous_level.unwrap() {
                direction = Some(Direction::Increasing);
            } else {
                direction = Some(Direction::Decreasing);
            };
        }

        if previous_level != None {
            if previous_level.unwrap() == *level {
                return false;
            }

            if !check(previous_level.unwrap(), *level, direction.unwrap()) {
                return false;
            }
        }

        previous_level = Some(*level);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "7 6 4 2 1
         1 2 7 8 9
         9 7 6 2 1
         1 3 2 4 5
         8 6 4 4 1
         1 3 6 7 9";

        assert_eq!(part1(&test_input), 2);
    }

    #[test]
    fn test_part2() {
        let test_input = "7 6 4 2 1
         1 2 7 8 9
         9 7 6 2 1
         1 3 2 4 5
         8 6 4 4 1
         1 3 6 7 9";

        assert_eq!(part2(&test_input), 4);
    }
}
