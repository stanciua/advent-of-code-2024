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
    // Populate the adjacency list and in-degree counts
    for (before, after) in rules {
        adjacency_list.entry(before).or_default().insert(after);
    }

    'outer: for page in pages {
        for (idx, n) in page.iter().enumerate() {
            if !(idx + 1..page.len())
                .into_iter()
                .all(|idx| adjacency_list[&page[idx]].get(&n).is_none())
            {
                continue 'outer;
            }
        }
        sum += page[page.len() / 2];
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
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
    // Populate the adjacency list and in-degree counts
    for (before, after) in rules {
        adjacency_list.entry(before).or_default().insert(after);
    }

    let mut not_in_order = Vec::new();
    'outer: for page in pages {
        for (idx, n) in page.iter().enumerate() {
            if !(idx + 1..page.len())
                .into_iter()
                .all(|idx| adjacency_list[&page[idx]].get(&n).is_none())
            {
                not_in_order.push(page);
                continue 'outer;
            }
        }
    }

    for page in not_in_order {
        let mut ordered_page = Vec::new();
        let mut deque = VecDeque::from_iter(page.iter().cloned());
        while !deque.is_empty() {
            if let Some(n) = deque.pop_front() {
                if deque
                    .iter()
                    .cloned()
                    .all(|r| adjacency_list[&r].get(&n).is_none())
                {
                    ordered_page.push(n);
                } else {
                    deque.push_back(n);
                }
            }
        }
        sum += ordered_page[ordered_page.len() / 2];
    }

    Some(sum)
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
