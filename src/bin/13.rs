advent_of_code::solution!(13);

use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{anychar, char, digit1, newline, space1},
    combinator::{map_res, opt},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
struct Button {
    name: char,
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Prize {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Entry {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

fn parse_integer(input: &str) -> IResult<&str, i64> {
    map_res(digit1, |s: &str| i64::from_str(s))(input)
}

fn parse_button(input: &str) -> IResult<&str, Button> {
    let (input, name) = preceded(tag("Button "), anychar)(input)?;
    let (input, _) = tag(": X+")(input)?;
    let (input, x) = parse_integer(input)?;
    let (input, _) = tag(", Y+")(input)?;
    let (input, y) = parse_integer(input)?;
    Ok((input, Button { name, x, y }))
}

fn parse_prize(input: &str) -> IResult<&str, Prize> {
    let (input, _) = tag("Prize: X=")(input)?;
    let (input, x) = parse_integer(input)?;
    let (input, _) = tag(", Y=")(input)?;
    let (input, y) = parse_integer(input)?;
    Ok((input, Prize { x, y }))
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let (input, button_a) = parse_button(input)?;
    let (input, _) = newline(input)?;
    let (input, button_b) = parse_button(input)?;
    let (input, _) = newline(input)?;
    let (input, prize) = parse_prize(input)?;
    Ok((
        input,
        Entry {
            button_a,
            button_b,
            prize,
        },
    ))
}

fn parse_entries(input: &str) -> IResult<&str, Vec<Entry>> {
    separated_list1(tag("\n\n"), parse_entry)(input)
}

fn compute_cost(entries: &mut [Entry], increment: i64) -> Option<i64> {
    for entry in entries.iter_mut() {
        entry.prize.x += increment;
        entry.prize.y += increment;
    }
    let mut total_cost = 0;

    for entry in entries {
        let det_a =
            (entry.button_a.x * entry.button_b.y - entry.button_a.y * entry.button_b.x).abs();
        let det_ax = (entry.prize.x * entry.button_b.y - entry.prize.y * entry.button_b.x).abs();
        let det_ay = (entry.button_a.x * entry.prize.y - entry.button_a.y * entry.prize.x).abs();
        if det_ax % det_a == 0 && det_ay % det_a == 0 {
            let x = det_ax / det_a;
            let y = det_ay / det_a;
            total_cost += x * 3 + y;
        }
    }

    Some(total_cost)
}

pub fn part_one(input: &str) -> Option<i64> {
    let (_, mut entries) = parse_entries(input).unwrap_or_default();
    compute_cost(&mut entries, 0)
}


pub fn part_two(input: &str) -> Option<i64> {
    let (_, mut entries) = parse_entries(input).unwrap_or_default();
    compute_cost(&mut entries, 10000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
