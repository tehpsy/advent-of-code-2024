use std::collections::{HashMap, HashSet};
use std::ops::{Add, Neg, Sub};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Neg for Position {
    type Output = Position;

    fn neg(self) -> Position {
        Position {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add<(i32, i32)> for Position {
    type Output = Position;

    fn add(self, offset: (i32, i32)) -> Position {
        Position {
            x: self.x + offset.0,
            y: self.y + offset.1,
        }
    }
}

impl Position {
    fn offset_from(&self, other: &Position) -> Position {
        Position {
            x: self.x as i32 - other.x,
            y: self.y as i32 - other.y,
        }
    }

    fn is_in_bounds(&self, bounds: (i32, i32)) -> bool {
        self.x >= 0 && self.x < bounds.0 && self.y >= 0 && self.y < bounds.1
    }

    fn check_bounds(&self, bounds: (i32, i32)) -> Option<Position> {
        if self.is_in_bounds(bounds) {
            return Some(*self);
        }

        None
    }
}

pub fn run(input: String) {
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn parse_input(input: &str) -> HashMap<char, HashSet<Position>> {
    input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (y, line)| {
            for (x, ch) in line.chars().enumerate() {
                if ch != '.' {
                    map.entry(ch).or_insert_with(HashSet::new).insert(Position {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
            map
        })
}

fn bounds(input: &str) -> (i32, i32) {
    let max_x = input.lines().next().map_or(0, |line| line.chars().count()) as i32;
    let max_y = input.lines().count() as i32;
    (max_x, max_y)
}

pub fn solve_part1(input: &str) -> usize {
    let antennas = parse_input(input);
    let bounds = bounds(input);
    let mut antinodes: HashSet<Position> = HashSet::new();

    for (_, positions_hashset) in &antennas {
        let positions: Vec<_> = positions_hashset.iter().collect();

        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let antenna1 = positions[i];
                let antenna2 = positions[j];
                let offset = antenna2.offset_from(antenna1);

                if let Some(antinode1_position) = (*antenna1 - offset).check_bounds(bounds) {
                    antinodes.insert(antinode1_position);
                }

                if let Some(antinode2_position) = (*antenna2 + offset).check_bounds(bounds) {
                    antinodes.insert(antinode2_position);
                }
            }
        }
    }

    antinodes.len()
}

fn collect_positions_in_line(
    start: Position,
    offset: Position,
    bounds: (i32, i32),
    antinodes: &mut HashSet<Position>,
) {
    let mut current_position = start;

    while current_position.is_in_bounds(bounds) {
        antinodes.insert(current_position);
        current_position = current_position + offset;
    }
}

pub fn solve_part2(input: &str) -> usize {
    let antennas = parse_input(input);
    let bounds = bounds(input);
    let mut antinodes: HashSet<Position> = HashSet::new();

    for (_, positions_hashset) in &antennas {
        let positions: Vec<_> = positions_hashset.iter().collect();

        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let antenna1 = positions[i];
                let antenna2 = positions[j];
                let offset = antenna2.offset_from(antenna1);

                collect_positions_in_line(*antenna2, offset, bounds, &mut antinodes);
                collect_positions_in_line(*antenna2, -offset, bounds, &mut antinodes);
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 34);
    }
}
