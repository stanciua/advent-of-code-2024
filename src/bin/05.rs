#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(5);

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pages) = parse_input(input);

    let adjacency_list = get_adjacency_list(&rules);

    let mut sum = 0;
    'outer: for page in pages {
        for (idx, n) in page.iter().enumerate() {
            if !(idx + 1..page.len()).all(|idx| !adjacency_list[&page[idx]].contains(n)) {
                continue 'outer;
            }
        }
        sum += page[page.len() / 2];
    }

    Some(sum)
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let (rules, pages) = parse_input(input);

    let adjacency_list = get_adjacency_list(&rules);

    let mut sum = 0;
    'outer: for page in pages {
        for (idx, n) in page.iter().enumerate() {
            if !(idx + 1..page.len()).all(|idx| !adjacency_list[&page[idx]].contains(n)) {
                // page is not in order, reorder it
                let mut ordered_page = Vec::new();
                let mut deque = page.iter().copied().collect::<VecDeque<_>>();
                while !deque.is_empty() {
                    if let Some(n) = deque.pop_front() {
                        if deque
                            .iter()
                            .copied()
                            .all(|r| !adjacency_list[&r].contains(&n))
                        {
                            ordered_page.push(n);
                        } else {
                            deque.push_back(n);
                        }
                    }
                }
                sum += ordered_page[ordered_page.len() / 2];
                continue 'outer;
            }
        }
    }

    Some(sum)
}

fn get_adjacency_list(rules: &[(u32, u32)]) -> HashMap<u32, HashSet<u32>> {
    let mut numbers = HashSet::new();
    for &(k, v) in rules {
        numbers.insert(k);
        numbers.insert(v);
    }

    let mut adjacency_list: HashMap<u32, HashSet<u32>> = HashMap::new();
    for num in numbers {
        adjacency_list.insert(num, HashSet::new());
    }
    // Populate the adjacency list and in-degree counts
    for &(before, after) in rules {
        adjacency_list.entry(before).or_default().insert(after);
    }

    adjacency_list
}

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut rules = Vec::new();
    let mut pages = Vec::new();
    let mut pages_flag = false;

    for line in input.lines() {
        if let Some((p1, p2)) = line.split_once('|') {
            if let (Ok(pn1), Ok(pn2)) = (p1.parse::<u32>(), p2.parse::<u32>()) {
                rules.push((pn1, pn2));
            }
            continue;
        }
        if line.is_empty() {
            pages_flag = true;
            continue;
        }

        if pages_flag {
            let update = line
                .split(',')
                .filter_map(|p| p.parse::<u32>().ok())
                .collect::<Vec<_>>();
            pages.push(update);
        }
    }

    (rules, pages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
