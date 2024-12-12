use anyhow::Result;
use std::collections::HashMap;

fn parse_input(input: &str) -> Result<Vec<i64>> {
    let numbers: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()?;
    Ok(numbers)
}

fn blink_recursive(
    value: i64,
    remaining_blinks: usize,
    cache: &mut HashMap<(i64, usize), usize>,
) -> Result<usize> {
    if remaining_blinks == 0 {
        return Ok(1);
    }

    if let Some(&result) = cache.get(&(value, remaining_blinks)) {
        return Ok(result);
    }

    let result = if value == 0 {
        blink_recursive(1, remaining_blinks - 1, cache)?
    } else {
        let value_str = value.to_string();
        if value_str.len() % 2 == 0 {
            let mid = value_str.len() / 2;
            let first_half = value_str[..mid]
                .parse::<i64>()
                .map_err(|e| anyhow::anyhow!("Failed to parse first half: {}", e))?;
            let second_half = value_str[mid..]
                .parse::<i64>()
                .map_err(|e| anyhow::anyhow!("Failed to parse second half: {}", e))?;
            blink_recursive(first_half, remaining_blinks - 1, cache)?
                + blink_recursive(second_half, remaining_blinks - 1, cache)?
        } else {
            blink_recursive(2024 * value, remaining_blinks - 1, cache)?
        }
    };

    cache.insert((value, remaining_blinks), result);
    Ok(result)
}

fn blink(stones: &[i64], blinks: usize) -> Result<usize> {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|&x| blink_recursive(x, blinks, &mut cache))
        .try_fold(0, |acc, x| Ok(acc + x?))
}

fn part2(input: &str, blinks: usize) -> Result<usize> {
    let stones = parse_input(input)?;
    blink(&stones, blinks)
}

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");
    dbg!(part2(input, 75)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_part1() {
        let input = "0 1 10 99 999";
        assert_eq!(part2(input, 1).unwrap(), 7);
    }

    #[test]
    fn test_part2_part2() {
        let input = "125 17";
        assert_eq!(part2(input, 6).unwrap(), 22);
    }

    #[test]
    fn test_part2_part3() {
        let input = "8435 234 928434 14 0 7 92446 8992692";
        assert_eq!(part2(input, 25).unwrap(), 182081);
    }
}
