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
    Mul{op1: u32, op2: u32 },
    Do,
    Dont,
}

#[derive(Debug)]
struct FunctionCall {
    name: String,
    args: (u32, u32),
}

fn parse_1_to_3_digits(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 3, |c: char| c.is_ascii_digit())(input)
}

fn function_name(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("mul")(input)?;
    Ok((input, "mul".to_string()))
}

fn argument(input: &str) -> IResult<&str, u32> {
    map_res(
        preceded(multispace0, parse_1_to_3_digits),
        |digit_str: &str| digit_str.parse::<u32>(),
    )(input)
}

fn arguments(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, (arg1, _, arg2)) = tuple((argument, char(','), argument))(input)?;
    Ok((input, (arg1, arg2)))
}

fn function_call(input: &str) -> IResult<&str, FunctionCall> {
    let (input, name) = function_name(input)?;
    let (input, args) = delimited(char('('), arguments, char(')'))(input)?;
    let fcall = FunctionCall { name, args };

    Ok((input, fcall))
}

fn function_do(input: &str) -> IResult<&str, FunctionCall> {
    let (input, _) = tag("do()")(input)?;
    let do_call = FunctionCall {
        name: "do".to_string(),
        args: (0, 0),
    };

    Ok((input, do_call))
}

fn function_dont(input: &str) -> IResult<&str, FunctionCall> {
    let (input, _) = tag("don't()")(input)?;
    let dont_call = FunctionCall {
        name: "dont".to_string(),
        args: (0, 0),
    };

    Ok((input, dont_call))
}

fn prefix_and_function_call(input: &str) -> IResult<&str, FunctionCall> {
    let (input, _) = many_till(
        anychar,
        peek(alt((function_call, function_do, function_dont))),
    )(input)?;
    alt((function_call, function_do, function_dont))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, function_calls) = many1(prefix_and_function_call)(input).ok()?;
    Some(
        function_calls
            .into_iter()
            .map(|fc| fc.args.0 * fc.args.1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, function_calls) = many1(prefix_and_function_call)(input).ok()?;
    let mut dont_flag = false;
    let mut sum = 0;
    for fcall in function_calls {
        if fcall.name == "dont" {
            dont_flag = true;
            continue;
        } else if fcall.name == "do" {
            dont_flag = false;
            continue;
        } else if !dont_flag {
            sum += fcall.args.0 * fcall.args.1;
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
