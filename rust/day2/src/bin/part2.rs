use anyhow::{Context, Result};

fn parse_input(input: &str) -> Result<Vec<Vec<i32>>> {
    input
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            line.split_whitespace()
                .enumerate()
                .map(|(num_idx, num)| {
                    num.parse().with_context(|| {
                        format!(
                            "Failed to parse number '{}' at position {} on line {}",
                            num,
                            num_idx + 1,
                            line_num + 1
                        )
                    })
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()
}

fn count_safe_reports(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

fn is_safe_sequence(level: &[i32]) -> bool {
    if level.len() < 2 {
        return false;
    }

    let diffs: Vec<_> = level.windows(2).map(|w| w[1] - w[0]).collect();

    if diffs.iter().any(|&d| d.abs() < 1 || d.abs() > 3) {
        return false;
    }

    diffs.iter().all(|&d| d > 0) || diffs.iter().all(|&d| d < 0)
}

fn is_safe(level: &[i32]) -> bool {
    // Check if sequence is safe without removing any level
    if is_safe_sequence(level) {
        return true;
    }

    // Try removing one level at a time and check if remaining sequence is safe
    for i in 0..level.len() {
        let mut modified = Vec::from(level);
        modified.remove(i);
        if is_safe_sequence(&modified) {
            return true;
        }
    }

    false
}

fn part2(input: &str) -> Result<usize> {
    let reports = parse_input(input)?;
    Ok(count_safe_reports(&reports))
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
    fn test_part2() {
        let input = "    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
    ";

        assert_eq!(part2(input).unwrap(), 4);
    }
}
