#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
advent_of_code::solution!(3);

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{anychar, char, multispace0},
    combinator::{map_res, peek},
    multi::{many1, many_till},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
enum Operation {
    Mul { op1: u32, op2: u32 },
    Do,
    Dont,
}

fn digits(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 3, |c: char| c.is_ascii_digit())(input)
}

fn argument(input: &str) -> IResult<&str, u32> {
    map_res(
        preceded(multispace0, digits),
        |digit_str: &str| digit_str.parse::<u32>(),
    )(input)
}

fn arguments(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, (arg1, _, arg2)) = tuple((argument, char(','), argument))(input)?;
    Ok((input, (arg1, arg2)))
}

fn mul(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("mul")(input)?;
    let (input, args) = delimited(char('('), arguments, char(')'))(input)?;
    let op = Operation::Mul {
        op1: args.0,
        op2: args.1,
    };

    Ok((input, op))
}

fn r#do(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Operation::Do))
}

fn dont(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Operation::Dont))
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = many_till(
        anychar,
        peek(alt((mul, r#do, dont))),
    )(input)?;
    alt((mul, r#do, dont))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, operations) = many1(operation)(input).ok()?;
    Some(
        operations
            .into_iter()
            .map(|op| {
                if let Operation::Mul { op1, op2 } = op {
                    op1 * op2
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, operations) = many1(operation)(input).ok()?;
    let mut dont_flag = false;
    let mut sum = 0;
    for op in operations {
        match op {
            Operation::Dont => dont_flag = true,
            Operation::Do => dont_flag = false,
            Operation::Mul { op1, op2 } => {
                if !dont_flag {
                    sum += op1 * op2;
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
