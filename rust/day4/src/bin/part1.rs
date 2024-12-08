fn count_occurrences(grid: &Vec<Vec<char>>, word: &str) -> usize {
    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (0, -1),  // Left
        (-1, 0),  // Up
        (1, 1),   // Down-Right
        (1, -1),  // Down-Left
        (-1, 1),  // Up-Right
        (-1, -1), // Up-Left
    ];

    let mut count = 0;
    let word_len = word.len() as isize;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let word_chars: Vec<char> = word.chars().collect();

    for row in 0..rows {
        for col in 0..cols {
            for &(row_direction, col_direction) in &directions {
                let mut matched = true;
                for i in 0..word_len {
                    let nr = row + i * row_direction;
                    let nc = col + i * col_direction;
                    if nr < 0
                        || nr >= rows
                        || nc < 0
                        || nc >= cols
                        || grid[nr as usize][nc as usize] != word_chars[i as usize]
                    {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    count += 1;
                }
            }
        }
    }

    count
}

fn read_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> usize {
    let grid = read_input(input);
    count_occurrences(&grid, "XMAS")
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
        assert_eq!(part1(input), 18);
    }
}
