#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn offset_by(&self, direction: Direction) -> Position {
        Position {
            x: self.x as i32 + direction.offset().0,
            y: self.y as i32 + direction.offset().1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn bit(self) -> u8 {
        match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }

    fn offset(&self) -> (i32, i32) {
        self.offset_by(1)
    }

    fn offset_by(&self, amount: i32) -> (i32, i32) {
        match self {
            Direction::Up => (0, -amount),
            Direction::Down => (0, amount),
            Direction::Left => (-amount, 0),
            Direction::Right => (amount, 0),
        }
    }

    fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

type Grid = Vec<Vec<Cell>>;

pub fn get_cell(grid: &mut Grid, position: Position) -> Option<&mut Cell> {
    grid.get_mut(position.y as usize)
        .and_then(|row| row.get_mut(position.x as usize))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Cell {
    value: u8,
    incline_bitset: u8,
    visited: bool,
    position: Position,
}

impl Cell {
    fn can_climb_to(&self, direction: Direction) -> bool {
        (self.incline_bitset & (1 << direction.bit())) != 0
    }

    fn enable_climb_to(&mut self, direction: Direction) {
        self.incline_bitset |= 1 << direction.bit();
    }
}

pub fn run(input: String) {
    println!("Part 2: {}", solve(&input, true));
    println!("Part 1: {}", solve(&input, false));
}

fn parse_input(input: &str) -> Grid {
    let mut grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    c.to_digit(10).map(|d| Cell {
                        value: d as u8,
                        incline_bitset: 0,
                        visited: false,
                        position: Position {
                            x: x as i32,
                            y: y as i32,
                        },
                    })
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Grid>();

    let bounds = (grid[0].len(), grid.len());

    for y in 0..bounds.1 {
        for x in 0..bounds.0 {
            let mut cell = grid[y][x].clone();
            let position = cell.position;

            for direction in Direction::all() {
                let new_position = position.offset_by(direction);

                if let Some(new_cell) = get_cell(&mut grid, new_position) {
                    if new_cell.value == cell.value + 1 {
                        cell.enable_climb_to(direction);
                    }
                }
            }

            grid[y][x] = cell;
        }
    }

    grid
}

pub fn visit(grid: &mut Grid, position: Position, ignore_visited_cells: bool) -> u32 {
    let mut count = 0;

    if let Some(cell) = get_cell(grid, position) {
        if ignore_visited_cells && cell.visited {
            return 0;
        }

        cell.visited = true;

        if cell.value == 9 {
            return 1;
        }

        let climbable_directions = Direction::all()
            .into_iter()
            .filter(|&direction| cell.can_climb_to(direction))
            .collect::<Vec<_>>();

        let cell_position = cell.position;
        for direction in climbable_directions {
            let new_position = cell_position.offset_by(direction);
            count += visit(grid, new_position, ignore_visited_cells);
        }
    }

    count
}

pub fn solve(input: &str, ignore_visited_cells: bool) -> u32 {
    let grid = parse_input(input);

    let mut total = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.value == 0 {
                let mut cloned_grid = grid.clone();
                total += visit(
                    &mut cloned_grid,
                    Position {
                        x: x as i32,
                        y: y as i32,
                    },
                    ignore_visited_cells,
                );
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!(solve(TEST_INPUT, true), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(TEST_INPUT, false), 81);
    }
}
