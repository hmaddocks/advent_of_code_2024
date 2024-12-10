use anyhow::{Context, Result};
use regex::Regex;

fn extract_and_sum_multiplications(input: &str) -> Result<i32> {
    let regex =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").context("Failed to compile regex pattern")?;

    Ok(regex
        .captures_iter(input)
        .map(|cap| {
            let num1 = cap[1]
                .parse::<i32>()
                .with_context(|| format!("Failed to parse first number: {}", &cap[1]))?;
            let num2 = cap[2]
                .parse::<i32>()
                .with_context(|| format!("Failed to parse second number: {}", &cap[2]))?;
            Ok(num1 * num2)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .sum())
}

fn part1(input: &str) -> Result<i32> {
    extract_and_sum_multiplications(input)
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
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input).unwrap(), 161);
    }
}
