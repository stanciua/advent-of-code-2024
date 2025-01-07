advent_of_code::solution!(15);
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Index, IndexMut},
};

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    TopLeft = 4,
    TopRight = 5,
    BottomLeft = 6,
    BottomRight = 7,
}

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Pos(i32, i32);

#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct Warehouse(Vec<Vec<char>>);

const DIRECTIONS: [Pos; 8] = [
    Pos(-1, 0),
    Pos(1, 0),
    Pos(0, -1),
    Pos(0, 1),
    Pos(-1, -1),
    Pos(-1, 1),
    Pos(1, -1),
    Pos(1, 1),
];

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Box {
    left: Pos,
    right: Pos,
}

#[derive(Debug, Eq, PartialEq)]
struct Document {
    warehouse: Warehouse,
    moves: Vec<Direction>,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}

// Implement the Index trait for Warehose
impl Index<Pos> for Warehouse {
    type Output = char;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.0[index.0 as usize][index.1 as usize]
    }
}

// Implement the IndexMut trait for Warehose
impl IndexMut<Pos> for Warehouse {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.0[index.0 as usize][index.1 as usize]
    }
}

fn parse_input(input: &str) -> Document {
    use Direction::*;
    let mut warehouse = Vec::new();
    let mut moves = Vec::new();
    let mut moves_flag = false;
    for line in input.lines() {
        if line.is_empty() {
            moves_flag = true;
            continue;
        }

        if !moves_flag {
            warehouse.push(line.chars().collect());
        } else {
            moves.extend(
                line.chars()
                    .map(|c| {
                        if c == '<' {
                            Left
                        } else if c == '>' {
                            Right
                        } else if c == '^' {
                            Up
                        } else {
                            Down
                        }
                    })
                    .collect::<Vec<_>>(),
            );
        }
    }

    Document {
        warehouse: Warehouse(warehouse),
        moves,
    }
}

fn double_box_set(warehouse: &Warehouse) -> HashSet<Box> {
    let mut boxes = HashSet::new();
    for i in 0..warehouse.0.len() {
        for j in 0..warehouse.0[0].len() - 1 {
            let left = Pos(i as i32, j as i32);
            let right = Pos(i as i32, (j + 1) as i32);
            if warehouse[left] == '[' && warehouse[right] == ']' {
                boxes.insert(Box { left, right });
            }
        }
    }
    boxes
}

fn get_neighbors_for_dir(
    warehouse: &Warehouse,
    boxes: &HashSet<Box>,
    pos: Pos,
    dir: Direction,
) -> Vec<Box> {
    use Direction::*;
    let mut neighbors = Vec::new();

    let left_to_box = boxes
        .iter()
        .copied()
        .map(|b| (b.left, b))
        .collect::<HashMap<_, _>>();
    let right_to_box = boxes
        .iter()
        .copied()
        .map(|b| (b.right, b))
        .collect::<HashMap<_, _>>();
    match dir {
        // ##############
        // ##.......##.##
        // ##....  ....##
        // ##...[][]...##
        // ##....[]....##
        // ##.....@....##
        // ##############
        Up => {
            let up = pos + DIRECTIONS[Up as usize];
            if warehouse[up] == ']' {
                neighbors.push(right_to_box[&up]);
            } else if warehouse[up] == '[' {
                neighbors.push(left_to_box[&up]);
            }
        }
        Down => {
            let down = pos + DIRECTIONS[Down as usize];
            if warehouse[down] == ']' {
                neighbors.push(right_to_box[&down]);
            } else if warehouse[down] == '[' {
                neighbors.push(left_to_box[&down]);
            }
        }
        Left => {
            let mut left = pos + DIRECTIONS[Left as usize];
            while warehouse[left] == ']' {
                neighbors.push(right_to_box[&left]);
                left = pos + DIRECTIONS[Left as usize] + DIRECTIONS[Left as usize];
            }
        }
        Right => {
            let mut right = pos + DIRECTIONS[Right as usize];
            while warehouse[right] == '[' {
                neighbors.push(left_to_box[&right]);
                right = pos + DIRECTIONS[Right as usize] + DIRECTIONS[Right as usize];
            }
        }
        _ => {
            unreachable!();
        }
    }

    neighbors
}
fn get_all_neighbors_for_dir(warehouse: &Warehouse, boxes: &HashSet<Box>, pos: Pos, dir: Direction) ->Vec<Box>{

    match dir {
        Up => {
            let mut neighbors = get_neighbors_for_dir(warehouse, boxes, pos, Up);
            while !neighbors.is_empty() {
                sticked.extend(
                    neighbors
                        .into_iter()
                        .flat_map(|b| [b.left, b.right].into_iter())
                        .collect::<Vec<_>>(),
                );

                neighbors = get_neighbors_for_dir(warehouse, boxes, pos, Up);
            }
        }
}

fn stick_robot_to_double_boxes(
    warehouse: &Warehouse,
    boxes: &HashSet<Box>,
    pos: Pos,
    dir: Direction,
) -> Vec<Pos> {
    use Direction::*;
    // the robot included is sticked to the a list of boxes
    let mut sticked = Vec::from([pos]);
    match dir {
        Up => {
            let mut neighbors = get_neighbors_for_dir(warehouse, boxes, pos, Up);
            while !neighbors.is_empty() {
                sticked.extend(
                    neighbors
                        .into_iter()
                        .flat_map(|b| [b.left, b.right].into_iter())
                        .collect::<Vec<_>>(),
                );

                neighbors = get_neighbors_for_dir(warehouse, boxes, pos, Up);
            }
        }
        Down => {
            let mut neighbors = get_neighbors_for_dir(warehouse, boxes, pos, Down);
            while !neighbors.is_empty() {
                sticked.extend(
                    neighbors
                        .into_iter()
                        .flat_map(|b| [b.left, b.right].into_iter())
                        .collect::<Vec<_>>(),
                );

                neighbors = get_neighbors_for_dir(warehouse, boxes, pos, Down);
            }
        }
        Left => {
            let neighbors = get_neighbors_for_dir(warehouse, boxes, pos, Left);
            if !neighbors.is_empty() {
                sticked.extend(
                    neighbors
                        .into_iter()
                        .flat_map(|b| [b.left, b.right].into_iter())
                        .collect::<Vec<_>>(),
                );
            }
        }
        Right => {
            let neighbors = get_neighbors_for_dir(warehouse, boxes, pos, Right);
            if !neighbors.is_empty() {
                sticked.extend(
                    neighbors
                        .into_iter()
                        .flat_map(|b| [b.left, b.right].into_iter())
                        .collect::<Vec<_>>(),
                );
            }
        }
        _ => {
            unreachable!();
        }
    }

    sticked.reverse();
    sticked
}

fn move_boxes_by_robot(document: &mut Document, pos: Pos) {
    let mut pos = pos;
    for (idx, &dir) in document.moves.iter().enumerate() {
        let sticked = stick_robot_to_boxes(&document.warehouse, pos, dir);
        let first_pos = sticked[0];
        let next = first_pos + DIRECTIONS[dir as usize];
        let warehouse = &mut document.warehouse;
        if warehouse[next] == '.' {
            // we are able to move in this direction, so move all sticked boxes if any and the
            // robot
            for (idx, &p) in sticked.iter().enumerate() {
                let next = p + DIRECTIONS[dir as usize];
                warehouse[next] = warehouse[p];
                if idx == sticked.len() - 1 {
                    pos = next;
                }
            }
            // the last position in the sticked list after move should be '.'
            let last_pos = sticked[sticked.len() - 1];
            warehouse[last_pos] = '.';
        }

        // display(&document.warehouse);
    }
}

fn stick_robot_to_boxes(warehouse: &Warehouse, pos: Pos, dir: Direction) -> Vec<Pos> {
    use Direction::*;
    // the robot included is sticked to the a list of boxes
    let mut sticked = Vec::from([pos]);
    match dir {
        Up => {
            let mut next = pos + DIRECTIONS[Up as usize];
            while warehouse[next] == 'O' {
                sticked.push(next);
                next = next + DIRECTIONS[Up as usize];
            }
        }
        Down => {
            let mut next = pos + DIRECTIONS[Down as usize];
            while warehouse[next] == 'O' {
                sticked.push(next);
                next = next + DIRECTIONS[Down as usize];
            }
        }
        Left => {
            let mut next = pos + DIRECTIONS[Left as usize];
            while warehouse[next] == 'O' {
                sticked.push(next);
                next = next + DIRECTIONS[Left as usize];
            }
        }
        Right => {
            let mut next = pos + DIRECTIONS[Right as usize];
            while warehouse[next] == 'O' {
                sticked.push(next);
                next = next + DIRECTIONS[Right as usize];
            }
        }
        _ => {
            unreachable!();
        }
    }

    sticked.reverse();
    sticked
}

fn move_robot(document: &mut Document, pos: Pos) {
    let mut pos = pos;
    for (idx, &dir) in document.moves.iter().enumerate() {
        let sticked = stick_robot_to_boxes(&document.warehouse, pos, dir);
        let first_pos = sticked[0];
        let next = first_pos + DIRECTIONS[dir as usize];
        let warehouse = &mut document.warehouse;
        if warehouse[next] == '.' {
            // we are able to move in this direction, so move all sticked boxes if any and the
            // robot
            for (idx, &p) in sticked.iter().enumerate() {
                let next = p + DIRECTIONS[dir as usize];
                warehouse[next] = warehouse[p];
                if idx == sticked.len() - 1 {
                    pos = next;
                }
            }
            // the last position in the sticked list after move should be '.'
            let last_pos = sticked[sticked.len() - 1];
            warehouse[last_pos] = '.';
        }

        // display(&document.warehouse);
    }
}

fn display(warehouse: &Warehouse) {
    for row in &warehouse.0 {
        for c in row {
            print!("{c}");
        }
        println!();
    }
}

fn sum_gps_coord(warehouse: &Warehouse) -> usize {
    let mut sum = 0;
    for (i, row) in warehouse.0.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if warehouse[Pos(i as i32, j as i32)] == 'O' {
                sum += (i) * 100 + j;
            }
        }
    }
    sum
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut document = parse_input(input);
    let rows = document.warehouse.0.len();
    let cols = document.warehouse.0[0].len();
    let mut robot_pos = Pos(0, 0);
    {
        let warehouse = &document.warehouse;
        'outer: for i in 0..rows {
            for j in 0..cols {
                if warehouse[Pos(i as i32, j as i32)] == '@' {
                    robot_pos = Pos(i as i32, j as i32);
                    break 'outer;
                }
            }
        }
    }

    move_robot(&mut document, robot_pos);

    // display(&document.warehouse);

    Some(sum_gps_coord(&document.warehouse) as u64)
}

fn double_tiles(warehouse: &Warehouse) -> Warehouse {
    let mut new_warehouse = Vec::new();
    for row in &warehouse.0 {
        let mut new_row = Vec::new();
        for &tile in row {
            if tile == '#' {
                new_row.push('#');
                new_row.push('#');
            } else if tile == 'O' {
                new_row.push('[');
                new_row.push(']');
            } else if tile == '.' {
                new_row.push('.');
                new_row.push('.');
            } else if tile == '@' {
                new_row.push('@');
                new_row.push('.');
            }
        }

        new_warehouse.push(new_row);
    }

    Warehouse(new_warehouse)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut document = parse_input(input);
    let new_warehouse = double_tiles(&document.warehouse);
    document.warehouse = new_warehouse;
    display(&document.warehouse);
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
