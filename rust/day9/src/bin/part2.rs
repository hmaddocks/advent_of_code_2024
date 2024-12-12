use anyhow::Result;

fn parse_input(input: &str) -> Result<Vec<i8>> {
    let numbers: Vec<i8> = input
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|d| d as i8)
                .ok_or_else(|| anyhow::anyhow!("Invalid digit '{}'", c))
        })
        .collect::<Result<_>>()?;

    Ok(numbers)
}

#[derive(Debug, Clone, Copy)]
struct File {
    id: u32,
    start: usize,
    size: usize,
}

fn create_initial_disk(numbers: &[i8]) -> Vec<Option<u32>> {
    let disk_size = numbers.chunks(2).fold(0, |acc, chunk| {
        acc + chunk[0] as usize + chunk.get(1).map_or(0, |&x| x as usize)
    });

    let mut disk = Vec::with_capacity(disk_size);
    for (file_id, chunk) in numbers.chunks(2).enumerate() {
        disk.extend(std::iter::repeat(Some(file_id as u32)).take(chunk[0] as usize));
        if let Some(&spaces) = chunk.get(1) {
            disk.extend(std::iter::repeat(None).take(spaces as usize));
        }
    }
    disk
}

fn collect_files(disk: &[Option<u32>]) -> Vec<File> {
    let mut files = Vec::new();
    let mut current_file = None;
    let mut start_pos = 0;

    for (pos, &block) in disk.iter().enumerate() {
        match (current_file, block) {
            (None, Some(file_id)) => {
                current_file = Some(file_id);
                start_pos = pos;
            }
            (Some(current_id), Some(file_id)) if current_id != file_id => {
                files.push(File {
                    id: current_id,
                    start: start_pos,
                    size: pos - start_pos,
                });
                current_file = Some(file_id);
                start_pos = pos;
            }
            (Some(current_id), None) => {
                files.push(File {
                    id: current_id,
                    start: start_pos,
                    size: pos - start_pos,
                });
                current_file = None;
            }
            _ => {}
        }
    }

    if let Some(current_id) = current_file {
        files.push(File {
            id: current_id,
            start: start_pos,
            size: disk.len() - start_pos,
        });
    }

    files
}

fn find_leftmost_span(disk: &[Option<u32>], up_to: usize, size_needed: usize) -> Option<usize> {
    let mut current_span = 0;

    for (i, &block) in disk[..up_to].iter().enumerate() {
        if block.is_none() {
            current_span += 1;
            if current_span == size_needed {
                return Some(i - size_needed + 1);
            }
        } else {
            current_span = 0;
        }
    }
    None
}

fn calculate_score(disk: &[Option<u32>]) -> i64 {
    disk.iter()
        .enumerate()
        .filter_map(|(i, &block)| block.map(|file_id| i as i64 * file_id as i64))
        .sum()
}

fn compact_disk(numbers: Vec<i8>) -> Result<i64> {
    let mut disk = create_initial_disk(&numbers);
    let mut files = collect_files(&disk);
    files.sort_unstable_by_key(|file| std::cmp::Reverse(file.id));

    for file in files {
        if let Some(new_start) = find_leftmost_span(&disk, file.start, file.size) {
            disk[file.start..file.start + file.size].fill(None);
            disk[new_start..new_start + file.size].fill(Some(file.id));
        }
    }

    Ok(calculate_score(&disk))
}

fn part2(input: &str) -> Result<i64> {
    let numbers = parse_input(input)?;
    Ok(compact_disk(numbers)?)
}

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");
    dbg!(part2(input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_disk() {
        let input = "2333133121414131402";
        assert_eq!(part2(input).unwrap(), 2858);
    }
}
