use anyhow::{Context, Result};
use regex::Regex;

fn extract_and_sum_multiplications(input: &str) -> Result<i32> {
    let regex = Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))")
        .context("Failed to compile regex pattern")?;
    let mut enabled = true;
    let mut sum = 0;

    for (idx, cap) in regex.captures_iter(input).enumerate() {
        let full_match = &cap[1];
        match full_match {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if enabled => {
                // Must be a multiplication since it's the only other capture pattern
                let num1 = cap[2].parse::<i32>().with_context(|| {
                    format!(
                        "Failed to parse first number '{}' at match {}",
                        &cap[2],
                        idx + 1
                    )
                })?;
                let num2 = cap[3].parse::<i32>().with_context(|| {
                    format!(
                        "Failed to parse second number '{}' at match {}",
                        &cap[3],
                        idx + 1
                    )
                })?;
                sum += num1 * num2;
            }
            _ => {}
        }
    }

    Ok(sum)
}

fn part2(input: &str) -> Result<i32> {
    extract_and_sum_multiplications(input)
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
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)do()?mul(8,5))";
        assert_eq!(part2(input).unwrap(), 48);
    }
}
