use anyhow::{Context, Result};
use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Node {
    result: i64,
    value: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    const fn new(
        result: i64,
        value: i64,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    ) -> Self {
        Self {
            result,
            value,
            left,
            right,
        }
    }

    const fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    fn can_reach_result(&self, current: i64) -> bool {
        if self.is_leaf() || current == self.result {
            return current == self.result;
        }

        let check_child = |child: &Node, current: i64| {
            child.can_reach_result(current + child.value)
                || child.can_reach_result(current * child.value)
        };

        match (&self.left, &self.right) {
            (Some(left), Some(right)) => check_child(left, current) || check_child(right, current),
            (Some(child), None) | (None, Some(child)) => check_child(child, current),
            (None, None) => false,
        }
    }

    fn build_tree(result: i64, numbers: &[i64]) -> Option<Box<Self>> {
        match numbers {
            [] => None,
            [single] => Some(Box::new(Self::new(result, *single, None, None))),
            [first, rest @ ..] => {
                let subtree = Self::build_tree(result, rest);
                Some(Box::new(Self::new(
                    result,
                    *first,
                    subtree.clone(),
                    subtree,
                )))
            }
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<(i64, Vec<i64>)>> {
    input
        .lines()
        .map(|line| {
            let (result, numbers) = line
                .trim()
                .split_once(": ")
                .context("Invalid line format")?;

            let result = result.parse().context("Invalid result")?;
            let numbers = numbers
                .split_whitespace()
                .map(str::parse)
                .collect::<std::result::Result<_, _>>()
                .context("Invalid numbers")?;

            Ok((result, numbers))
        })
        .collect()
}

fn evaluate_line(result: i64, numbers: &[i64]) -> i64 {
    Node::build_tree(result, numbers)
        .filter(|tree| tree.can_reach_result(tree.value))
        .map_or(0, |_| result)
}

fn part1(input: &str) -> Result<i64> {
    let parsed = parse_input(input)?;
    Ok(parsed
        .par_iter()
        .map(|(result, numbers)| evaluate_line(*result, numbers))
        .sum())
}

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");
    println!("Result: {}", part1(input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_numbers_ok() {
        let input = "190: 10 19";
        assert_eq!(part1(input).unwrap(), 190);
    }

    #[test]
    fn test_two_numbers_not_ok() {
        let input = "83: 17 5";
        assert_eq!(part1(input).unwrap(), 0);
    }

    #[test]
    fn test_three_numbers_ok() {
        let input = "3267: 81 40 27";
        assert_eq!(part1(input).unwrap(), 3267);
    }

    #[test]
    fn test_three_numbers_not_ok() {
        let input = "161011: 16 10 13";
        assert_eq!(part1(input).unwrap(), 0);
    }

    #[test]
    fn test_four_numbers_ok() {
        let input = "292: 11 6 16 20";
        assert_eq!(part1(input).unwrap(), 292);
    }

    #[test]
    fn test_four_numbers_not_ok() {
        let input = "7290: 6 8 6 15";
        assert_eq!(part1(input).unwrap(), 0);
    }

    #[test]
    fn test_solve_puzzle() {
        let input = "\
            190: 10 19\n\
            3267: 81 40 27\n\
            83: 17 5\n\
            156: 15 6\n\
            7290: 6 8 6 15\n\
            161011: 16 10 13\n\
            192: 17 8 14\n\
            21037: 9 7 18 13\n\
            292: 11 6 16 20\n\
        ";
        assert_eq!(part1(input).unwrap(), 3749);
    }

    #[test]
    fn test_invalid_input() {
        let input = "not a number: 10 19";
        assert!(part1(input).is_err());
    }
}