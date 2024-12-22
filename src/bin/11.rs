use std::collections::{HashMap, HashSet, LinkedList, VecDeque};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let stones = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    blinking(&stones, 25)
}

pub fn part_two(input: &str) -> Option<usize> {
    let stones = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    blinking(&stones, 75)
}

fn blinking(stones: &[String], times: usize) -> Option<usize> {
    let mut init_blinking_stones = HashMap::new();
    stones.iter().for_each(|v| {
        init_blinking_stones.insert(v.clone(), 1usize);
    });
    for i in 0..times {
        let mut blinking_stones = HashMap::new();
        for (stone, &times) in &init_blinking_stones {
            let mut num = stone.parse::<u64>().ok()?;
            if num == 0 {
                blinking_stones
                    .entry("1".to_owned())
                    .and_modify(|s| *s += times)
                    .or_insert(times);
            } else if stone.len() % 2 == 0 {
                let (num1, num2) = stone.split_at(stone.len() / 2);
                let (num1, num2) = (num1.trim_start_matches('0'), num2.trim_start_matches('0'));
                if num1.is_empty() {
                    blinking_stones
                        .entry("0".to_owned())
                        .and_modify(|s| *s += times)
                        .or_insert(times);
                } else {
                    blinking_stones
                        .entry(num1.to_owned())
                        .and_modify(|s| *s += times)
                        .or_insert(times);
                }
                if num2.is_empty() {
                    blinking_stones
                        .entry("0".to_owned())
                        .and_modify(|s| *s += times)
                        .or_insert(times);
                } else {
                    blinking_stones
                        .entry(num2.to_owned())
                        .and_modify(|s| *s += times)
                        .or_insert(times);
                }
            } else {
                num *= 2024;
                blinking_stones
                    .entry(num.to_string())
                    .and_modify(|s| *s += times)
                    .or_insert(times);
            }
        }
        init_blinking_stones = blinking_stones;
    }

    Some(init_blinking_stones.values().sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
