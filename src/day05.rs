use std::collections::{HashMap, HashSet};

type Page = u8;
type Rule = (Page, Page);
type Update = Vec<Page>;
type Rules = HashMap<Page, HashSet<Page>>;

pub fn run(input: String) {
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

pub fn solve_part1(input: &str) -> u32 {
    let (rules_input, updates_input) = split_input(input);
    let rules = parse_rules(rules_input);
    let updates = parse_updates(updates_input);

    updates
        .into_iter()
        .filter(|update| in_correct_order(update, &rules))
        .map(|update| get_middle_page(&update) as u32)
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    let (rules_input, updates_input) = split_input(input);
    let rules = parse_rules(rules_input);
    let rules_vec = parse_rules_vec(rules_input);
    let updates = parse_updates(updates_input);

    updates
        .into_iter()
        .filter(|update| !in_correct_order(update, &rules))
        .map(|update| reorder(&update, &rules_vec))
        .map(|update| get_middle_page(&update) as u32)
        .sum()
}

fn split_input(input: &str) -> (&str, &str) {
    input.split_once("\n\n").unwrap()
}

fn parse_rules(input: &str) -> Rules {
    input.lines().fold(HashMap::new(), |mut map, line| {
        if let Some((key, value)) = line.split_once('|') {
            if let (Ok(x), Ok(y)) = (key.parse::<Page>(), value.parse::<Page>()) {
                map.entry(x).or_insert_with(HashSet::new).insert(y);
            }
        }
        map
    })
}

fn parse_rules_vec(input: &str) -> Vec<Rule> {
    input
        .lines()
        .filter_map(|line| {
            if let Some((key, value)) = line.split_once('|') {
                if let (Ok(x), Ok(y)) = (key.parse::<Page>(), value.parse::<Page>()) {
                    return Some((x, y));
                }
            }
            None
        })
        .collect()
}

fn parse_updates(input: &str) -> Vec<Update> {
    input.lines().fold(Vec::new(), |mut vec, line| {
        vec.push(
            line.split(',')
                .filter_map(|num| num.trim().parse::<u8>().ok())
                .collect(),
        );

        vec
    })
}

fn reorder(update: &Update, rules_vec: &Vec<Rule>) -> Update {
    let mut reordered = update.clone();

    loop {
        let mut changed = false;

        for rule in rules_vec {
            if let (Some(first), Some(second)) = (
                reordered.iter().position(|&x| x == rule.0),
                reordered.iter().position(|&x| x == rule.1),
            ) {
                if first > second {
                    reordered.swap(first, second);
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }

    reordered
}

fn has_rule(rules: &Rules, first_number: Page, second_number: Page) -> bool {
    rules
        .get(&first_number)
        .map_or(false, |set| set.contains(&second_number))
}

fn get_middle_page(update: &Update) -> Page {
    update[(update.len() - 1) / 2]
}

fn in_correct_order(update: &Update, rules: &Rules) -> bool {
    for i in 0..update.len() {
        for j in i..update.len() {
            if i != j {
                if !has_rule(rules, update[i], update[j] as Page) {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 123);
    }
}
