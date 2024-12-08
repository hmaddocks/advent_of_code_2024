use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Graph {
    edges: HashMap<i32, HashSet<i32>>,
    positions_cache: HashMap<i32, usize>,
}

impl Graph {
    fn new(rules: &[(i32, i32)]) -> Self {
        let edges = rules.iter().map(|&(from, to)| (from, to)).fold(
            HashMap::new(),
            |mut acc, (from, to)| {
                acc.entry(from).or_insert_with(HashSet::new).insert(to);
                acc
            },
        );

        Self {
            edges,
            positions_cache: HashMap::with_capacity(32),
        }
    }

    fn is_valid_sequence(&mut self, sequence: &[i32]) -> bool {
        self.positions_cache.clear();
        self.positions_cache.extend(
            sequence
                .iter()
                .copied()
                .enumerate()
                .map(|(i, val)| (val, i)),
        );

        sequence.iter().copied().all(|page| {
            self.edges.get(&page).map_or(true, |deps| {
                deps.iter().all(|&dep| {
                    self.positions_cache
                        .get(&dep)
                        .map_or(true, |&dep_pos| self.positions_cache[&page] <= dep_pos)
                })
            })
        })
    }
}

fn find_middle(sequence: &[i32]) -> i32 {
    sequence[sequence.len() / 2]
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let (rules_section, updates_section) = input
        .split_once("\n\n")
        .expect("Invalid input format: missing section separator");

    let rules = rules_section
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (from, to) = line
                .split_once('|')
                .expect("Invalid rule format: missing separator");
            (
                from.trim().parse().expect("Invalid number"),
                to.trim().parse().expect("Invalid number"),
            )
        })
        .collect::<Vec<_>>();

    let updates = updates_section
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|num| num.trim().parse::<i32>().expect("Invalid number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (rules, updates)
}

fn part1(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);
    let mut graph = Graph::new(&rules);

    updates
        .iter()
        .filter(|update| graph.is_valid_sequence(update))
        .map(|update| find_middle(update.as_slice()))
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
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

        assert_eq!(part1(input), 143);
    }
}
