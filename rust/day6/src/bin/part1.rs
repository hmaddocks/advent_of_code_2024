use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_forward(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0, pos.1 + 1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
        }
    }
}

fn simulate_movement(grid: &[Vec<char>], start_pos: (i32, i32), start_dir: Direction) -> usize {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut pos = start_pos;
    let mut dir = start_dir;
    let mut visited = HashSet::new();
    visited.insert(pos);

    loop {
        // Check if next position is blocked or out of bounds
        let next_pos = dir.move_forward(pos);
        if next_pos.0 < 0 || next_pos.0 >= rows || next_pos.1 < 0 || next_pos.1 >= cols {
            break;
        }

        let next_cell = grid[next_pos.0 as usize][next_pos.1 as usize];
        if next_cell == '#' {
            // Turn right if blocked
            dir = dir.turn_right();
        } else {
            // Move forward
            pos = next_pos;
            visited.insert(pos);
        }
    }

    visited.len()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (i32, i32), Direction) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Find starting position using iterator methods
    let (start_pos, _) = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &ch)| ch == '^')
                .map(|(j, _)| ((i as i32, j as i32), Direction::Up))
        })
        .expect("No starting position found");

    (grid, start_pos, Direction::Up)
}

fn part1(input: &str) -> usize {
    let (grid, start_pos, start_dir) = parse_input(input);
    simulate_movement(&grid, start_pos, start_dir)
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        assert_eq!(part1(input), 41);
    }
}
