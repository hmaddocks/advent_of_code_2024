use anyhow::{Context, Result};

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn parse_moves(moves: &str) -> Vec<(i32, i32)> {
    moves
        .trim()
        .replace('\n', "")
        .chars()
        .map(|char| match char {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("Invalid move character: {}", char),
        })
        .collect()
}

fn find_start(map: &[Vec<char>]) -> Option<(usize, usize)> {
    map.iter().enumerate().find_map(|(y, row)| {
        row.iter()
            .enumerate()
            .find(|&(_, &cell)| cell == '@')
            .map(|(x, _)| (x, y))
    })
}

/// Move the robot and objects according to the given moves
fn move_robot(
    mut map: Vec<Vec<char>>,
    mut start: (usize, usize),
    moves: &[(i32, i32)],
) -> Vec<Vec<char>> {
    for &(dx, dy) in moves {
        let current_pos = start;
        let new_pos = (
            (current_pos.0 as i32 + dx) as usize,
            (current_pos.1 as i32 + dy) as usize,
        );

        // Skip if new position is blocked
        if map[new_pos.1][new_pos.0] == '#' {
            continue;
        }

        // Collect and move objects in the path
        let mut objects_to_move = Vec::new();
        let mut check_pos = new_pos;

        // Collect objects in the chain
        while map[check_pos.1][check_pos.0] == 'O' {
            objects_to_move.push(check_pos);
            check_pos = (
                (check_pos.0 as i32 + dx) as usize,
                (check_pos.1 as i32 + dy) as usize,
            );
        }

        // Skip if the end of the chain is blocked
        if map[check_pos.1][check_pos.0] == '#' {
            continue;
        }

        // Move objects
        for obj_pos in objects_to_move.iter().rev() {
            map[obj_pos.1][obj_pos.0] = '.';
            let new_obj_pos = (
                (obj_pos.0 as i32 + dx) as usize,
                (obj_pos.1 as i32 + dy) as usize,
            );
            map[new_obj_pos.1][new_obj_pos.0] = 'O';
        }

        // Update start position for the next iteration
        start = (
            (start.0 as i32 + dx) as usize,
            (start.1 as i32 + dy) as usize,
        );
    }
    map
}

/// Find all box positions in the map
fn find_box_positions(map: &[Vec<char>]) -> Vec<(usize, usize)> {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, &cell)| if cell == 'O' { Some((x, y)) } else { None })
        })
        .collect()
}

/// Solve part 1 of the puzzle
fn part1(input: &str) -> Result<usize> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let map = parse_map(parts.first().context("Invalid input: no map")?);
    let moves = parse_moves(parts.get(1).context("Invalid input: no moves")?);
    let start = find_start(&map).context("No start position found")?;

    let final_map = move_robot(map, start, &moves);
    let box_positions = find_box_positions(&final_map);

    Ok(box_positions.iter().map(|&(x, y)| y * 100 + x).sum())
}

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");
    println!("Result: {}", part1(input)?);
    Ok(())
}
