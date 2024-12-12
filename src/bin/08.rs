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

    let mut count = 0;
    for positions in antennas.values() {
        let combinations = get_all_antennas_combinations(positions);
        for c in combinations {
            update_antinodes(c, &mut seen, &map, &mut count, false);
        }
    }
    Some(count)
}
fn update_antinodes(
    antenna_pair: [(usize, usize); 2],
    seen: &mut HashSet<(usize, usize)>,
    map: &[Vec<char>],
    count: &mut u32,
    multi_antinodes_flg: bool,
) {
    let [(x1, y1), (x2, y2)] = antenna_pair;
    let dx = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let dy = if y1 > y2 { y1 - y2 } else { y2 - y1 };
    let (x1, y1, x2, y2, dx, dy) = (
        x1 as isize,
        y1 as isize,
        x2 as isize,
        y2 as isize,
        dx as isize,
        dy as isize,
    );
    let mut positions = [
        (x1 - dx, y1 + dy), // top-right
        (x1 - dx, y1 - dy), // top-right
        (x2 + dx, y2 + dy), // bottom-right
        (x2 + dx, y2 - dy), // bottom-left
    ];
    loop {
        for (idx, (ax, ay)) in positions.iter_mut().enumerate() {
            if is_pos_out_of_bounds(map, (*ax, *ay)) {
                continue;
            }
            if are_3_points_inline((x1, y1), (x2, y2), (*ax, *ay))
                && !seen.contains(&(*ax as usize, *ay as usize))
            {
                if multi_antinodes_flg {
                    if map[*ax as usize][*ay as usize] == '.' {
                        *count += 1;
                        seen.insert((*ax as usize, *ay as usize));
                    }
                } else {
                    *count += 1;
                    seen.insert((*ax as usize, *ay as usize));
                }
            }
            match idx {
                0 => {
                    *ax -= dx;
                    *ay += dy;
                }
                1 => {
                    *ax -= dx;
                    *ay -= dy;
                }
                2 => {
                    *ax += dx;
                    *ay += dy;
                }
                3 => {
                    *ax += dx;
                    *ay -= dy;
                }
                _ => unreachable!(),
            }
        }

        if !multi_antinodes_flg || positions.iter().all(|pos| is_pos_out_of_bounds(map, *pos)) {
            break;
        }
    }
}

fn is_pos_out_of_bounds(map: &[Vec<char>], pos: (isize, isize)) -> bool {
    let (ax, ay) = pos;
    ax < 0 || ax >= map.len() as isize || ay < 0 || ay >= map[0].len() as isize
}

fn get_all_antennas_combinations(positions: &[(usize, usize)]) -> Vec<[(usize, usize); 2]> {
    let mut combinations = Vec::new();
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            combinations.push([positions[i], positions[j]]);
        }
    }

    combinations
}

fn are_3_points_inline(a1: (isize, isize), a2: (isize, isize), in1: (isize, isize)) -> bool {
    let (x1, y1) = a1;
    let (x2, y2) = a2;
    let (x3, y3) = in1;
    (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs() as f64 / 2.0 == 0.0
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

    let mut count = 0;
    for positions in antennas.values() {
        let combinations = get_all_antennas_combinations(positions);
        for c in combinations {
            update_antinodes(c, &mut seen, &map, &mut count, true);
        }
    }
    Some(
        count
            + antennas
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
