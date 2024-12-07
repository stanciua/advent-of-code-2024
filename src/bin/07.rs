#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Multiply,
    Concatenation,
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    use Operation::{Add, Multiply};
    let equations = parse_input(input);
    let operations = [Add, Multiply];
    Some(solve_equation(&equations, &operations))
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    use Operation::{Add, Concatenation, Multiply};
    let equations = parse_input(input);
    let operations = [Add, Multiply, Concatenation];
    Some(solve_equation(&equations, &operations))
}

fn solve_equation(equations: &[(u64, Vec<u64>)], operations: &[Operation]) -> u64 {
    use Operation::{Add, Concatenation, Multiply};
    let mut total = 0u64;
    for equation in equations {
        let combinations = std::iter::repeat(operations.iter())
            .take(equation.1.len() - 1)
            .multi_cartesian_product()
            .collect::<Vec<_>>();

        for combination in combinations {
            let mut result = equation.1[0];
            for (idx, n) in equation.1[1..].iter().enumerate() {
                match combination.get(idx) {
                    Some(Add) => {
                        result += n;
                    }
                    Some(Multiply) => {
                        result *= n;
                    }
                    Some(Concatenation) => {
                        let mut temp = *n;
                        let mut multiplier = 1;
                        while temp > 0 {
                            multiplier *= 10;
                            temp /= 10;
                        }
                        result = result * multiplier + *n;
                    }
                    _ => {
                        panic!("Unknown operation");
                    }
                }
            }
            if result == equation.0 {
                total += result;
                break;
            }
        }
    }

    total
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
