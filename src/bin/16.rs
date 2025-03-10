advent_of_code::solution!(16);

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
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
const SYMS: [char; 4] = ['^', 'v', '<', '>'];

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

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        use Direction::*;
        match value {
            0 => Up,
            1 => Down,
            2 => Left,
            3 => Right,
            _ => panic!("Invalid usize value for Direction"),
        }
    }
}

// A struct to represent a state in the priority queue
#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: Pos,
    orientation: Direction,
}

// Implementing ordering for the BinaryHeap to prioritize smaller costs
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse to make BinaryHeap a min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

fn find_start_and_end_pos(map: &Map) -> (Pos, Pos) {
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

fn find_shortest_path(
    map: &Map,
    start: Pos,
    end: Pos,
) -> (HashMap<Pos, (usize, Direction)>, HashMap<Pos, Option<Pos>>) {
    let mut distances = HashMap::new();
    let mut previous_pos = HashMap::new();
    let mut priority_queue = BinaryHeap::new();
    use Direction::*;

    for i in 0..map.0.len() {
        for j in 0..map.0[i].len() {
            distances.insert(Pos(i as i32, j as i32), (usize::MAX, Right));
        }
    }

    distances.insert(start, (0, Right));
    priority_queue.push(State {
        cost: 0,
        pos: start,
        orientation: Right,
    });

    // 0123456789
    //0##########
    //1#.......E#
    //2#.##.#####
    //3#..#....##
    //4##.####.##
    //5#S......##
    //6##########
    while let Some(state) = priority_queue.pop() {
        let State {
            cost,
            pos,
            orientation,
        } = state;

        if pos == end {
            return (distances, previous_pos);
        }
        let (curr_cost, _) = *distances.get(&pos).unwrap_or(&(usize::MAX, Right));
        if curr_cost != usize::MAX && cost > curr_cost {
            continue;
        }
        // Explore neighbors
        // UP, DOWN, LEFT, RIGHT
        for (idx, &dir) in DIRECTIONS.iter().enumerate() {
            let new_pos = pos + dir;

            if map[new_pos] != '#' && new_pos != pos {
                let (orientation, weight) = get_penalty(orientation, idx);
                let new_cost = cost + weight;
                let (mut curr_cost, curr_dir) = *distances.get(&new_pos).unwrap_or(&(usize::MAX, Right));
                if curr_cost != usize::MAX {
                    let (_, weight) = get_penalty(curr_dir, idx);
                    curr_cost += weight;
                }
                if new_cost < curr_cost {
                    distances.insert(new_pos, (new_cost, orientation));
                    previous_pos.insert(new_pos, Some(pos));
                    let state = State {
                        cost: new_cost,
                        pos: new_pos,
                        orientation,
                    };
                    priority_queue.push(state);
                }
            }
        }
    }

    (distances, previous_pos)
}

fn cost_in_all_dirs(orientation: Direction, curr_cost: usize) -> [usize; 4] {
    let mut costs = [0usize; 4];

    for (idx, _) in DIRECTIONS.iter().enumerate() {
        let (_, c) = get_penalty(orientation, idx);
        if curr_cost == usize::MAX {
            costs[idx] = usize::MAX;
        } else {
            costs[idx] = c;
        }
    }

    costs
}

// A function to reconstruct the shortest path
fn compute_score_for_path(
    previous_pos: &HashMap<Pos, Option<Pos>>,
    start: Pos,
    target: Pos,
) -> Vec<Pos> {
    let mut path = Vec::new();
    let mut current = Some(target);

    while let Some(pos) = current {
        path.push(pos);
        current = *previous_pos.get(&pos).unwrap_or(&None);
    }

    path.reverse();

    if path.first() == Some(&start) {
        path
    } else {
        vec![] // Return an empty path if no valid path exists
    }
}
// fn find_shortest_path(map: &Map, start: Pos, end: Pos) -> Option<usize> {
//     let mut visited = HashSet::new();
//     let mut queue = VecDeque::new();
//     use Direction::*;
//     let mut update_map = map.clone();
//
//     // Initialize BFS
//     queue.push_back((start, 0, Right)); // (row, col, distance)
//     visited.insert(start);
//
//     while let Some((pos, dist, orientation)) = queue.pop_front() {
//         if pos == end {
//             for i in 0..update_map.0.len() {
//                 for j in 0..update_map.0[i].len() {
//                     print!("{}", update_map[Pos(i as i32,j as i32)]);
//                 }
//                 println!();
//             }
//             return Some(dist);
//         }
//
//         // Explore neighbors
//         for (idx, &dir) in DIRECTIONS.iter().enumerate() {
//             let new_pos = pos + dir;
//
//             if map[new_pos] != '#' && !visited.contains(&new_pos) {
//                 let (orientation, penalty) = get_penalty(orientation, idx);
//                 visited.insert(new_pos);
//                 update_map[new_pos] = SYMS[orientation as usize];
//                 queue.push_back((new_pos, dist + penalty, orientation));
//             }
//         }
//     }
//
//     None // No path found
// }

fn get_penalty(orientation: Direction, dir: usize) -> (Direction, usize) {
    use Direction::*;
    let clockwise = [Up, Right, Down, Left];
    let anti_clockwise = [Up, Left, Down, Right];
    let mut penalty_clockwise = 0;
    let mut penalty_anti_clockwise = 0;
    let mut curr_orientation = orientation;
    let next_orientation = Direction::from(dir);
    if orientation == next_orientation {
        return (orientation, 1);
    }

    let mut idx = clockwise
        .into_iter()
        .position(|d| d == orientation)
        .unwrap_or(0);

    while curr_orientation != next_orientation {
        penalty_clockwise += 1000;
        idx = (idx + 1) % clockwise.len();
        curr_orientation = clockwise[idx];
    }

    let mut idx = anti_clockwise
        .into_iter()
        .position(|d| d == orientation)
        .unwrap_or(0);

    let mut curr_orientation = orientation;
    while curr_orientation != next_orientation {
        penalty_anti_clockwise += 1000;
        idx = (idx + 1) % anti_clockwise.len();
        curr_orientation = anti_clockwise[idx];
    }

    if penalty_clockwise < penalty_anti_clockwise {
        (next_orientation, penalty_clockwise + 1)
    } else {
        (next_orientation, penalty_anti_clockwise + 1)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_input(input);
    let (start, end) = find_start_and_end_pos(&map);

    let (distances, previous_pos) = find_shortest_path(&map, start, end);
    // distances.get(&end).map(|(dist, _)| dist).copied()
    if !compute_score_for_path(&previous_pos, start, end).is_empty() {
        distances.get(&end).copied().map(|(c, _)| c)
    } else {
        Some(0)
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use Direction::*;

    #[test]
    fn test_penalty_for_turning() {
        // same orientation
        assert_eq!((Up, 1), get_penalty(Up, Up as usize));
        // orientation Up, next Left
        assert_eq!((Left, 1000), get_penalty(Up, Left as usize));
        // orientation Up, next Down
        assert_eq!((Down, 2000), get_penalty(Up, Down as usize));
        // orientation Up, next Right
        assert_eq!((Right, 1000), get_penalty(Up, Right as usize));
    }

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
    #[test]
    fn test_call_in_all_dir() {
        let costs = cost_in_all_dirs(Right, 0);
        assert_eq!([1001, 1001, 2001, 1], cost_in_all_dirs(Right, 0));

        // enum Direction {
        //     Up = 0,
        //     Down = 1,
        //     Left = 2,
        //     Right = 3,
        // }
    }
}
