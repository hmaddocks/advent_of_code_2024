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

fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return false;
    }

    let diffs: Vec<_> = levels.windows(2).map(|w| w[1] - w[0]).collect();

    if diffs.iter().any(|&d| d.abs() < 1 || d.abs() > 3) {
        return false;
    }

    diffs.iter().all(|&d| d > 0) || diffs.iter().all(|&d| d < 0)
}

fn part1(input: &str) -> Result<usize> {
    let reports = parse_input(input)?;
    Ok(count_safe_reports(&reports))
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
    fn test_part1() {
        let input = "    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
    ";

        assert_eq!(part1(input).unwrap(), 2);
    }

    #[test]
    fn test_is_safe() {
        // Test too short sequence
        assert!(!is_safe(&[1]));
        assert!(!is_safe(&[]));

        // Test valid increasing sequences
        assert!(is_safe(&[1, 2])); // Minimal increasing
        assert!(is_safe(&[1, 3])); // Max allowed increase
        assert!(is_safe(&[1, 2, 4, 6])); // Longer increasing

        // Test valid decreasing sequences
        assert!(is_safe(&[6, 4])); // Minimal decreasing
        assert!(is_safe(&[10, 8, 6, 4])); // Longer decreasing

        // Test invalid sequences
        assert!(!is_safe(&[1, 5])); // Difference > 3
        assert!(!is_safe(&[1, 1])); // No difference
        assert!(!is_safe(&[1, 2, 1])); // Mixed increasing/decreasing
        assert!(!is_safe(&[3, 1, 4])); // Mixed decreasing/increasing

        // Test edge cases
        assert!(is_safe(&[1, 2, 3])); // All differences = 1
        assert!(is_safe(&[1, 4, 7])); // All differences = 3
        assert!(is_safe(&[7, 4, 1])); // All differences = -3
    }
}
