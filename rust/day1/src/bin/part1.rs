fn parse_line(line: &str) -> (i32, i32) {
    let parts: Vec<_> = line.trim().split_whitespace().collect();
    let left = parts[0].parse::<i32>().unwrap();
    let right = parts[1].parse::<i32>().unwrap();
    (left, right)
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let (left, right) = parse_line(line);
        left_list.push(left);
        right_list.push(right);
    }

    (left_list, right_list)
}

fn part1(input: &str) -> i32 {
    let (mut left_list, mut right_list) = parse_input(input.trim());

    left_list.sort();
    right_list.sort();

    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(part1(input));
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

        assert_eq!(part1(input), 11);
    }
}
