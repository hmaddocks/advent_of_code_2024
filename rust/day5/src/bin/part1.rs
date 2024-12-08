use std::collections::{HashMap, HashSet};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Default)]
struct Graph {
    edges: HashMap<i32, HashSet<i32>>,
    positions_cache: HashMap<i32, usize>,
}

impl Graph {
    fn new(rules: &[(i32, i32)]) -> Self {
        let edges = rules.iter().fold(HashMap::new(), |mut graph, &(from, to)| {
            graph.entry(from).or_insert_with(HashSet::new).insert(to);
            graph
        });
        Self {
            edges,
            positions_cache: HashMap::with_capacity(32),
        }
    }

    fn is_valid_sequence(&mut self, sequence: &[i32]) -> bool {
        self.positions_cache.clear();

        for (i, &val) in sequence.iter().enumerate() {
            self.positions_cache.insert(val, i);
        }

        sequence.iter().all(|&page| {
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

fn parse_input(input: &str) -> Result<(Vec<(i32, i32)>, Vec<Vec<i32>>)> {
    let mut parts = input.split("\n\n");

    // Parse rules section
    let rules_section = parts.next().ok_or("Missing rules section")?;
    let mut rules = Vec::with_capacity(rules_section.lines().count());

    for line in rules_section.lines() {
        if line.is_empty() {
            continue;
        }

        let mut iter = line.split('|');
        let from = iter
            .next()
            .ok_or("Missing 'from' value")?
            .trim()
            .parse::<i32>()?;
        let to = iter
            .next()
            .ok_or("Missing 'to' value")?
            .trim()
            .parse::<i32>()?;
        rules.push((from, to));
    }

    let updates_section = parts.next().ok_or("Missing updates section")?;
    let mut updates = Vec::with_capacity(updates_section.lines().count());

    for line in updates_section.lines() {
        if line.is_empty() {
            continue;
        }

        let mut numbers = Vec::with_capacity(8);
        for num_str in line.split(',') {
            numbers.push(num_str.trim().parse::<i32>()?);
        }
        updates.push(numbers);
    }

    Ok((rules, updates))
}

fn part1(input: &str) -> Result<i32> {
    let (rules, updates) = parse_input(input)?;
    let mut graph = Graph::new(&rules);

    Ok(updates
        .iter()
        .filter(|update| graph.is_valid_sequence(update))
        .map(|update| find_middle(update))
        .sum())
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(part1(input).unwrap());
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

        assert_eq!(part1(input).unwrap(), 143);
    }
}
