pub fn run() {
    use std::fs;

    let input = fs::read_to_string("inputs/day1.txt").expect("Failed to read input file");

    println!("Distance sum: {:?}", solve(&input));
}

pub fn solve(input: &str) -> u32 {
    let sorted_input: Vec<Vec<u8>> = parse_input(input)
        .into_iter()
        .map(|inner| {
            let mut sorted_inner = inner.clone();
            sorted_inner.sort();
            sorted_inner
        })
        .collect();

    sum_distances(sorted_input)
}

pub fn sum_distances(columns: Vec<Vec<u8>>) -> u32 {
    let num_rows = columns.first().map_or(0, |row| row.len());
    let mut sum: u32 = 0;
    for x in 0..num_rows {
        sum += (columns[0][x] as u32).abs_diff(columns[1][x] as u32);
    }

    sum
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let rows: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u8>().expect("Invalid number"))
                .collect()
        })
        .collect();

    let num_columns = rows.first().map_or(0, |row| row.len());

    let mut columns: Vec<Vec<u8>> = vec![Vec::new(); num_columns];

    for row in rows {
        for (col_idx, value) in row.into_iter().enumerate() {
            columns[col_idx].push(value);
        }
    }

    columns
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
