use anyhow::Result;

fn parse_input(input: &str) -> Result<Vec<i64>> {
    let numbers: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()?;
    Ok(numbers)
}

fn process_stone(stone: i64, blinks: usize) -> Vec<i64> {
    if blinks == 0 {
        return vec![stone];
    }

    let mut result = Vec::new();
    if stone == 0 {
        result.extend(process_stone(1, blinks - 1));
    } else {
        let digits: Vec<char> = stone.abs().to_string().chars().collect();
        if digits.len() % 2 == 0 {
            let mid = digits.len() / 2;
            let left: String = digits[..mid].iter().collect();
            let right: String = digits[mid..].iter().collect();
            result.extend(process_stone(left.parse::<i64>().unwrap(), blinks - 1));
            result.extend(process_stone(right.parse::<i64>().unwrap(), blinks - 1));
        } else {
            result.extend(process_stone(stone * 2024, blinks - 1));
        }
    }
    result
}

fn blink(stones: Vec<i64>, blinks: usize) -> Result<usize> {
    let mut result = Vec::new();
    for stone in stones {
        result.extend(process_stone(stone, blinks));
    }
    Ok(result.len())
}

fn part1(input: &str, blinks: usize) -> Result<usize> {
    let stones = parse_input(input)?;
    Ok(blink(stones, blinks)?)
}

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");
    dbg!(part1(input, 25)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_part1() {
        let input = "0 1 10 99 999";
        assert_eq!(part1(input, 1).unwrap(), 7);
    }

    #[test]
    fn test_part1_part2() {
        let input = "125 17";
        assert_eq!(part1(input, 6).unwrap(), 22);
    }

    #[test]
    fn test_part1_part3() {
        let input = "8435 234 928434 14 0 7 92446 8992692";
        assert_eq!(part1(input, 25).unwrap(), 182081);
    }
}
