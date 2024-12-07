use regex::Regex;

type Number = u64;

#[derive(Debug, Clone, PartialEq)]
pub struct Equation {
    result: Number,
    operands: Vec<Number>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    fn all() -> [Operator; 3] {
        [Operator::Add, Operator::Multiply, Operator::Concat]
    }
}

fn parse_equation(input: &str) -> Option<Equation> {
    let re = Regex::new(r"^(\d+):\s*([\d\s]+)$").unwrap();
    if let Some(captures) = re.captures(input) {
        let result = captures[1].parse::<Number>().ok()?;
        let operands = captures[2]
            .split_whitespace()
            .filter_map(|s| s.parse::<Number>().ok())
            .collect();
        Some(Equation { result, operands })
    } else {
        None
    }
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .filter_map(|line| parse_equation(line))
        .collect()
}

pub fn run(input: String) {
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

pub fn perform_operation(operator: Operator, operand1: Number, operand2: Number) -> Number {
    match operator {
        Operator::Add => operand1 + operand2,
        Operator::Multiply => operand1 * operand2,
        Operator::Concat => {
            let operand2_num_digits = (operand2 as f64).log10().floor() as u64 + 1;
            return operand1 * 10u64.pow(operand2_num_digits as u32) + operand2;
        }
    }
}

pub fn could_be_true(equation: &Equation, allow_concat: bool) -> bool {
    if equation.operands.len() == 1 {
        return equation.operands[0] == equation.result;
    }

    if equation.operands[0] > equation.result {
        return false;
    }

    for operator in Operator::all() {
        if !allow_concat && operator == Operator::Concat {
            continue;
        }

        let result = perform_operation(operator, equation.operands[0], equation.operands[1]);
        let mut cloned_eq = equation.clone();
        cloned_eq.operands.splice(0..2, std::iter::once(result));

        if could_be_true(&cloned_eq, allow_concat) {
            return true;
        }
    }

    false
}

pub fn solve_part1(input: &str) -> Number {
    parse_input(input)
        .iter()
        .filter(|equation| could_be_true(equation, false))
        .map(|equation| equation.result)
        .sum::<Number>()
}

pub fn solve_part2(input: &str) -> Number {
    parse_input(input)
        .iter()
        .filter(|equation| could_be_true(equation, true))
        .map(|equation| equation.result)
        .sum::<Number>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 11387);
    }
}
