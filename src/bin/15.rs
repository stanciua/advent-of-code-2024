advent_of_code::solution!(15);
use std::{
    collections::{HashMap, HashSet, VecDeque},
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

// 00000000001111111111
// 01234567890123456789
//0####################
//1##[].......[].[][]##
//2##[]...........[].##
//3##[]........[][][]##
//4##[]......[]...@[]##
//5##..##......[]....##
//6##..[]............##
//7##.........[].[][]##
//8##......[][]..[]..##
//9####################";
fn stick_robot_to_boxes2(warehouse: &Warehouse, pos: Pos, dir: Direction) -> Vec<Pos> {
    use Direction::*;
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
                }
            }
        }
        Left => {
            let mut next = pos + DIRECTIONS[Left as usize];
            while warehouse[next] == '[' || warehouse[next] == ']' {
                sticked.push(next);
                next = next + DIRECTIONS[Left as usize];
            }
        }
        Right => {
            let mut next = pos + DIRECTIONS[Right as usize];
            while warehouse[next] == '[' || warehouse[next] == ']' {
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
fn stick_robot_to_boxes(warehouse: &Warehouse, pos: Pos, dir: Direction) -> Vec<Pos> {
    use Direction::*;
    // the robot included is sticked to the a list of boxes
    let mut sticked = Vec::from([pos]);
    // 00000000001111111111
    // 01234567890123456789
    //0####################
    //1##[].......[].[][]##
    //2##[]...........[].##
    //3##[]........[][][]##
    //4##[]......[]...@[]##
    //5##..##......[]....##
    //6##..[]............##
    //7##.........[].[][]##
    //8##......[][]..[]..##
    //9####################";
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

fn move_robot(document: &mut Document, pos: Pos, double: bool) {
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
    }
}

fn move_robot_in_dir(warehouse: &mut Warehouse, pos: Pos, dir: Direction) {
    let sticked = stick_robot_to_boxes2(warehouse, pos, dir);

    if can_boxes_move_in_dir(warehouse, &sticked, dir) {
        // we are able to move in this direction, so move all sticked boxes if any and the
        // robot
        for &p in &sticked {
            let next = p + DIRECTIONS[dir as usize];
            dbg!(p);
            dbg!(next);
            dbg!("before", warehouse[p]);
            dbg!("before", warehouse[next]);
            warehouse[next] = warehouse[p];
            warehouse[p] = '.';
            dbg!("after", warehouse[p]);
            dbg!("after", warehouse[next]);
        }
    }
}
fn move_robot2(document: &mut Document, pos: Pos) {
    for &dir in &document.moves {
        move_robot_in_dir(&mut document.warehouse, pos, dir);
        dbg!(dir);
        display(&document.warehouse);
    }
}

fn can_boxes_move_in_dir(warehouse: &Warehouse, positions: &Vec<Pos>, dir: Direction) -> bool {
    use Direction::*;
    match dir {
        Up => {
            for &pos in positions {
                let next = pos + DIRECTIONS[Up as usize];
                let next_tile = warehouse[next];
                if next_tile == '#' {
                    return false;
                }
            }
        }
        Down => {
            for &pos in positions {
                let next = pos + DIRECTIONS[Down as usize];
                let next_tile = warehouse[next];
                if next_tile == '#' {
                    return false;
                }
            }
        }

        Left => {
            let fbox_left = positions[0];
            let next_left = fbox_left + DIRECTIONS[Left as usize];
            if warehouse[next_left] == '#' {
                return false;
            }
            if positions.len() > 1 {
                let fbox_right = positions[1];
                let next_right = fbox_right + DIRECTIONS[Left as usize];
                if warehouse[next_right] == '#' {
                    return false;
                }
            }
        }

        Right => {
            let fbox_left = positions[0];
            let next_left = fbox_left + DIRECTIONS[Right as usize];
            if warehouse[next_left] == '#' {
                return false;
            }
            if positions.len() > 1 {
                let fbox_right = positions[1];
                let next_right = fbox_right + DIRECTIONS[Right as usize];
                if warehouse[next_right] == '#' {
                    return false;
                }
            }
        }
    }

    true
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

    move_robot(&mut document, robot_pos, false);
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
    let robot_pos = find_robot_pos(&document.warehouse);

    move_robot2(&mut document, robot_pos);
    Some(0)
}

fn find_robot_pos(warehouse: &Warehouse) -> Pos {
    let rows = warehouse.0.len();
    let cols = warehouse.0[0].len();
    for i in 0..rows {
        for j in 0..cols {
            if warehouse[Pos(i as i32, j as i32)] == '@' {
                return Pos(i as i32, j as i32);
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
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
        move_robot_in_dir(&mut document.warehouse, robot_pos, Up);

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

        move_robot_in_dir(&mut document.warehouse, robot_pos, Down);
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
