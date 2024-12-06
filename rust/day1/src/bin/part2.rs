fn parse_line(line: &str) -> (u32, u32) {
    let parts: Vec<_> = line.trim().split_whitespace().collect();
    let left = parts[0].parse::<u32>().unwrap();
    let right = parts[1].parse::<u32>().unwrap();
    (left, right)
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let (left, right) = parse_line(line);
        left_list.push(left);
        right_list.push(right);
    }

    (left_list, right_list)
}

fn part2(input: &str) -> u32 {
    let (left_list, right_list) = parse_input(input);

    let mut right_map: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();

    for right in right_list {
        *right_map.entry(right).or_insert(0) += 1;
    }

    let mut count: u32 = 0;
    for left in left_list {
        if right_map.contains_key(&left) {
            count += left * right_map.get(&left).unwrap();
        }
    }

    count
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(part2(input));
}

#[test]
fn test_part2() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(part2(input), 31);
}
