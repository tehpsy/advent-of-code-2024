use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn offset(&self, direction: Direction) -> Position {
        Position {
            x: self.x as i32 + direction.offset().0,
            y: self.y as i32 + direction.offset().1,
        }
    }

    fn neighbors_in_set(&self, others: &HashSet<Position>) -> HashSet<Direction> {
        Direction::all()
            .iter()
            .filter(|&&direction| others.contains(&self.offset(direction)))
            .cloned()
            .collect()
    }

    fn directly_adjacent_count(&self, others: &HashSet<Position>) -> usize {
        let horizontal_vertical: HashSet<Direction> = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .cloned()
        .collect();

        let neighbors = self.neighbors_in_set(others);

        neighbors.intersection(&horizontal_vertical).count()
    }

    fn diagonally_count(&self, others: &HashSet<Position>) -> usize {
        let horizontal_vertical: HashSet<Direction> = [
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
        .iter()
        .cloned()
        .collect();

        let neighbors = self.neighbors_in_set(others);

        neighbors.intersection(&horizontal_vertical).count()
    }

    fn neighbours_count(&self, directions: Vec<Direction>, others: &HashSet<Position>) -> u32 {
        let directions_hash = directions.iter().cloned().collect();

        let neighbors = self.neighbors_in_set(others);

        neighbors.intersection(&directions_hash).count() as u32
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (1, -1),
            Direction::DownLeft => (-1, 1),
            Direction::DownRight => (1, 1),
        }
    }

    fn all() -> [Direction; 8] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
    }
}

#[derive(Debug)]
struct Region {
    letter: char,
    positions: HashSet<Position>,
}

impl Region {
    fn perimeter(&self) -> u32 {
        self.positions
            .iter()
            .flat_map(|&position| {
                [
                    position.offset(Direction::Right),
                    position.offset(Direction::Left),
                    position.offset(Direction::Down),
                    position.offset(Direction::Up),
                ]
            })
            .filter(|neighbor| !self.positions.contains(neighbor))
            .count() as u32
    }

    fn area(&self) -> u32 {
        self.positions.len() as u32
    }

    fn count_corners(&self) -> u32 {
        self.positions
            .iter()
            .map(|position| self.count_corners_for(position))
            .sum()
    }

    fn count_corners_for(&self, position: &Position) -> u32 {
        let adjacent_count = position.directly_adjacent_count(&self.positions);

        if adjacent_count == 0 {
            return 4;
        }

        if adjacent_count == 1 {
            return 2;
        }

        if adjacent_count == 4 {
            return 4 - position.diagonally_count(&self.positions) as u32;
        }

        if adjacent_count == 2 {
            if position.neighbours_count(vec![Direction::Left, Direction::Right], &self.positions)
                == 2
                || position.neighbours_count(vec![Direction::Up, Direction::Down], &self.positions)
                    == 2
            {
                return 0;
            }

            if position.neighbours_count(vec![Direction::Left, Direction::Up], &self.positions) == 2
            {
                return 2 - position.neighbours_count(vec![Direction::UpLeft], &self.positions);
            }

            if position.neighbours_count(vec![Direction::Up, Direction::Right], &self.positions)
                == 2
            {
                return 2 - position.neighbours_count(vec![Direction::UpRight], &self.positions);
            }

            if position.neighbours_count(vec![Direction::Right, Direction::Down], &self.positions)
                == 2
            {
                return 2 - position.neighbours_count(vec![Direction::DownRight], &self.positions);
            }

            if position.neighbours_count(vec![Direction::Down, Direction::Left], &self.positions)
                == 2
            {
                return 2 - position.neighbours_count(vec![Direction::DownLeft], &self.positions);
            }
        }

        if adjacent_count == 3 {
            if !self.positions.contains(&position.offset(Direction::Left)) {
                return 2 - position.neighbours_count(
                    vec![Direction::UpRight, Direction::DownRight],
                    &self.positions,
                );
            }

            if !self.positions.contains(&position.offset(Direction::Up)) {
                return 2 - position.neighbours_count(
                    vec![Direction::DownLeft, Direction::DownRight],
                    &self.positions,
                );
            }

            if !self.positions.contains(&position.offset(Direction::Right)) {
                return 2 - position.neighbours_count(
                    vec![Direction::UpLeft, Direction::DownLeft],
                    &self.positions,
                );
            }

            if !self.positions.contains(&position.offset(Direction::Down)) {
                return 2 - position.neighbours_count(
                    vec![Direction::UpLeft, Direction::UpRight],
                    &self.positions,
                );
            }
        }

        0
    }
}

fn form_regions(grid: Vec<Vec<char>>) -> Vec<Region> {
    let mut visited = HashSet::new();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut regions = Vec::new();

    for y in 0..rows {
        for x in 0..cols {
            let position = Position {
                x: x as i32,
                y: y as i32,
            };
            if visited.contains(&position) {
                continue;
            }

            let mut region_positions = HashSet::new();
            let letter = grid[y][x];

            explore_region(&grid, &mut visited, &mut region_positions, letter, position);

            regions.push(Region {
                letter,
                positions: region_positions,
            });
        }
    }

    regions
}

fn explore_region(
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<Position>,
    region_positions: &mut HashSet<Position>,
    letter: char,
    position: Position,
) {
    if visited.contains(&position) {
        return;
    }

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    if position.x >= cols
        || position.x < 0
        || position.y >= rows
        || position.y < 0
        || grid[position.y as usize][position.x as usize] != letter
    {
        return;
    }

    visited.insert(position.clone());
    region_positions.insert(position);

    for neighbor in [
        position.offset(Direction::Right),
        position.offset(Direction::Left),
        position.offset(Direction::Down),
        position.offset(Direction::Up),
    ] {
        explore_region(grid, visited, region_positions, letter, neighbor);
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn run(input: String) {
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

pub fn solve_part1(input: &str) -> u32 {
    let grid = parse_input(input);
    let regions = form_regions(grid);
    regions
        .iter()
        .map(|region| region.perimeter() * region.area())
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    let grid = parse_input(input);
    let regions = form_regions(grid);
    regions
        .iter()
        .map(|region| region.count_corners() * region.area())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 1206);
    }

    #[test]
    fn test_ex1() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(solve_part2(input), 80);
    }

    #[test]
    fn test_ex2() {
        let input = "BB
BB";
        assert_eq!(solve_part2(input), 16);
    }

    #[test]
    fn test_count_corners() {
        let input = "..XXX
XX.XX
XXXX.";

        let grid = parse_input(input);
        let regions = form_regions(grid);
        let corners: u32 = regions.iter().map(|region| region.count_corners()).sum();

        assert_eq!(corners, 24);
    }
}
