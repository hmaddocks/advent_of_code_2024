use anyhow::{Context, Result};
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

fn find_middle(sequence: &[i32]) -> Result<i32> {
    if sequence.is_empty() {
        anyhow::bail!("Cannot find middle of empty sequence");
    }
    Ok(sequence[sequence.len() / 2])
}

fn parse_input(input: &str) -> Result<(Vec<(i32, i32)>, Vec<Vec<i32>>)> {
    let (rules_section, updates_section) = input
        .split_once("\n\n")
        .context("Invalid input format: missing section separator")?;

    let rules = rules_section
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(line_num, line)| {
            let (from, to) = line.split_once('|').with_context(|| {
                format!(
                    "Invalid rule format at line {}: missing separator",
                    line_num + 1
                )
            })?;

            let from = from.trim().parse().with_context(|| {
                format!(
                    "Invalid 'from' number '{}' at line {}",
                    from.trim(),
                    line_num + 1
                )
            })?;
            let to = to.trim().parse().with_context(|| {
                format!(
                    "Invalid 'to' number '{}' at line {}",
                    to.trim(),
                    line_num + 1
                )
            })?;

            Ok((from, to))
        })
        .collect::<Result<Vec<_>>>()?;

    let updates = updates_section
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(line_num, line)| {
            line.split(',')
                .enumerate()
                .map(|(num_idx, num)| {
                    num.trim().parse::<i32>().with_context(|| {
                        format!(
                            "Invalid number '{}' at position {} in update line {}",
                            num.trim(),
                            num_idx + 1,
                            line_num + 1
                        )
                    })
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((rules, updates))
}

fn part2(input: &str) -> Result<i32> {
    let (rules, updates) = parse_input(input)?;
    let mut graph = Graph::new(&rules);

    let invalid_updates: Vec<_> = updates
        .iter()
        .filter(|update| !graph.is_valid_sequence(update))
        .collect();

    let mut sum = 0;
    for update in invalid_updates {
        let sorted = sort_sequence_by_rules(update, &graph);
        sum += find_middle(&sorted)?;
    }
    Ok(sum)
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

        assert_eq!(part2(input).unwrap(), 123);
    }
}
