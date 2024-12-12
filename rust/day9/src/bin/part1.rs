use anyhow::Result;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy)]
struct Block {
    file_id: Option<u32>,
}

fn parse_input(input: &str) -> Result<Vec<u32>> {
    let mut numbers = Vec::with_capacity(input.len());
    for c in input.trim().chars() {
        numbers.push(
            c.to_digit(10)
                .ok_or_else(|| anyhow::anyhow!("Invalid digit '{}'", c))?,
        );
    }
    Ok(numbers)
}

fn calculate_total_size(numbers: &[u32]) -> usize {
    let mut total_size = 0;
    for chunk in numbers.chunks(2) {
        total_size += chunk[0] as usize;
        if let Some(&spaces) = chunk.get(1) {
            total_size += spaces as usize;
        }
    }
    total_size
}

fn create_initial_disk(numbers: &[u32], total_size: usize) -> (Vec<Block>, BTreeSet<usize>) {
    let mut disk = Vec::with_capacity(total_size);
    let mut free_spaces = BTreeSet::new();
    let mut current_pos = 0;

    for (file_id, chunk) in numbers.chunks(2).enumerate() {
        let file_size = chunk[0] as usize;
        for _ in 0..file_size {
            disk.push(Block {
                file_id: Some(file_id as u32),
            });
            current_pos += 1;
        }

        if let Some(&spaces) = chunk.get(1) {
            for i in 0..spaces as usize {
                disk.push(Block { file_id: None });
                free_spaces.insert(current_pos + i);
            }
            current_pos += spaces as usize;
        }
    }
    (disk, free_spaces)
}

fn optimize_disk_layout(disk: &mut Vec<Block>, free_spaces: &mut BTreeSet<usize>) {
    for i in (0..disk.len()).rev() {
        if let Some(file_id) = disk[i].file_id {
            if let Some(&free_pos) = free_spaces.range(..i).next() {
                disk[free_pos].file_id = Some(file_id);
                disk[i].file_id = None;
                free_spaces.remove(&free_pos);
                free_spaces.insert(i);
            }
        }
    }
}

fn calculate_score(disk: &[Block]) -> i64 {
    disk.iter()
        .enumerate()
        .filter_map(|(i, block)| block.file_id.map(|file_id| i as i64 * file_id as i64))
        .sum()
}

fn compact_disk(numbers: Vec<u32>) -> Result<i64> {
    let total_size = calculate_total_size(&numbers);
    let (mut disk, mut free_spaces) = create_initial_disk(&numbers, total_size);
    optimize_disk_layout(&mut disk, &mut free_spaces);
    Ok(calculate_score(&disk))
}

fn part1(input: &str) -> Result<i64> {
    let numbers = parse_input(input)?;
    Ok(compact_disk(numbers)?)
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
    fn test_compact_disk() {
        let input = "2333133121414131402";
        assert_eq!(part1(input).unwrap(), 1928);
    }
}
