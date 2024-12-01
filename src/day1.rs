use std::collections::HashMap;

type Matrix = Vec<Vec<u32>>;

fn read_input(file: &str) -> String {
    std::fs::read_to_string(file).expect("Failed to read input file")
}

pub fn run() {
    let input = read_input("inputs/day1.txt");

    println!("Distance sum: {}", solve_sum_distances(&input));
    println!("Similarity: {}", solve_similarity(&input));
}

pub fn solve_sum_distances(input: &str) -> u32 {
    let sorted_input: Matrix = parse_input(input)
        .into_iter()
        .map(|mut inner| {
            inner.sort();
            inner
        })
        .collect();

    sum_distances(sorted_input)
}

pub fn sum_distances(columns: Matrix) -> u32 {
    columns[0]
        .iter()
        .zip(&columns[1])
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn parse_input(input: &str) -> Matrix {
    let rows: Matrix = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().expect("Invalid number"))
                .collect()
        })
        .collect();

    transpose(rows)
}

fn transpose(rows: Matrix) -> Matrix {
    if rows.is_empty() {
        return Vec::new();
    }
    let num_columns = rows[0].len();
    (0..num_columns)
        .map(|col_idx| rows.iter().map(|row| row[col_idx]).collect())
        .collect()
}

pub fn solve_similarity(input: &str) -> u32 {
    let parsed_input = parse_input(input);
    let occurrence_lookup = count_occurrences(&parsed_input[1]);

    parsed_input[0]
        .iter()
        .map(|&x| x * *occurrence_lookup.get(&x).unwrap_or(&0) as u32)
        .sum()
}

fn count_occurrences(numbers: &[u32]) -> HashMap<u32, usize> {
    let mut counts = HashMap::new();
    for &number in numbers {
        *counts.entry(number).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(solve_sum_distances(&test_input), 11);
    }

    #[test]
    fn test_part2() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(solve_similarity(&test_input), 31);
    }
}
