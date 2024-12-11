#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let mut antennas = HashMap::new();
    let mut seen = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let sym = map[i][j];
            if sym.is_numeric() || sym.is_lowercase() || sym.is_uppercase() {
                antennas
                    .entry(sym)
                    .and_modify(|pos: &mut Vec<(usize, usize)>| pos.push((i, j)))
                    .or_insert_with(|| Vec::from([(i, j)]));
            }
        }
    }

    let mut antinodes = 0;
    for positions in antennas.values() {
        let combinations = get_all_antennas_combinations(&positions);
        for c in &combinations {
            let diff_x = (c[0].0 as isize - c[1].0 as isize).unsigned_abs();
            let diff_y = (c[0].1 as isize - c[1].1 as isize).unsigned_abs();
            // look in the top-right corner
            let anx = c[0].0 as isize - diff_x as isize;
            let any = c[0].1 + diff_y;
            if anx >= 0 && any < map[0].len() && in_line_3_points(c[0], c[1], (anx as usize, any)) {
                if !seen.contains(&(anx as usize, any)) {
                    antinodes += 1;
                    seen.insert((anx as usize, any));
                }
            }
            // look in the top-left corner
            let anx = c[0].0 as isize - diff_x as isize;
            let any = c[0].1 as isize - diff_y as isize;
            if anx >= 0 && any >= 0 && in_line_3_points(c[0], c[1], (anx as usize, any as usize)) {
                if !seen.contains(&(anx as usize, any as usize)) {
                    antinodes += 1;
                    seen.insert((anx as usize, any as usize));
                }
            }
            // look in the bottom-right corner
            let anx = c[1].0 + diff_x;
            let any = c[1].1 + diff_y;
            if anx < map.len() && any < map[0].len() && in_line_3_points(c[0], c[1], (anx, any)) {
                if !seen.contains(&(anx, any)) {
                    antinodes += 1;
                    seen.insert((anx, any));
                }
            }
            // look in the bottom-left corner
            let anx = c[1].0 + diff_x;
            let any = c[1].1 as isize - diff_y as isize;
            if anx < map.len() && any >= 0 && in_line_3_points(c[0], c[1], (anx, any as usize)) {
                if !seen.contains(&(anx, any as usize)) {
                    antinodes += 1;
                    seen.insert((anx, any as usize));
                }
            }
        }
    }

    Some(antinodes)
}

fn get_all_antennas_combinations(positions: &[(usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut combinations = Vec::new();
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            combinations.push(Vec::from([positions[i], positions[j]]));
        }
    }

    combinations
}

fn in_line_2_points(a1: (usize, usize), a2: (usize, usize)) -> bool {
    let (x1, y1) = a1;
    let (x2, y2) = a2;
    let diff_x = (x2 as isize - x1 as isize).unsigned_abs();
    let diff_y = (y2 as isize - y1 as isize).unsigned_abs();

    a2 == (a1.0 + diff_x, a1.1 + diff_y)
}

fn in_line_3_points(a1: (usize, usize), a2: (usize, usize), in1: (usize, usize)) -> bool {
    let (x1, y1) = a1;
    let (x2, y2) = a2;
    let (x3, y3) = in1;
    let area = (x1 as isize * (y2 as isize - y3 as isize)
        + x2 as isize * (y3 as isize - y1 as isize)
        + x3 as isize * (y1 as isize - y2 as isize))
        .abs() as f64
        / 2.0;
    area == 0.0
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse_input(input);
    let mut antennas = HashMap::new();
    let mut seen = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let sym = map[i][j];
            if sym.is_numeric() || sym.is_lowercase() || sym.is_uppercase() {
                antennas
                    .entry(sym)
                    .and_modify(|pos: &mut Vec<(usize, usize)>| pos.push((i, j)))
                    .or_insert(Vec::from([(i, j)]));
            }
        }
    }

    let mut antinodes = 0;
    let antennas_copy = antennas.clone();
    for positions in antennas.values() {
        let combinations = get_all_antennas_combinations(&positions);
        for c in &combinations {
            let diff_x = (c[0].0 as isize - c[1].0 as isize).unsigned_abs();
            let diff_y = (c[0].1 as isize - c[1].1 as isize).unsigned_abs();
            // look in the top-right corner
            let mut anx = c[0].0 as isize - diff_x as isize;
            let mut any = c[0].1 + diff_y;
            while anx >= 0 && any < map[0].len() {
                if in_line_3_points(c[0], c[1], (anx as usize, any)) {
                    if !seen.contains(&(anx as usize, any)) && map[anx as usize][any] == '.' {
                        antinodes += 1;
                        seen.insert((anx as usize, any));
                    }
                }
                anx -= diff_x as isize;
                any += diff_y;
            }
            // look in the top-left corner
            let mut anx = c[0].0 as isize - diff_x as isize;
            let mut any = c[0].1 as isize - diff_y as isize;
            while anx >= 0 && any >= 0 {
                if in_line_3_points(c[0], c[1], (anx as usize, any as usize)) {
                    if !seen.contains(&(anx as usize, any as usize))
                        && map[anx as usize][any as usize] == '.'
                    {
                        antinodes += 1;
                        seen.insert((anx as usize, any as usize));
                    }
                }
                anx -= diff_x as isize;
                any -= diff_y as isize;
            }
            // look in the bottom-right corner
            let mut anx = c[1].0 + diff_x;
            let mut any = c[1].1 + diff_y;
            while anx < map.len() && any < map[0].len() {
                if in_line_3_points(c[0], c[1], (anx, any)) && map[anx][any] == '.' {
                    if !seen.contains(&(anx, any)) {
                        antinodes += 1;
                        seen.insert((anx, any));
                    }
                }
                anx += diff_x;
                any += diff_y;
            }
            // look in the bottom-left corner
            let mut anx = c[1].0 + diff_x;
            let mut any = c[1].1 as isize - diff_y as isize;
            while anx < map.len() && any >= 0 {
                if in_line_3_points(c[0], c[1], (anx, any as usize)) {
                    if !seen.contains(&(anx, any as usize)) && map[anx][any as usize] == '.' {
                        antinodes += 1;
                        seen.insert((anx, any as usize));
                    }
                }
                anx += diff_x;
                any -= diff_y as isize;
            }
        }
    }
    Some(
        antinodes
            + antennas_copy
                .into_values()
                .filter_map(|v| {
                    if v.len() > 1 {
                        Some(v.len() as u32)
                    } else {
                        None
                    }
                })
                .sum::<u32>(),
    )
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut map = Vec::new();
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        map.push(chars);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
