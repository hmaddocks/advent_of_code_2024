fn check_mas_from_center(
    grid: &Vec<Vec<char>>,
    center_row: isize,
    center_col: isize,
    dr: isize,
    dc: isize,
) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    // Check if all positions are within bounds
    for i in -1..=1 {
        let nr = center_row + i * dr;
        let nc = center_col + i * dc;
        if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
            return false;
        }
    }

    // Get the three characters in order
    let chars: Vec<char> = vec![
        grid[(center_row - dr) as usize][(center_col - dc) as usize],
        grid[center_row as usize][center_col as usize],
        grid[(center_row + dr) as usize][(center_col + dc) as usize],
    ];

    // Must be exactly MAS or SAM
    chars == vec!['M', 'A', 'S'] || chars == vec!['S', 'A', 'M']
}

fn count_x_mas(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut count = 0;

    // For each possible center point of the X
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            // Check if it's an 'A' (center of X)
            if grid[row as usize][col as usize] != 'A' {
                continue;
            }

            // Check both possible X configurations
            let directions = [
                ((1, 1), (1, -1)), // Check both diagonal directions from center
            ];

            'outer: for &(dir1, dir2) in &directions {
                // For each direction pair, try both orientations of MAS/SAM
                let checks = [
                    // Check dir1 as MAS and dir2 as MAS
                    (dir1, dir2),
                    // Check dir1 as SAM and dir2 as SAM
                    ((-dir1.0, -dir1.1), (-dir2.0, -dir2.1)),
                ];

                for &(d1, d2) in &checks {
                    if check_mas_from_center(grid, row as isize, col as isize, d1.0, d1.1)
                        && check_mas_from_center(grid, row as isize, col as isize, d2.0, d2.1)
                    {
                        count += 1;
                        continue 'outer; // Only count each X once
                    }
                }
            }
        }
    }

    count
}

fn read_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part2(input: &str) -> usize {
    let grid = read_input(input);
    count_x_mas(&grid)
}

fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part2(input), 9);
    }
}
