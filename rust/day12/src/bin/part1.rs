fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

#[derive(Debug, Default)]
struct Region {
    area: usize,
    perimeter: usize,
}

impl Region {
    fn score(&self) -> usize {
        self.area * self.perimeter
    }
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn flood_fill(
    grid: &[Vec<char>],
    visited: &mut Vec<Vec<bool>>,
    start_i: usize,
    start_j: usize,
    ch: char,
) -> Region {
    let mut region = Region::default();
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut stack = vec![(start_i, start_j)];

    while let Some((i, j)) = stack.pop() {
        if visited[i][j] || grid[i][j] != ch {
            continue;
        }

        visited[i][j] = true;
        region.area += 1;

        // Count boundary edges
        region.perimeter += (i == 0) as usize
            + (i == grid.len() - 1) as usize
            + (j == 0) as usize
            + (j == grid[0].len() - 1) as usize;

        // Check all adjacent cells
        let i = i as isize;
        let j = j as isize;
        for (di, dj) in DIRECTIONS {
            let ni = i + di;
            let nj = j + dj;

            if ni >= 0 && ni < rows && nj >= 0 && nj < cols {
                let (ni, nj) = (ni as usize, nj as usize);
                if grid[ni][nj] != ch {
                    region.perimeter += 1;
                } else if !visited[ni][nj] {
                    stack.push((ni, nj));
                }
            }
        }
    }

    region
}

fn find_regions(grid: &[Vec<char>]) -> Vec<Region> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut regions = Vec::new();

    for i in 0..rows {
        for j in 0..cols {
            if !visited[i][j] {
                regions.push(flood_fill(grid, &mut visited, i, j, grid[i][j]));
            }
        }
    }

    regions
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    find_regions(&grid).iter().map(Region::score).sum()
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = "A";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn test_two() {
        let input = "BB";
        assert_eq!(part1(input), 12);
    }

    #[test]
    fn test_six() {
        let input = "
        AAB
        CCB
        ";
        assert_eq!(part1(input), 36);
    }

    #[test]
    fn test_part1() {
        let input = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
        ";

        assert_eq!(part1(input), 1930);
    }
}
