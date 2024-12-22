#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
use std::collections::HashMap;
use std::string::ToString;

advent_of_code::solution!(11);

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let stones = input
        .split_whitespace()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    blinking(&stones, 25)
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    let stones = input
        .split_whitespace()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    blinking(&stones, 75)
}

fn blinking(stones: &[String], times: usize) -> Option<usize> {
    let mut init_blinking_stones = HashMap::new();
    for v in stones {
        init_blinking_stones.insert(v.clone(), 1usize);
    }
    for _ in 0..times {
        let mut blinking_stones = HashMap::new();
        for (stone, &times) in &init_blinking_stones {
            let mut num = stone.parse::<u64>().ok()?;
            // all numbers 0's will be converted to 1's
            if num == 0 {
                blinking_stones
                    .entry("1".to_owned())
                    .and_modify(|s| *s += times)
                    .or_insert(times);
            } else if stone.len() % 2 == 0 {
                // if length is even, split the number in 2, taking into account
                // edge cases
                let (num1, num2) = stone.split_at(stone.len() / 2);
                let (num1, num2) = (num1.trim_start_matches('0'), num2.trim_start_matches('0'));
                for val in <[&str; 2]>::from((num1, num2)) {
                    let n = if val.is_empty() {
                        "0".to_owned()
                    } else {
                        val.to_owned()
                    };
                    blinking_stones
                        .entry(n)
                        .and_modify(|s| *s += times)
                        .or_insert(times);
                }
            } else {
                // else no special case, just multiply the number by 2024
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
        assert_eq!(result, Some(65_601_038_650_482));
    }
}
