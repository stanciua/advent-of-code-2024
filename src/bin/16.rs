advent_of_code::solution!(16);

use std::{
    collections::{HashSet, VecDeque},
    ops::{Add, Index, IndexMut},
};

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Ord, PartialOrd, Default, Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Pos(i32, i32);

#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct Map(Vec<Vec<char>>);

const DIRECTIONS: [Pos; 4] = [Pos(-1, 0), Pos(1, 0), Pos(0, -1), Pos(0, 1)];

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

// Implement the Index trait for Warehose
impl Index<Pos> for Map {
    type Output = char;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.0[usize::try_from(index.0).unwrap_or_default()]
            [usize::try_from(index.1).unwrap_or_default()]
    }
}

// Implement the IndexMut trait for Warehose
impl IndexMut<Pos> for Map {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.0[usize::try_from(index.0).unwrap_or_default()]
            [usize::try_from(index.1).unwrap_or_default()]
    }
}

fn parse_input(input: &str) -> Map {
    let mut map = Vec::new();
    for line in input.lines() {
        map.push(line.chars().collect());
    }

    Map(map)
}

fn display(map: &Map) {
    for row in &map.0 {
        for c in row {
            print!("{c}");
        }
        println!();
    }
}

fn find_start_and_end_pos(map: &Map) -> (Pos, Pos){
    let (mut start, mut end) = (Pos(0, 0), Pos(0, 0));
    for i in 0..map.0.len() {
        for j in 0..map.0[0].len() {
            let pos = Pos(i as i32, j as i32);
            let tile = map[pos];
            if tile == 'S' {
                start = pos;
            } else if tile == 'E' {
                end = pos;
            }
        }
    }

    (start, end)
}

fn find_shortest_path(map: &Map, start: Pos, end: Pos) -> Option<u64> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    // Initialize BFS
    queue.push_back((start, 0)); // (row, col, distance)
    visited.insert(start);

    while let Some((pos, dist)) = queue.pop_front() {
        if pos == end {
            return Some(dist);
        }

        // Explore neighbors
        for dir in DIRECTIONS {
            let new_pos = pos + dir;

            if map[new_pos] != '#' && !visited.contains(&new_pos) {
                visited.insert(new_pos);
                queue.push_back((new_pos, dist + 1));
            }
        }
    }

    None // No path found
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let (start, end) = find_start_and_end_pos(&map);

    find_shortest_path(&map, start, end)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
