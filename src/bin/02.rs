#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
enum LevelType {
    Increasing,
    Decresing,
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let mut safe = 0;
    for report in reports {
        if is_report_safe(&report) {
            safe += 1;
        }
    }

    Some(safe)
}

fn is_report_safe(report: &[u32]) -> bool {
    use LevelType::{Decresing, Increasing};

    let level_type = if report[0] > report[1] {
        Decresing
    } else {
        Increasing
    };

    for i in 0..report.len() - 1 {
        let l1 = report[i];
        let l2 = report[i + 1];

        let diff = if l1 > l2 { l1 - l2 } else { l2 - l1 };

        if (level_type == Increasing && l1 > l2)
            || (level_type == Decresing && l1 < l2)
            || !(1..=3).contains(&diff)
        {
            return false;
        }
    }

    true
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let mut safe = 0;
    for report in reports {
        if is_report_safe(&report) {
            safe += 1;
            continue;
        }

        for i in 0..report.len() {
            let mut safer_report: Vec<u32> = report[0..i].to_vec();
            safer_report.append(&mut report[i + 1..].to_vec());
            if is_report_safe(&safer_report) {
                safe += 1;
                break;
            }
        }
    }

    Some(safe)
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut reports = Vec::new();

    for line in input.lines() {
        let levels: Vec<u32> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<u32>().ok())
            .collect();
        reports.push(levels);
    }

    reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
