use std::collections::HashMap;

pub fn run(input: String) {
    println!("25: {}", solve_recursive(&input, 25));
    println!("75: {}", solve_recursive(&input, 75));
}

type Number = u64;

fn parse_input(input: &str) -> Vec<Number> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse::<Number>().ok())
        .collect()
}

fn split_number(num: Number) -> (Number, Number) {
    let num_digits = (num as f64).log10() as Number + 1;
    let half_digits = num_digits / 2;
    let divisor = 10u64.pow(half_digits as u32);

    let first_half = num / divisor;
    let second_half = num % divisor;

    (first_half, second_half)
}

fn transform(val: Number) -> Vec<Number> {
    if val == 0 {
        return vec![1];
    }

    let num_digits = (val as f64).log10() as Number + 1;

    if num_digits % 2 == 0 {
        let (first_half, second_half) = split_number(val);

        return vec![first_half, second_half];
    }

    vec![val * 2024]
}

fn count_stones(
    vec: &Vec<Number>,
    blink_count: u32,
    max_blink_count: u32,

    cache: &mut HashMap<(Number, u32), u64>,
) -> u64 {
    if blink_count == max_blink_count {
        return vec.len() as u64;
    }

    vec.iter()
        .map(|&val| {
            if let Some(&cached_result) = cache.get(&(val, blink_count)) {
                return cached_result;
            }

            let stones = transform(val);
            let result = count_stones(&stones, blink_count + 1, max_blink_count, cache);

            cache.insert((val, blink_count), result);

            result
        })
        .sum()
}

pub fn solve_recursive(input: &str, blinks: u32) -> u64 {
    let stones = parse_input(input);
    let mut cache = HashMap::new();
    count_stones(&stones, 0, blinks, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_recursive() {
        assert_eq!(solve_recursive(TEST_INPUT, 25), 55312);
    }
}
