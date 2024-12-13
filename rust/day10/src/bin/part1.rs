use anyhow::Result;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    col: i32,
}

fn parse_input(input: &str) -> Result<Vec<Vec<u32>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or_else(|| anyhow::anyhow!("Invalid digit '{}'", c))
                })
                .collect()
        })
        .collect()
}

fn find_paths(grid: &[Vec<u32>]) -> Vec<Vec<Point>> {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut paths = Vec::new();
    let mut visited = HashSet::new();
    
    // Find starting points (value 0)
    let mut start_points = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            if grid[row as usize][col as usize] == 0 {
                start_points.push(Point { row, col });
            }
        }
    }

    // Directions: up, right, down, left only (no diagonals)
    let directions = [
        (-1, 0), (1, 0), (0, -1), (0, 1)
    ];

    for start in start_points {
        let mut queue = VecDeque::new();
        queue.push_back(vec![start]);
        visited.clear();
        visited.insert(start);

        while let Some(current_path) = queue.pop_front() {
            let current = *current_path.last().unwrap();
            let current_value = grid[current.row as usize][current.col as usize];

            if current_value == 9 {
                paths.push(current_path.clone());
                continue;
            }

            for (dx, dy) in directions.iter() {
                let new_row = current.row + dx;
                let new_col = current.col + dy;

                if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
                    let new_point = Point { row: new_row, col: new_col };
                    let new_value = grid[new_row as usize][new_col as usize];

                    if new_value == current_value + 1 && !visited.contains(&new_point) {
                        let mut new_path = current_path.clone();
                        new_path.push(new_point);
                        visited.insert(new_point);
                        queue.push_back(new_path);
                    }
                }
            }
        }
    }

    paths
}

fn part1(input: &str) -> Result<i64> {
    let grid = parse_input(input)?;
    let paths = find_paths(&grid);
    Ok(paths.len() as i64)
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
    fn test() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        assert_eq!(part1(input).unwrap(), 36);
    }
}
