use anyhow::{Context, Result};
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

fn simulate_movement(
    grid: &[Vec<char>],
    start_pos: (i32, i32),
    start_dir: Direction,
) -> Result<usize> {
    if grid.is_empty() {
        anyhow::bail!("Grid cannot be empty");
    }

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    if start_pos.0 < 0 || start_pos.0 >= rows || start_pos.1 < 0 || start_pos.1 >= cols {
        anyhow::bail!(
            "Invalid start position ({}, {}): must be within grid bounds (0..{}, 0..{})",
            start_pos.0,
            start_pos.1,
            rows,
            cols
        );
    }

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

    Ok(visited.len())
}

fn parse_input(input: &str) -> Result<(Vec<Vec<char>>, (i32, i32), Direction)> {
    if input.is_empty() {
        anyhow::bail!("Input cannot be empty");
    }

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    if grid.is_empty() {
        anyhow::bail!("Grid cannot be empty");
    }

    let width = grid[0].len();
    if !grid.iter().all(|row| row.len() == width) {
        anyhow::bail!("Grid rows must have equal length");
    }

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
        .context("No starting position (^) found in grid")?;

    Ok((grid, start_pos, Direction::Up))
}

fn part1(input: &str) -> Result<usize> {
    let (grid, start_pos, start_dir) = parse_input(input)?;
    simulate_movement(&grid, start_pos, start_dir)
}

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");
    dbg!(part1(input)?);
    Ok(())
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

        assert_eq!(part1(input).unwrap(), 41);
    }
}
