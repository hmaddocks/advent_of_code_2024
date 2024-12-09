use anyhow::{Context, Result};

fn parse_line(line: &str) -> Result<(u32, u32)> {
    let parts: Vec<_> = line.trim().split_whitespace().collect();
    if parts.len() != 2 {
        anyhow::bail!(
            "Invalid line format: expected 2 numbers, got {} parts",
            parts.len()
        );
    }
    let left = parts[0]
        .parse::<u32>()
        .with_context(|| format!("Failed to parse left number: {}", parts[0]))?;
    let right = parts[1]
        .parse::<u32>()
        .with_context(|| format!("Failed to parse right number: {}", parts[1]))?;
    Ok((left, right))
}

fn parse_input(input: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for (line_num, line) in input.lines().enumerate() {
        let (left, right) =
            parse_line(line).with_context(|| format!("Error parsing line {}", line_num + 1))?;
        left_list.push(left);
        right_list.push(right);
    }

    Ok((left_list, right_list))
}

fn part2(input: &str) -> Result<u32> {
    let (left_list, right_list) = parse_input(input)?;

    let mut right_map: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();

    for right in right_list {
        *right_map.entry(right).or_insert(0) += 1;
    }

    let mut count: u32 = 0;
    for left in left_list {
        if let Some(&right_count) = right_map.get(&left) {
            count += left * right_count;
        }
    }

    Ok(count)
}

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");
    dbg!(part2(input)?);
    Ok(())
}

#[test]
fn test_part2() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(part2(input).unwrap(), 31);
}
