use std::collections::{BTreeMap, HashMap};

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Multiply,
    Concatenation,
}

pub fn part_one(input: &str) -> Option<u64> {
    use Operation::*;
    let equations = parse_input(input);
    let operations = [Add, Multiply];
    let mut total = 0u64;
    for equation in equations {
        let combinations = std::iter::repeat(operations.iter())
            .take(equation.1.len() - 1)
            .multi_cartesian_product()
            .collect::<Vec<_>>();

        for combination in combinations {
            let mut result = equation.1[0];
            for (idx, n) in equation.1[1..].iter().enumerate() {
                match combination[idx] {
                    Add => {
                        result += n;
                    }
                    Multiply => {
                        result *= n;
                    }
                    _ => {
                        panic!("value not supported");
                    }
                }
            }
            if result == equation.0 {
                total += result;
                break;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    use Operation::*;
    let equations = parse_input(input);
    let operations = [Add, Multiply, Concatenation];
    let mut total = 0u64;
    for equation in equations {
        let combinations = std::iter::repeat(operations.iter())
            .take(equation.1.len() - 1)
            .multi_cartesian_product()
            .collect::<Vec<_>>();

        for combination in combinations {
            let mut result = equation.1[0];
            for (idx, n) in equation.1[1..].iter().enumerate() {
                match combination[idx] {
                    Add => {
                        result += n;
                    }
                    Multiply => {
                        result *= n;
                    }
                    Concatenation => {
                        let mut r = result.to_string();
                        r.push_str(&n.to_string());
                        result = r.parse::<u64>().unwrap_or_default();
                    }
                }
            }
            if result == equation.0 {
                total += result;
                break;
            }
        }
    }
    Some(total)
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    let mut equations = Vec::new();
    for line in input.lines() {
        if let Some((result, calibrations)) = line.split_once(':') {
            let calibrations: Vec<u64> = calibrations
                .split_whitespace()
                .filter_map(|n| n.parse::<u64>().ok())
                .collect();
            if let Ok(result) = result.parse::<u64>() {
                equations.push((result, calibrations));
            }
        }
    }

    equations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
