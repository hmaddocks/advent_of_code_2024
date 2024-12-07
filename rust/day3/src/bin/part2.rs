use regex::Regex;

fn extract_and_sum_multiplications(input: &str) -> i32 {
    let regex = Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut enabled = true;
    let mut sum = 0;

    for cap in regex.captures_iter(input) {
        let full_match = &cap[1];
        match full_match {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if enabled => {
                // Must be a multiplication since it's the only other capture pattern
                let x = cap[2].parse::<i32>().unwrap();
                let y = cap[3].parse::<i32>().unwrap();
                sum += x * y;
            }
            _ => {}
        }
    }

    sum
}

fn part2(input: &str) -> i32 {
    extract_and_sum_multiplications(input)
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)do()?mul(8,5))";
        assert_eq!(part2(input), 48);
    }
}
