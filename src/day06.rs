#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    y: i32,
    x: i32,
}

impl Position {
    fn offset(&self, direction: Direction) -> Position {
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

    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
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
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err("Invalid character for Direction"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridCell {
    has_obstacle: bool,
    exit_bitset: u8,
    last_exited: Option<Direction>,
}

impl GridCell {
    fn exit_from(&mut self, direction: Direction) {
        self.last_exited = Some(direction);
        self.exit_bitset |= 1 << direction.bit();
    }

    fn has_exited(&mut self, direction: Direction) -> bool {
        self.exit_bitset & 1 << direction.bit() != 0
    }
}

impl From<char> for GridCell {
    fn from(value: char) -> Self {
        match value {
            '.' => GridCell {
                has_obstacle: false,
                exit_bitset: 0,
                last_exited: None,
            },
            '#' => GridCell {
                has_obstacle: true,
                exit_bitset: 0,
                last_exited: None,
            },
            '^' => GridCell {
                has_obstacle: false,
                exit_bitset: Direction::Up.bit(),
                last_exited: Some(Direction::Up),
            },
            _ => unreachable!("Input contains invalid character: {}", value),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    guard: Option<(Position, Direction)>,
    rows: Vec<Vec<GridCell>>,
}

impl Grid {
    pub fn display(&self) {
        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some((guard_pos, guard_dir)) = self.guard {
                    if guard_pos.x == x as i32 && guard_pos.y == y as i32 {
                        print!(
                            "{}",
                            match guard_dir {
                                Direction::Up => '^',
                                Direction::Down => 'v',
                                Direction::Left => '<',
                                Direction::Right => '>',
                            }
                        );
                        continue;
                    }
                }

                if cell.has_obstacle {
                    print!("#");
                } else if cell.last_exited.is_none() {
                    print!(".");
                } else if cell.last_exited == Some(Direction::Up) {
                    print!("↑");
                } else if cell.last_exited == Some(Direction::Down) {
                    print!("↓");
                } else if cell.last_exited == Some(Direction::Left) {
                    print!("←");
                } else if cell.last_exited == Some(Direction::Right) {
                    print!("→");
                }
            }
            println!();
        }

        println!();
    }
}

impl Grid {
    pub fn get_cell(&mut self, position: Position) -> Option<&mut GridCell> {
        self.rows
            .get_mut(position.y as usize)
            .and_then(|row| row.get_mut(position.x as usize))
    }

    pub fn step(&mut self) {
        let new_position = self.guard.unwrap().0.offset(self.guard.unwrap().1);
        let current_position = self.guard.unwrap().0;
        let current_direction = self.guard.unwrap().1;

        if let Some(new_cell) = self.get_cell(new_position) {
            if new_cell.has_obstacle {
                self.guard = Some((current_position, current_direction.rotate()));
            } else {
                self.get_cell(current_position)
                    .unwrap()
                    .exit_from(current_direction);
                self.guard = Some((new_position, current_direction));
            }
        } else {
            self.get_cell(current_position)
                .unwrap()
                .exit_from(current_direction);

            self.guard = None
        }
    }
}

fn parse_initial_direction(input: &str) -> Option<(Position, Direction)> {
    for (row, line) in input.lines().enumerate() {
        for (col, character) in line.chars().enumerate() {
            if let Ok(direction) = Direction::try_from(character) {
                return Some((
                    Position {
                        y: row as i32,
                        x: col as i32,
                    },
                    direction,
                ));
            }
        }
    }
    None
}

fn parse_cells(input: &str) -> Vec<Vec<GridCell>> {
    input
        .lines()
        .map(|line| line.chars().map(GridCell::from).collect())
        .collect()
}

fn parse_input(input: &str) -> Grid {
    Grid {
        guard: parse_initial_direction(input),
        rows: parse_cells(input),
    }
}

fn count_visited(grid: &Grid) -> u32 {
    grid.rows
        .iter()
        .flat_map(|row| row.iter())
        .filter(|cell| !cell.last_exited.is_none())
        .count() as u32
}

pub fn run(input: String) {
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

pub fn solve_part1(input: &str) -> u32 {
    let mut grid = parse_input(input);

    loop {
        grid.step();
        if grid.guard.is_none() {
            break;
        }
    }

    grid.display();

    count_visited(&grid)
}

fn has_loop(grid: &mut Grid) -> bool {
    loop {
        grid.step();

        if let Some(guard) = grid.guard {
            if let Some(cell) = grid.get_cell(guard.0) {
                if cell.has_exited(guard.1) {
                    return true;
                }
            }
        } else {
            return false;
        }
    }
}

pub fn solve_part2(input: &str) -> u32 {
    let original_grid = parse_input(input);

    let mut loop_count = 0;

    for row in 0..original_grid.rows.len() {
        for column in 0..original_grid.rows[0].len() {
            let mut grid = original_grid.clone();
            let position = Position {
                y: row as i32,
                x: column as i32,
            };

            if position == grid.guard.unwrap().0 {
                continue;
            }

            if grid.get_cell(position).unwrap().has_obstacle {
                continue;
            }

            grid.get_cell(position).unwrap().has_obstacle = true;

            if has_loop(&mut grid) {
                loop_count += 1;
            }
        }
    }

    loop_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&TEST_INPUT), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&TEST_INPUT), 6);
    }

    #[test]
    fn test_line() {
        let input: &str = ".#.....#..
#........#
.^......#.
";
        let mut grid = parse_input(input);
        assert_eq!(has_loop(&mut grid), true);
    }
}
