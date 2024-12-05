use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pages) = parse_input(input);
    let mut numbers = HashSet::new();
    rules.iter().for_each(|&(k, v)| {
        numbers.insert(k);
        numbers.insert(v);
    });
    let numbers: Vec<u32> = numbers.into_iter().collect();

    let mut sum = 0;

    let mut adjacency_list: HashMap<u32, HashSet<u32>> = HashMap::new();
    for num in numbers.into_iter() {
        adjacency_list.insert(num, HashSet::new());
    }

    if let Some(inorder) = topological_sort(rules.as_slice(), numbers.as_slice()) {
        let mut inorder_map = HashMap::new();
        inorder.iter().enumerate().for_each(|(idx, n)| {
            inorder_map.insert(n, idx);
        });
        for mut page in pages {
            let curr_page = page.clone();
            page.sort_by(|a, b| inorder_map[a].cmp(&inorder_map[b]));
            if page == curr_page {
                sum += page[page.len() / 2];
            }
        }
    } else {
        dbg!("ups");
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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

// Perform a topological sort
fn topological_sort(rules: &[(u32, u32)], numbers: &[u32]) -> Option<Vec<u32>> {
    // Build adjacency list and in-degree count
    let mut adjacency_list: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();

    // Initialize in-degree for all numbers
    for &num in numbers {
        in_degree.insert(num, 0);
        adjacency_list.insert(num, vec![]);
    }
    // Populate the adjacency list and in-degree counts
    for &(before, after) in rules {
        adjacency_list.entry(before).or_default().push(after);
        *in_degree.entry(after).or_default() += 1;
    }
    dbg!(&adjacency_list);

    // Start with nodes that have no incoming edges
    let mut queue: VecDeque<u32> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&num, _)| num)
        .collect();

    let mut sorted_order = vec![];

    while let Some(current) = queue.pop_front() {
        sorted_order.push(current);

        if let Some(neighbors) = adjacency_list.get(&current) {
            for &neighbor in neighbors {
                let entry = in_degree.entry(neighbor).or_default();
                *entry -= 1;
                if *entry == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    // Check if all nodes are sorted (no cycles)
    if sorted_order.len() == numbers.len() {
        Some(sorted_order)
    } else {
        None // Cycle detected
    }
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
        assert_eq!(result, None);
    }
}
