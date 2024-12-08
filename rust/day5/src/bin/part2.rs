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

fn sort_sequence_by_rules(sequence: &[i32], graph: &Graph) -> Vec<i32> {
    let mut sorted = sequence.to_vec();
    sorted.sort_by(|&a, &b| {
        graph
            .edges
            .get(&a)
            .and_then(|deps| deps.contains(&b).then_some(std::cmp::Ordering::Greater))
            .or_else(|| {
                graph
                    .edges
                    .get(&b)
                    .and_then(|deps| deps.contains(&a).then_some(std::cmp::Ordering::Less))
            })
            .unwrap_or_else(|| b.cmp(&a))
    });
    sorted
}

fn find_middle(sequence: &[i32]) -> i32 {
    sequence[sequence.len() / 2]
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let (rules_section, updates_section) =
        input.split_once("\n\n").expect("Missing section separator");

    let rules = rules_section
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (from, to) = line.split_once('|').expect("Invalid rule format");
            (
                from.trim().parse().expect("Invalid 'from' number"),
                to.trim().parse().expect("Invalid 'to' number"),
            )
        })
        .collect();

    let updates = updates_section
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|num| num.trim().parse().expect("Invalid number in sequence"))
                .collect()
        })
        .collect();

    (rules, updates)
}

fn part2(input: &str) -> i32 {
    let (rules, updates) = parse_input(input);
    let mut graph = Graph::new(&rules);

    let invalid_updates: Vec<_> = updates
        .iter()
        .filter(|update| !graph.is_valid_sequence(update))
        .collect();

    invalid_updates
        .iter()
        .map(|update| {
            let sorted = sort_sequence_by_rules(update, &graph);
            find_middle(&sorted)
        })
        .sum()
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

        assert_eq!(part2(input), 123);
    }
}
