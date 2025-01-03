#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
use std::collections::HashMap;

advent_of_code::solution!(14);

#[derive(Debug, PartialEq, Eq)]
struct Robot {
    pos: (i32, i32),
    velocity: (i32, i32),
}

#[derive(Debug, PartialEq, Eq)]
struct Space {
    robots: Vec<Robot>,
    width: i32,
    height: i32,
    robots_in_pos: HashMap<(i32, i32), u64>,
}

fn parse_input(input: &str) -> Option<Space> {
    let mut robots = Vec::new();
    let mut robots_in_pos = HashMap::new();

    let mut height = i32::MIN;
    let mut width = i32::MIN;
    for line in input.lines() {
        let (pos, velocity) = line.split_once(char::is_whitespace)?;
        let pos = pos.strip_prefix("p=")?.split_once(',')?;
        let pos = (pos.0.parse::<i32>().ok()?, pos.1.parse::<i32>().ok()?);
        let (x, y) = pos;
        if x > width {
            width = x;
        }
        if y > height {
            height = y;
        }

        let velocity = velocity.strip_prefix("v=")?.split_once(',')?;
        let velocity = (
            velocity.0.parse::<i32>().ok()?,
            velocity.1.parse::<i32>().ok()?,
        );

        robots_in_pos
            .entry(pos)
            .and_modify(|count| *count += 1)
            .or_insert_with(|| 1);

        robots.push(Robot { pos, velocity });
    }

    width += 1;
    height += 1;

    Some(Space {
        robots,
        width,
        height,
        robots_in_pos,
    })
}

fn navigate(space: &mut Space) {
    for robot in &mut space.robots {
        let (x, y) = robot.pos;
        space
            .robots_in_pos
            .entry((x, y))
            .and_modify(|count| *count -= 1);
        robot.pos = (
            (x + robot.velocity.0 + space.width) % space.width,
            (y + robot.velocity.1 + space.height) % space.height,
        );
        space
            .robots_in_pos
            .entry(robot.pos)
            .and_modify(|count| *count += 1)
            .or_insert_with(|| 1);
    }

    // remove 0 count robots
    space.robots_in_pos.retain(|&_, &mut v| v > 0);
}

fn count_robots(space: &Space) -> usize {
    // quadrants
    // 1 2
    // 3 4
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for robot in &space.robots {
        let (x, y) = robot.pos;
        let (middle_x, middle_y) = (space.width / 2, space.height / 2);
        if x < middle_x && y < middle_y {
            q1 += 1;
        } else if x > middle_x && y < middle_y {
            q2 += 1;
        } else if x < middle_x && y > middle_y {
            q3 += 1;
        } else if x > middle_x && y > middle_y {
            q4 += 1;
        }
    }

    q1 * q2 * q3 * q4
}

#[must_use] pub fn part_one(input: &str) -> Option<u64> {
    let mut space = parse_input(input)?;
    for _ in 0..100 {
        navigate(&mut space);
    }

    Some(count_robots(&space) as u64)
}

#[must_use] pub fn part_two(input: &str) -> Option<u64> {
    let mut space = parse_input(input)?;
    let mut seconds = 0;
    for i in 0..10000 {
        navigate(&mut space);
        // if there are no more overlapping robots on the same position,
        // we found the tree
        if space.robots_in_pos.values().all(|&v| v == 1) {
            seconds = i + 1;
            break;
        }
    }

    Some(seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
