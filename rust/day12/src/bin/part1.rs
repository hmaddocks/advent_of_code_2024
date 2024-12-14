fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .filter(|line: &Vec<char>| !line.is_empty())
        .collect()
}

#[derive(Debug)]
struct Region {
    area: usize,
    perimeter: usize,
}

fn flood_fill(
    grid: &[Vec<char>],
    visited: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize,
    ch: char,
) -> Region {
    let mut region = Region {
        area: 0,
        perimeter: 0,
    };
    let rows = grid.len();
    let cols = grid[0].len();
    let mut stack = vec![(i, j)];

    while let Some((i, j)) = stack.pop() {
        if visited[i][j] || grid[i][j] != ch {
            continue;
        }

        visited[i][j] = true;
        region.area += 1;

        // Count boundary edges
        if i == 0 {
            region.perimeter += 1;
        }
        if i == rows - 1 {
            region.perimeter += 1;
        }
        if j == 0 {
            region.perimeter += 1;
        }
        if j == cols - 1 {
            region.perimeter += 1;
        }

        // Count edges with different characters
        if i > 0 && grid[i - 1][j] != ch {
            region.perimeter += 1;
        }
        if i < rows - 1 && grid[i + 1][j] != ch {
            region.perimeter += 1;
        }
        if j > 0 && grid[i][j - 1] != ch {
            region.perimeter += 1;
        }
        if j < cols - 1 && grid[i][j + 1] != ch {
            region.perimeter += 1;
        }

        // Add adjacent cells of same character to stack
        if i > 0 && !visited[i - 1][j] && grid[i - 1][j] == ch {
            stack.push((i - 1, j));
        }
        if i < rows - 1 && !visited[i + 1][j] && grid[i + 1][j] == ch {
            stack.push((i + 1, j));
        }
        if j > 0 && !visited[i][j - 1] && grid[i][j - 1] == ch {
            stack.push((i, j - 1));
        }
        if j < cols - 1 && !visited[i][j + 1] && grid[i][j + 1] == ch {
            stack.push((i, j + 1));
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
                let region = flood_fill(grid, &mut visited, i, j, grid[i][j]);
                regions.push(region);
            }
        }
    }

    regions
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let regions = find_regions(&grid);
    regions.iter().map(|r| r.area * r.perimeter).sum()
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
