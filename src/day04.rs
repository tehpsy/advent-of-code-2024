#[derive(Debug, PartialEq, Clone, Copy)]
enum Character {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Position {
    y: i32,
    x: i32,
}

impl Position {
    fn offset_by(&self, offset: (i32, i32)) -> Position {
        Position {
            x: self.x as i32 + offset.0,
            y: self.y as i32 + offset.1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Top,
    Bottom,
    Right,
    Left,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        self.offset_by(1)
    }

    fn offset_by(&self, amount: i32) -> (i32, i32) {
        match self {
            Direction::Top => (0, -amount),
            Direction::Bottom => (0, amount),
            Direction::Right => (amount, 0),
            Direction::Left => (-amount, 0),
            Direction::TopRight => (amount, -amount),
            Direction::TopLeft => (-amount, -amount),
            Direction::BottomRight => (amount, amount),
            Direction::BottomLeft => (-amount, amount),
        }
    }

    fn all() -> [Direction; 8] {
        [
            Direction::Top,
            Direction::Bottom,
            Direction::Right,
            Direction::Left,
            Direction::TopRight,
            Direction::TopLeft,
            Direction::BottomRight,
            Direction::BottomLeft,
        ]
    }
}

impl From<char> for Character {
    fn from(value: char) -> Self {
        match value {
            'X' => Character::X,
            'M' => Character::M,
            'A' => Character::A,
            'S' => Character::S,
            _ => unreachable!("Input contains invalid character: {}", value),
        }
    }
}

pub fn run(input: String) {
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

pub fn solve_part1(input: &str) -> u32 {
    let grid = parse_input(input);

    grid.iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(move |(column_index, _)| Position {
                    x: column_index as i32,
                    y: row_index as i32,
                })
        })
        .map(|position| {
            Direction::all()
                .iter()
                .filter(|direction| found_match_part1(&grid, &position, direction))
                .count() as u32
        })
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    let grid = parse_input(input);

    grid.iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(move |(column_index, _)| Position {
                    x: column_index as i32,
                    y: row_index as i32,
                })
        })
        .filter(|position| found_match_part2(&grid, position))
        .count() as u32
}

fn parse_input(input: &str) -> Vec<Vec<Character>> {
    input
        .lines()
        .map(|line| line.chars().map(Character::from).collect())
        .collect()
}

fn check_character(
    grid: &Vec<Vec<Character>>,
    position: &Position,
    character_to_match: Character,
) -> bool {
    if let Some(row) = grid.get(position.y as usize) {
        if let Some(character) = row.get(position.x as usize) {
            return *character == character_to_match;
        }
    }
    false
}

fn found_match_part1(
    grid: &Vec<Vec<Character>>,
    position: &Position,
    direction: &Direction,
) -> bool {
    if !check_character(grid, &position, Character::X) {
        return false;
    }

    if !check_character(
        grid,
        &position.offset_by(direction.offset_by(1)),
        Character::M,
    ) {
        return false;
    }

    if !check_character(
        grid,
        &position.offset_by(direction.offset_by(2)),
        Character::A,
    ) {
        return false;
    }

    if !check_character(
        grid,
        &position.offset_by(direction.offset_by(3)),
        Character::S,
    ) {
        return false;
    }

    true
}

fn found_match_part2(grid: &Vec<Vec<Character>>, position: &Position) -> bool {
    if !check_character(grid, &position, Character::A) {
        return false;
    }

    let top_left = &position.offset_by(Direction::TopLeft.offset());
    let top_right = &position.offset_by(Direction::TopRight.offset());
    let bottom_left = &position.offset_by(Direction::BottomLeft.offset());
    let bottom_right = &position.offset_by(Direction::BottomRight.offset());

    let left_diagonal_matches = check_character(grid, top_left, Character::M)
        && check_character(grid, bottom_right, Character::S)
        || check_character(grid, top_left, Character::S)
            && check_character(grid, bottom_right, Character::M);

    let right_diagonal_matches = check_character(grid, top_right, Character::M)
        && check_character(grid, bottom_left, Character::S)
        || check_character(grid, top_right, Character::S)
            && check_character(grid, bottom_left, Character::M);

    left_diagonal_matches && right_diagonal_matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(solve_part1(&test_input), 18);
    }

    #[test]
    fn test_part2() {
        let test_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(solve_part2(&test_input), 9);
    }
}
