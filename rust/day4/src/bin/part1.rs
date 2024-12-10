const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),   // Right
    (1, 0),   // Down
    (0, -1),  // Left
    (-1, 0),  // Up
    (1, 1),   // Down-Right
    (1, -1),  // Down-Left
    (-1, 1),  // Up-Right
    (-1, -1), // Up-Left
];

struct Grid {
    cells: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let rows = cells.len();
        let cols = cells.first().map_or(0, |row| row.len());
        Self { cells, rows, cols }
    }

    fn get(&self, row: i32, col: i32) -> Option<char> {
        if row >= 0 && (row as usize) < self.rows && col >= 0 && (col as usize) < self.cols {
            Some(self.cells[row as usize][col as usize])
        } else {
            None
        }
    }
}

fn word_at_position(row: i32, col: i32, direction: (i32, i32), word: &str, grid: &Grid) -> bool {
    word.chars().enumerate().all(|(i, expected)| {
        let new_row = row + (i as i32 * direction.0);
        let new_col = col + (i as i32 * direction.1);
        grid.get(new_row, new_col).map_or(false, |c| c == expected)
    })
}

fn count_xmas(grid: &Grid, word: &str) -> i32 {
    (0..grid.rows as i32)
        .flat_map(|row| (0..grid.cols as i32).map(move |col| (row, col)))
        .flat_map(|(row, col)| DIRECTIONS.iter().map(move |&dir| (row, col, dir)))
        .filter(|&(row, col, dir)| word_at_position(row, col, dir, word, grid))
        .count() as i32
}

fn part1(input: &str) -> i32 {
    let grid = Grid::new(input);
    count_xmas(&grid, "XMAS")
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
