use regex::Regex;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd)]
struct Position {
    x: i64,
    y: i64,
}

struct Machine {
    offset_a: Position,
    offset_b: Position,
    target: Position,
}

pub fn run(input: String) {
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn parse_offset(input: &str) -> Position {
    let re = Regex::new(r"X\+(-?\d+), Y\+(-?\d+)").unwrap();

    re.captures(input)
        .map(|caps| {
            let x: i64 = caps[1].parse().unwrap();
            let y: i64 = caps[2].parse().unwrap();
            Position { x, y }
        })
        .unwrap()
}

fn parse_target(input: &str, append_extra: bool) -> Position {
    let re = Regex::new(r"X\=(-?\d+), Y\=(-?\d+)").unwrap();

    re.captures(input)
        .map(|caps| {
            if append_extra {
                let x: i64 = format!("10000000000000{}", &caps[1]).parse().unwrap();
                let y: i64 = format!("10000000000000{}", &caps[2]).parse().unwrap();
                Position { x, y }
            } else {
                let x: i64 = caps[1].parse().unwrap();
                let y: i64 = caps[2].parse().unwrap();
                Position { x, y }
            }
        })
        .unwrap()
}

fn parse_input(input: &str, append_extra: bool) -> Vec<Machine> {
    let machine_strings: Vec<&str> = input.split("\n\n").map(|s| s.trim()).collect();

    machine_strings
        .iter()
        .map(|machine_string| {
            return Machine {
                offset_a: parse_offset(machine_string.lines().nth(0).unwrap()),
                offset_b: parse_offset(machine_string.lines().nth(1).unwrap()),
                target: parse_target(machine_string.lines().nth(2).unwrap(), append_extra),
            };
        })
        .collect()
}

fn is_integer(value: f64) -> bool {
    let rounded_value = value.round();
    (value - rounded_value).abs() < 0.00001
}

fn is_in_range(value: f64, min: f64, max: f64) -> bool {
    value >= min && value <= max
}

pub fn intersection_for(machine: &Machine) -> (f64, f64) {
    let y_intersection = (machine.target.y as f64
        - (machine.offset_b.y as f64 * machine.target.x as f64 / machine.offset_b.x as f64))
        / (1.0
            - (machine.offset_b.y as f64 * machine.offset_a.x as f64)
                / (machine.offset_b.x as f64 * machine.offset_a.y as f64));

    let x_intersection =
        y_intersection as f64 * machine.offset_a.x as f64 / machine.offset_a.y as f64;

    (x_intersection, y_intersection)
}

pub fn min_tickets(machine: &Machine) -> Option<u64> {
    let intersect = intersection_for(machine);

    if !is_in_range(intersect.0, 0.0, machine.target.x as f64)
        || !is_in_range(intersect.1, 0.0, machine.target.y as f64)
    {
        return None;
    }

    let num_a = intersect.1 / machine.offset_a.y as f64;
    let num_b = (machine.target.y as f64 - intersect.1) / machine.offset_b.y as f64;

    if !is_integer(num_a) || !is_integer(num_b) {
        return None;
    }

    let num_tickets_a = num_a.round() as u64 * 3;
    let num_tickets_b = num_b.round() as u64 * 1;
    Some(num_tickets_a + num_tickets_b)
}

pub fn solve_part1(input: &str) -> u64 {
    parse_input(input, false)
        .iter()
        .filter_map(|machine| min_tickets(machine))
        .sum()
}

pub fn solve_part2(input: &str) -> u64 {
    parse_input(input, true)
        .iter()
        .filter_map(|machine| min_tickets(machine))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 0);
    }
}
