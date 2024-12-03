use regex::Regex;

pub fn run(input: String) {
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

pub fn solve(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .filter_map(|capture| {
            let x = capture[1].parse::<u32>().ok()?;
            let y = capture[2].parse::<u32>().ok()?;
            Some(x * y)
        })
        .sum()
}

pub fn solve_part1(input: &str) -> u32 {
    solve(input)
}

fn get_do_dont_instruction_offsets(input: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"do\(\)|don't\(\)").unwrap();
    re.find_iter(input)
        .map(|mat| (mat.start(), mat.end()))
        .collect()
}

fn get_valid_regions(
    do_dont_instruction_offsets: &Vec<(usize, usize)>,
    input: &str,
) -> Vec<(usize, usize)> {
    if do_dont_instruction_offsets.is_empty() {
        return vec![(0, input.len())];
    }

    let mut regions: Vec<(usize, usize)> = Vec::new();

    if do_dont_instruction_offsets[0].0 != 0 {
        regions.push((0, do_dont_instruction_offsets[0].0));
    }

    for (index, offset) in do_dont_instruction_offsets.iter().enumerate() {
        let instruction_text_length = offset.1 - offset.0;
        let is_do_instruction = instruction_text_length == 4;

        if !is_do_instruction {
            continue;
        }

        let start = offset.1;
        let mut end = start;
        let is_final_instruction = index == do_dont_instruction_offsets.len() - 1;

        if is_final_instruction {
            end = input.len();
        } else {
            let next_offset = do_dont_instruction_offsets[index + 1];
            end = next_offset.0;
        }

        regions.push((start, end));
    }

    regions
}

pub fn solve_part2(input: &str) -> u32 {
    let instruction_offsets = get_do_dont_instruction_offsets(input);
    let valid_regions = get_valid_regions(&instruction_offsets, input);

    let output: String = valid_regions
        .iter()
        .map(|region| &input[region.0..region.1])
        .collect();

    solve(&output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(solve_part1(&test_input), 161);
    }

    #[test]
    fn test_part2() {
        let test_input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(solve_part2(&test_input), 48);
    }
}
