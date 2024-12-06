#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

#[repr(usize)]
#[derive(Debug, PartialEq)]
enum Guard {
    Up,
    Down,
    Left,
    Right,
}

const GUARD_SYM: [char; 4] = ['^', 'v', '<', '>'];

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let mut lab = parse_input(input);
    let start_pos = start_position(&lab);
    let bounds_x = HashSet::from([0, lab.len() - 1]);
    let bounds_y = HashSet::from([0, lab[0].len() - 1]);
    let mut visited = HashSet::from([start_pos]);
    let mut curr_pos = start_pos;
    // includes start position
    let mut steps = 1;
    while !is_done(&bounds_x, &bounds_y, curr_pos) {
        if can_move(&lab, curr_pos) {
            curr_pos = advance_pos(&mut lab, curr_pos);
            if !visited.contains(&curr_pos) {
                steps += 1;
            }
            visited.insert(curr_pos);
        } else {
            rotate90(&mut lab, curr_pos);
        }
    }

    Some(steps)
}

fn can_move(lab: &[Vec<char>], curr_pos: (usize, usize)) -> bool {
    use Guard::{Down, Left, Right, Up};
    let x = curr_pos.0;
    let y = curr_pos.1;
    let curr_sym = lab[x][y];
    curr_sym == GUARD_SYM[Up as usize] && lab[x - 1][y] == '.'
        || curr_sym == GUARD_SYM[Down as usize] && lab[x + 1][y] == '.'
        || curr_sym == GUARD_SYM[Left as usize] && lab[x][y - 1] == '.'
        || curr_sym == GUARD_SYM[Right as usize] && lab[x][y + 1] == '.'
}

fn advance_pos(lab: &mut [Vec<char>], curr_pos: (usize, usize)) -> (usize, usize) {
    use Guard::{Down, Left, Right, Up};
    let mut curr_pos = curr_pos;
    let x = curr_pos.0;
    let y = curr_pos.1;
    let curr_sym = lab[x][y];
    if curr_sym == GUARD_SYM[Up as usize] {
        lab[x][y] = '.';
        curr_pos = (x - 1, y);
        lab[x - 1][y] = curr_sym;
    } else if curr_sym == GUARD_SYM[Down as usize] {
        lab[x][y] = '.';
        curr_pos = (x + 1, y);
        lab[x + 1][y] = curr_sym;
    } else if curr_sym == GUARD_SYM[Left as usize] {
        lab[x][y] = '.';
        curr_pos = (x, y - 1);
        lab[x][y - 1] = curr_sym;
    } else if curr_sym == GUARD_SYM[Right as usize] {
        lab[x][y] = '.';
        curr_pos = (x, y + 1);
        lab[x][y + 1] = curr_sym;
    }

    curr_pos
}

fn rotate90(lab: &mut [Vec<char>], curr_pos: (usize, usize)) {
    use Guard::{Down, Left, Right, Up};
    let x = curr_pos.0;
    let y = curr_pos.1;
    let curr_sym = lab[x][y];
    if curr_sym == GUARD_SYM[Up as usize] {
        lab[x][y] = GUARD_SYM[Right as usize];
    } else if curr_sym == GUARD_SYM[Down as usize] {
        lab[x][y] = GUARD_SYM[Left as usize];
    } else if curr_sym == GUARD_SYM[Left as usize] {
        lab[x][y] = GUARD_SYM[Up as usize];
    } else if curr_sym == GUARD_SYM[Right as usize] {
        lab[x][y] = GUARD_SYM[Down as usize];
    }
}

#[must_use] pub fn part_two(input: &str) -> Option<u32> {
    use Guard::Up;
    let mut lab = parse_input(input);
    let start_pos = start_position(&lab);
    let bounds_x = HashSet::from([0, lab.len() - 1]);
    let bounds_y = HashSet::from([0, lab[0].len() - 1]);
    let obstacle_positions = get_obstacle_positions(&lab);
    // includes start position
    let mut no_positions = 0;
    'outer: for pos in obstacle_positions {
        lab[pos.0][pos.1] = 'O';
        let mut curr_pos = start_pos;
        let mut visited = HashMap::from([((start_pos, GUARD_SYM[Up as usize]), false)]);
        while !is_done(&bounds_x, &bounds_y, curr_pos) {
            if can_move(&lab, curr_pos) {
                curr_pos = advance_pos(&mut lab, curr_pos);
                if visited.contains_key(&(curr_pos, lab[curr_pos.0][curr_pos.1])) {
                    // check to see if th next position is also visited
                    if is_next_pos_visited(&lab, &visited, curr_pos) {
                        no_positions += 1;
                        lab[pos.0][pos.1] = '.';
                        lab[curr_pos.0][curr_pos.1] = '.';
                        // reset to start position
                        lab[start_pos.0][start_pos.1] = GUARD_SYM[Up as usize];
                        continue 'outer;
                    }
                }
                visited.insert((curr_pos, lab[curr_pos.0][curr_pos.1]), true);
            } else {
                rotate90(&mut lab, curr_pos);
            }
        }
        // loop not found, go to next position
        lab[pos.0][pos.1] = '.';
        lab[curr_pos.0][curr_pos.1] = '.';
        // reset to start position
        lab[start_pos.0][start_pos.1] = GUARD_SYM[Up as usize];
    }

    Some(no_positions)
}

fn is_next_pos_visited(
    lab: &[Vec<char>],
    visited: &HashMap<((usize, usize), char), bool>,
    curr_pos: (usize, usize),
) -> bool {
    use Guard::{Down, Left, Right, Up};
    let x = curr_pos.0;
    let y = curr_pos.1;

    let curr_sym = lab[x][y];
    if curr_sym == GUARD_SYM[Up as usize] {
        visited.contains_key(&((x + 1, y), curr_sym))
            || visited.contains_key(&((x, y - 1), curr_sym))
            || visited.contains_key(&((x, y + 1), curr_sym))
    } else if curr_sym == GUARD_SYM[Down as usize] {
        visited.contains_key(&((x - 1, y), curr_sym))
            || visited.contains_key(&((x, y - 1), curr_sym))
            || visited.contains_key(&((x, y + 1), curr_sym))
    } else if curr_sym == GUARD_SYM[Left as usize] {
        visited.contains_key(&((x + 1, y), curr_sym))
            || visited.contains_key(&((x - 1, y), curr_sym))
            || visited.contains_key(&((x, y + 1), curr_sym))
    } else if curr_sym == GUARD_SYM[Right as usize] {
        visited.contains_key(&((x - 1, y), curr_sym))
            || visited.contains_key(&((x + 1, y), curr_sym))
            || visited.contains_key(&((x, y - 1), curr_sym))
    } else {
        false
    }
}

fn get_obstacle_positions(lab: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for i in 0..lab.len() {
        for j in 0..lab[0].len() {
            if lab[i][j] == '.' {
                positions.push((i, j));
            }
        }
    }

    positions
}

fn start_position(lab: &[Vec<char>]) -> (usize, usize) {
    for i in 0..lab.len() {
        for j in 0..lab[0].len() {
            if lab[i][j] == GUARD_SYM[Guard::Up as usize] {
                return (i, j);
            }
        }
    }

    (0, 0)
}

fn is_done(bounds_x: &HashSet<usize>, bounds_y: &HashSet<usize>, curr_pos: (usize, usize)) -> bool {
    bounds_x.contains(&curr_pos.0) || bounds_y.contains(&curr_pos.1)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut lab = Vec::new();
    for line in input.lines() {
        lab.push(line.chars().collect::<Vec<_>>());
    }

    lab
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
