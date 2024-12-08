use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn move_forward(self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Self::Up => (pos.0 - 1, pos.1),
            Self::Right => (pos.0, pos.1 + 1),
            Self::Down => (pos.0 + 1, pos.1),
            Self::Left => (pos.0, pos.1 - 1),
        }
    }
}

type Grid = Vec<Vec<char>>;
type Position = (i32, i32);

struct GridState {
    grid: Grid,
    rows: i32,
    cols: i32,
    start_pos: Position,
    start_dir: Direction,
}

impl GridState {
    fn from_input(input: &str) -> Self {
        let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;

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

        Self {
            grid,
            rows,
            cols,
            start_pos,
            start_dir: Direction::Up,
        }
    }

    fn is_valid_pos(&self, pos: Position) -> bool {
        pos.0 >= 0 && pos.0 < self.rows && pos.1 >= 0 && pos.1 < self.cols
    }

    fn get_cell(&self, pos: Position) -> char {
        self.grid[pos.0 as usize][pos.1 as usize]
    }

    fn set_cell(&mut self, pos: Position, value: char) {
        self.grid[pos.0 as usize][pos.1 as usize] = value;
    }

    fn should_skip_pos(&self, pos: Position) -> bool {
        pos == self.start_pos || self.get_cell(pos) == '#' || self.get_cell(pos) == '^'
    }
}

fn has_loop(state: &GridState, visited: &mut HashSet<(Position, Direction)>) -> bool {
    let mut pos = state.start_pos;
    let mut dir = state.start_dir;
    visited.clear();
    visited.insert((pos, dir));

    loop {
        let next_pos = dir.move_forward(pos);
        if !state.is_valid_pos(next_pos) {
            return false;
        }

        match state.get_cell(next_pos) {
            '#' => {
                dir = dir.turn_right();
                if !visited.insert((pos, dir)) {
                    return true;
                }
            }
            _ => {
                pos = next_pos;
                if !visited.insert((pos, dir)) {
                    return true;
                }
            }
        }
    }
}

fn find_all_loops(state: &mut GridState) -> usize {
    let mut total_loops = 0;
    let mut visited = HashSet::with_capacity((state.rows * state.cols * 4) as usize);

    for i in 0..state.rows {
        for j in 0..state.cols {
            let pos = (i, j);
            if state.should_skip_pos(pos) {
                continue;
            }

            let original_char = state.get_cell(pos);
            state.set_cell(pos, '#');

            if has_loop(state, &mut visited) {
                total_loops += 1;
            }

            state.set_cell(pos, original_char);
        }
    }

    total_loops
}

fn part2(input: &str) -> usize {
    let mut state = GridState::from_input(input);
    find_all_loops(&mut state)
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
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

        assert_eq!(part2(input), 6);
    }
}
