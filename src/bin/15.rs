#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
advent_of_code::solution!(15);
use std::{
    collections::VecDeque,
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
struct Warehouse(Vec<Vec<char>>);

const DIRECTIONS: [Pos; 4] = [Pos(-1, 0), Pos(1, 0), Pos(0, -1), Pos(0, 1)];

#[derive(Debug, Eq, PartialEq)]
struct Document {
    warehouse: Warehouse,
    moves: Vec<Direction>,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

// Implement the Index trait for Warehose
impl Index<Pos> for Warehouse {
    type Output = char;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.0[usize::try_from(index.0).unwrap_or_default()]
            [usize::try_from(index.1).unwrap_or_default()]
    }
}

// Implement the IndexMut trait for Warehose
impl IndexMut<Pos> for Warehouse {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.0[usize::try_from(index.0).unwrap_or_default()]
            [usize::try_from(index.1).unwrap_or_default()]
    }
}

fn parse_input(input: &str) -> Document {
    use Direction::{Down, Left, Right, Up};
    let mut warehouse = Vec::new();
    let mut moves = Vec::new();
    let mut moves_flag = false;
    for line in input.lines() {
        if line.is_empty() {
            moves_flag = true;
            continue;
        }

        if moves_flag {
            moves.extend(line.chars().map(|c| {
                if c == '<' {
                    Left
                } else if c == '>' {
                    Right
                } else if c == '^' {
                    Up
                } else {
                    Down
                }
            }));
        } else {
            warehouse.push(line.chars().collect());
        }
    }

    Document {
        warehouse: Warehouse(warehouse),
        moves,
    }
}

fn stick_robot_to_boxes2(warehouse: &Warehouse, pos: Pos, dir: Direction) -> Vec<Pos> {
    use Direction::{Down, Left, Right, Up};
    // the robot included is sticked to the a list of boxes
    let mut sticked = Vec::from([pos]);
    let mut curr_boxes_per_level = VecDeque::from([pos]);
    match dir {
        Up => {
            while !curr_boxes_per_level.is_empty() {
                let Some(curr_pos) = curr_boxes_per_level.pop_front() else {
                    return sticked;
                };
                let next = curr_pos + DIRECTIONS[Up as usize];
                let next_tile = warehouse[next];
                if next_tile == '[' {
                    let right = next + DIRECTIONS[Right as usize];
                    if !sticked.contains(&next) {
                        sticked.push(next);
                        curr_boxes_per_level.push_back(next);
                    }
                    if !sticked.contains(&right) {
                        sticked.push(right);
                        curr_boxes_per_level.push_back(right);
                    }
                } else if next_tile == ']' {
                    let left = next + DIRECTIONS[Left as usize];
                    if !sticked.contains(&next) {
                        sticked.push(next);
                        curr_boxes_per_level.push_back(next);
                    }
                    if !sticked.contains(&left) {
                        sticked.push(left);
                        curr_boxes_per_level.push_back(left);
                    }
                } else if next_tile == 'O' && !sticked.contains(&next) {
                    sticked.push(next);
                    curr_boxes_per_level.push_back(next);
                }
            }
        }
        Down => {
            while !curr_boxes_per_level.is_empty() {
                let Some(curr_pos) = curr_boxes_per_level.pop_front() else {
                    return sticked;
                };
                let next = curr_pos + DIRECTIONS[Down as usize];
                let next_tile = warehouse[next];
                if next_tile == '[' {
                    let right = next + DIRECTIONS[Right as usize];
                    if !sticked.contains(&next) {
                        sticked.push(next);
                        curr_boxes_per_level.push_back(next);
                    }
                    if !sticked.contains(&right) {
                        sticked.push(right);
                        curr_boxes_per_level.push_back(right);
                    }
                } else if next_tile == ']' {
                    let left = next + DIRECTIONS[Left as usize];
                    if !sticked.contains(&next) {
                        sticked.push(next);
                        curr_boxes_per_level.push_back(next);
                    }
                    if !sticked.contains(&left) {
                        sticked.push(left);
                        curr_boxes_per_level.push_back(left);
                    }
                } else if next_tile == 'O' && !sticked.contains(&next) {
                    sticked.push(next);
                    curr_boxes_per_level.push_back(next);
                }
            }
        }
        Left => {
            let mut next = pos + DIRECTIONS[Left as usize];
            let mut next_tile = warehouse[next];
            while next_tile == '[' || next_tile == ']' || next_tile == 'O' {
                sticked.push(next);
                next = next + DIRECTIONS[Left as usize];
                next_tile = warehouse[next];
            }
        }
        Right => {
            let mut next = pos + DIRECTIONS[Right as usize];
            let mut next_tile = warehouse[next];
            while next_tile == '[' || next_tile == ']' || next_tile == 'O' {
                sticked.push(next);
                next = next + DIRECTIONS[Right as usize];
                next_tile = warehouse[next];
            }
        }
    }

    sticked.reverse();
    sticked
}

fn move_robot_in_dir(warehouse: &mut Warehouse, pos: &mut Pos, dir: Direction) {
    let sticked = stick_robot_to_boxes2(warehouse, *pos, dir);

    if can_boxes_move_in_dir(warehouse, &sticked, dir) {
        // we are able to move in this direction, so move all sticked boxes if any and the
        // robot
        for &p in &sticked {
            let next = p + DIRECTIONS[dir as usize];
            warehouse[next] = warehouse[p];
            warehouse[p] = '.';
            if warehouse[next] == '@' {
                *pos = next;
            }
        }
    }
}

fn move_robot(document: &mut Document, pos: &mut Pos) {
    for &dir in &document.moves {
        move_robot_in_dir(&mut document.warehouse, pos, dir);
    }
}

fn can_boxes_move_in_dir(warehouse: &Warehouse, sticked_boxes: &Vec<Pos>, dir: Direction) -> bool {
    use Direction::{Down, Left, Right, Up};
    match dir {
        Up => {
            for &pos in sticked_boxes {
                let next = pos + DIRECTIONS[Up as usize];
                let next_tile = warehouse[next];
                if next_tile == '#' {
                    return false;
                }
            }
        }
        Down => {
            for &pos in sticked_boxes {
                let next = pos + DIRECTIONS[Down as usize];
                let next_tile = warehouse[next];
                if next_tile == '#' {
                    return false;
                }
            }
        }

        Left => {
            let fbox_left = sticked_boxes[0];
            let next_left = fbox_left + DIRECTIONS[Left as usize];
            if warehouse[next_left] == '#' {
                return false;
            }
            if sticked_boxes.len() > 1 {
                let fbox_right = sticked_boxes[1];
                let next_right = fbox_right + DIRECTIONS[Left as usize];
                if warehouse[next_right] == '#' {
                    return false;
                }
            }
        }

        Right => {
            let fbox_left = sticked_boxes[0];
            let next_left = fbox_left + DIRECTIONS[Right as usize];
            if warehouse[next_left] == '#' {
                return false;
            }
            if sticked_boxes.len() > 1 {
                let fbox_right = sticked_boxes[1];
                let next_right = fbox_right + DIRECTIONS[Right as usize];
                if warehouse[next_right] == '#' {
                    return false;
                }
            }
        }
    }

    true
}

fn sum_gps_coord(warehouse: &Warehouse) -> usize {
    let mut sum = 0;
    for (i, row) in warehouse.0.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let tile = warehouse[Pos(
                i32::try_from(i).unwrap_or_default(),
                i32::try_from(j).unwrap_or_default(),
            )];
            if tile == 'O' || tile == '[' {
                sum += (i) * 100 + j;
            }
        }
    }
    sum
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    let mut document = parse_input(input);
    let mut robot_pos = find_robot_pos(&document.warehouse);
    move_robot(&mut document, &mut robot_pos);
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

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    let mut document = parse_input(input);
    let new_warehouse = double_tiles(&document.warehouse);
    document.warehouse = new_warehouse;
    let mut robot_pos = find_robot_pos(&document.warehouse);

    move_robot(&mut document, &mut robot_pos);
    Some(sum_gps_coord(&document.warehouse) as u64)
}

fn find_robot_pos(warehouse: &Warehouse) -> Pos {
    let rows = warehouse.0.len();
    let cols = warehouse.0[0].len();
    for i in 0..rows {
        for j in 0..cols {
            let pos = Pos(
                i32::try_from(i).unwrap_or_default(),
                i32::try_from(j).unwrap_or_default(),
            );
            let tile = warehouse[pos];
            if tile == '@' {
                return pos;
            }
        }
    }

    Pos(0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }

    #[test]
    fn test_stick_boxes_left() {
        use Direction::*;
        let input = r"########
........
..[][]@.";
        let document = parse_input(input);
        let robot_pos = find_robot_pos(&document.warehouse);
        let sticked = stick_robot_to_boxes2(&document.warehouse, robot_pos, Left);
        let expected = vec![Pos(2, 2), Pos(2, 3), Pos(2, 4), Pos(2, 5), Pos(2, 6)];
        assert_eq!(sticked, expected);
    }

    #[test]
    fn test_stick_boxes_left_move_ok() {
        use Direction::*;
        let input = r"########
........
..[][]@.";
        let document = parse_input(input);
        let robot_pos = find_robot_pos(&document.warehouse);
        let sticked = stick_robot_to_boxes2(&document.warehouse, robot_pos, Left);
        let expected = vec![Pos(2, 2), Pos(2, 3), Pos(2, 4), Pos(2, 5), Pos(2, 6)];
        assert_eq!(sticked, expected);
        assert!(can_boxes_move_in_dir(&document.warehouse, &sticked, Left));
    }

    #[test]
    fn test_stick_boxes_left_move_nok() {
        use Direction::*;
        let input = r"########
........
.#[][]@.";
        let document = parse_input(input);
        let robot_pos = find_robot_pos(&document.warehouse);
        let sticked = stick_robot_to_boxes2(&document.warehouse, robot_pos, Left);
        let expected = vec![Pos(2, 2), Pos(2, 3), Pos(2, 4), Pos(2, 5), Pos(2, 6)];
        assert_eq!(sticked, expected);
        assert!(!can_boxes_move_in_dir(&document.warehouse, &sticked, Left));
    }

    #[test]
    fn test_stick_boxes_right() {
        use Direction::*;
        let input = r"########
........
.@[][]..";
        let document = parse_input(input);
        let robot_pos = find_robot_pos(&document.warehouse);
        let sticked = stick_robot_to_boxes2(&document.warehouse, robot_pos, Right);
        let expected = vec![Pos(2, 5), Pos(2, 4), Pos(2, 3), Pos(2, 2), Pos(2, 1)];
        assert_eq!(sticked, expected);
    }

    #[test]
    fn test_stick_boxes_right_move_ok() {
        use Direction::*;
        let input = r"########
........
.@[][]..";
        let document = parse_input(input);
        let robot_pos = find_robot_pos(&document.warehouse);
        let sticked = stick_robot_to_boxes2(&document.warehouse, robot_pos, Right);
        let expected = vec![Pos(2, 5), Pos(2, 4), Pos(2, 3), Pos(2, 2), Pos(2, 1)];
        assert_eq!(sticked, expected);
        assert!(can_boxes_move_in_dir(&document.warehouse, &sticked, Right));
    }
    #[test]
    fn test_stick_boxes_right_move_nok() {
        use Direction::*;
        let input = r"########
........
.@[][]#.";
        let document = parse_input(input);
        let robot_pos = find_robot_pos(&document.warehouse);
        let sticked = stick_robot_to_boxes2(&document.warehouse, robot_pos, Right);
        let expected = vec![Pos(2, 5), Pos(2, 4), Pos(2, 3), Pos(2, 2), Pos(2, 1)];
        assert_eq!(sticked, expected);
        assert!(!can_boxes_move_in_dir(&document.warehouse, &sticked, Right));
    }
    #[test]
    fn test_stick_boxes_up() {
        use Direction::*;
        let input = r"########
........
...[]...
...[][].
....[]..
.....@..";
        let document = parse_input(input);
        let robot_pos = find_robot_pos(&document.warehouse);
        let sticked = stick_robot_to_boxes2(&document.warehouse, robot_pos, Up);
        let expected = vec![
            Pos(2, 3),
            Pos(2, 4),
            Pos(3, 3),
            Pos(3, 4),
            Pos(3, 6),
            Pos(3, 5),
            Pos(4, 4),
            Pos(4, 5),
            Pos(5, 5),
        ];
        assert_eq!(sticked, expected);
    }

    #[test]
    fn test_stick_boxes_up_move_ok() {
        use Direction::*;
        let input = r"########
........
...[]...
...[][].
....[]..
.....@..";
        let mut document = parse_input(input);
        let mut robot_pos = find_robot_pos(&document.warehouse);
        let sticked = stick_robot_to_boxes2(&document.warehouse, robot_pos, Up);
        let expected = vec![
            Pos(2, 3),
            Pos(2, 4),
            Pos(3, 3),
            Pos(3, 4),
            Pos(3, 6),
            Pos(3, 5),
            Pos(4, 4),
            Pos(4, 5),
            Pos(5, 5),
        ];
        assert_eq!(sticked, expected);
        assert!(can_boxes_move_in_dir(&document.warehouse, &sticked, Up));
        let expect = r"########
...[]...
...[][].
....[]..
.....@..
........"
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        move_robot_in_dir(&mut document.warehouse, &mut robot_pos, Up);

        assert_eq!(Warehouse(expect), document.warehouse);
    }

    #[test]
    fn test_stick_boxes_up_move_nok() {
        use Direction::*;
        let inputs = [
            r"########
...#....
...[]...
...[][].
....[]..
.....@..",
            r"########
........
...[]#..
...[][].
....[]..
.....@..",
        ];
        for input in inputs {
            let document = parse_input(input);
            let robot_pos = find_robot_pos(&document.warehouse);
            let sticked = stick_robot_to_boxes2(&document.warehouse, robot_pos, Up);
            let expected = vec![
                Pos(2, 3),
                Pos(2, 4),
                Pos(3, 3),
                Pos(3, 4),
                Pos(3, 6),
                Pos(3, 5),
                Pos(4, 4),
                Pos(4, 5),
                Pos(5, 5),
            ];
            assert_eq!(sticked, expected);
            assert!(!can_boxes_move_in_dir(&document.warehouse, &sticked, Up));
        }
    }

    #[test]
    fn test_stick_boxes_down() {
        use Direction::*;
        let input = r"########
...@....
...[]...
...[][].
....[]..
........";
        let document = parse_input(input);
        let robot_pos = find_robot_pos(&document.warehouse);
        let sticked = stick_robot_to_boxes2(&document.warehouse, robot_pos, Down);
        let expected = vec![
            Pos(4, 5),
            Pos(4, 4),
            Pos(3, 4),
            Pos(3, 3),
            Pos(2, 4),
            Pos(2, 3),
            Pos(1, 3),
        ];
        assert_eq!(sticked, expected);
    }

    #[test]
    fn test_stick_boxes_down_move_ok() {
        use Direction::*;
        let input = r"########
...@....
...[]...
...[][].
....[]..
........";
        let mut document = parse_input(input);
        let mut robot_pos = find_robot_pos(&document.warehouse);
        let sticked = stick_robot_to_boxes2(&document.warehouse, robot_pos, Down);
        let expected = vec![
            Pos(4, 5),
            Pos(4, 4),
            Pos(3, 4),
            Pos(3, 3),
            Pos(2, 4),
            Pos(2, 3),
            Pos(1, 3),
        ];
        assert_eq!(sticked, expected);
        assert!(can_boxes_move_in_dir(&document.warehouse, &sticked, Down));
        let expect = r"########
........
...@....
...[][].
...[]...
....[].."
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        move_robot_in_dir(&mut document.warehouse, &mut robot_pos, Down);
        assert_eq!(Warehouse(expect), document.warehouse);
    }

    #[test]
    fn test_stick_boxes_down_move_nok() {
        use Direction::*;
        let inputs = [
            r"########
...@....
...[]...
...[][].
....[]..
.....#..",
            r"########
...@....
...[]...
...[][].
...#[]..
........",
        ];
        for input in inputs {
            let document = parse_input(input);
            let robot_pos = find_robot_pos(&document.warehouse);
            let sticked = stick_robot_to_boxes2(&document.warehouse, robot_pos, Down);
            let expected = vec![
                Pos(4, 5),
                Pos(4, 4),
                Pos(3, 4),
                Pos(3, 3),
                Pos(2, 4),
                Pos(2, 3),
                Pos(1, 3),
            ];
            assert_eq!(sticked, expected);
            assert!(!can_boxes_move_in_dir(&document.warehouse, &sticked, Down));
        }
    }
}
