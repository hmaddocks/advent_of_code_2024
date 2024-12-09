use anyhow::{Context, Result};

fn parse_line(line: &str) -> Result<(i32, i32)> {
    let parts: Vec<_> = line.trim().split_whitespace().collect();
    if parts.len() != 2 {
        anyhow::bail!("invalid number of elements in line");
    }
    let left = parts[0]
        .parse::<i32>()
        .with_context(|| format!("failed to parse left number: {}", parts[0]))?;
    let right = parts[1]
        .parse::<i32>()
        .with_context(|| format!("failed to parse right number: {}", parts[1]))?;
    Ok((left, right))
}

fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for (line_num, line) in input.lines().enumerate() {
        let (left, right) =
            parse_line(line).with_context(|| format!("failed to parse line {}", line_num + 1))?;
        left_list.push(left);
        right_list.push(right);
    }

    Ok((left_list, right_list))
}

fn part1(input: &str) -> Result<i32> {
    let (mut left_list, mut right_list) = parse_input(input.trim())?;

    left_list.sort();
    right_list.sort();

    Ok(left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| (l - r).abs())
        .sum())
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
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(part1(input).unwrap(), 11);
    }
}
