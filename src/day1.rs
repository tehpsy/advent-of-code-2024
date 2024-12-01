use std::collections::HashMap;

pub fn run() {
    use std::fs;

    let input = fs::read_to_string("inputs/day1.txt").expect("Failed to read input file");

    println!("Distance sum: {:?}", solve_sum_distances(&input));
    println!("Similarity: {:?}", solve_similarity(&input));
}

pub fn solve_sum_distances(input: &str) -> u32 {
    let sorted_input: Vec<Vec<u32>> = parse_input(input)
        .into_iter()
        .map(|inner| {
            let mut sorted_inner = inner.clone();
            sorted_inner.sort();
            sorted_inner
        })
        .collect();

    sum_distances(sorted_input)
}

pub fn sum_distances(columns: Vec<Vec<u32>>) -> u32 {
    let num_rows = columns.first().map_or(0, |row| row.len());
    let mut sum: u32 = 0;
    for x in 0..num_rows {
        sum += (columns[0][x] as u32).abs_diff(columns[1][x] as u32);
    }

    sum
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let rows: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().expect("Invalid number"))
                .collect()
        })
        .collect();

    let num_columns = rows.first().map_or(0, |row| row.len());

    let mut columns: Vec<Vec<u32>> = vec![Vec::new(); num_columns];

    for row in rows {
        for (col_idx, value) in row.into_iter().enumerate() {
            columns[col_idx].push(value);
        }
    }

    columns
}

pub fn solve_similarity(input: &str) -> u32 {
    let parsed_input: Vec<Vec<u32>> = parse_input(input);
    let occurrence_lookup = count_occurrences(parsed_input[1].clone());

    parsed_input[0].iter().fold(0, |acc, &x| {
        acc + x * occurrence_lookup.get(&x).unwrap_or(&0)
    })
}

fn count_occurrences(numbers: Vec<u32>) -> HashMap<u32, u32> {
    let mut counts = HashMap::new();

    for number in numbers {
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
