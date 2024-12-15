use std::collections::{HashMap, HashSet};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let disk_map = input
        .trim_end()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    let mut blocks = get_blocks_from_disk_map(&disk_map)?;
    Some(compute_checksum(&mut blocks))
}

fn get_blocks_from_disk_map(disk_map: &[u32]) -> Option<Vec<i32>> {
    let mut blocks = Vec::new();
    let mut i = 0;
    for (idx, d) in disk_map.iter().enumerate() {
        if idx % 2 != 0 {
            blocks.extend(std::iter::repeat_n(-1, *d as usize));
        } else {
            blocks.extend(std::iter::repeat_n(i, *d as usize));
            i += 1;
        }
    }

    Some(blocks)
}

fn get_last_block_idx<F>(blocks: &[i32], f: F) -> usize
where
    F: Fn(&i32) -> bool,
{
    let mut last_block_idx = blocks.len() - 1;
    let mut block = blocks[last_block_idx];
    while last_block_idx > 1 && !f(&block) {
        last_block_idx -= 1;
        block = blocks[last_block_idx];
    }

    last_block_idx
}

fn count_spaces_at_the_end(blocks: &[i32]) -> usize {
    let mut curr = blocks[blocks.len() - 1];
    let mut count = 0;
    while curr == -1 {
        count += 1;
        curr = blocks[blocks.len() - 1];
    }

    count
}

fn compute_checksum(blocks: &mut [i32]) -> u64 {
    // get all the digits positions
    let mut digits_pos = Vec::new();
    for (idx, c) in blocks.iter().enumerate() {
        if *c != -1 {
            digits_pos.push(idx);
        }
    }
    let mut all_positions = blocks
        .iter()
        .enumerate()
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();
    let no_free_spaces = blocks.len() - digits_pos.len();
    let mut moved_spaces = count_spaces_at_the_end(blocks);

    for idx in 0..blocks.len() {
        if moved_spaces == no_free_spaces {
            break;
        }
        if blocks[idx] == -1 {
            loop {
                let pos = all_positions.pop().unwrap_or(0);
                if blocks[pos] == -1 {
                    moved_spaces += 1;
                    continue;
                } else {
                    blocks.swap(idx, pos);
                    moved_spaces += 1;
                    break;
                }
            }
        }
    }
    blocks
        .iter()
        .enumerate()
        .filter(|(_, d)| **d >= 0)
        .map(|(idx, d)| idx as u64 * *d as u64)
        .sum()
}

fn compute_checksum_whole_file(blocks: &mut [i32]) -> u64 {
    // get all the digits positions
    let mut file_ids_list = Vec::new();
    let mut file_ids = Vec::new();
    for (idx, c) in blocks.iter().enumerate() {
        if *c != -1 {
            // check to see if the file id has changed
            if !file_ids.is_empty() && blocks[idx - 1] != *c {
                file_ids_list.push(file_ids);
                file_ids = Vec::new();
            }
            file_ids.push(idx);
        } else if !file_ids.is_empty() {
            file_ids_list.push(file_ids);
            file_ids = Vec::new();
        }
    }

    if !file_ids.is_empty() {
        file_ids_list.push(file_ids);
    }
    let mut free_space_list = Vec::new();
    let mut free_space = Vec::new();
    for (idx, c) in blocks.iter().enumerate() {
        if *c == -1 {
            free_space.push(idx);
        } else if !free_space.is_empty() {
            free_space_list.push(free_space);
            free_space = Vec::new();
        }
    }

    if !free_space.is_empty() {
        free_space_list.push(free_space);
    }
    // 000000000011111111112222222222333333333344
    // 012345678901234567890123456789012345678901
    // 0..111....22222
    // 00...111...2...333.44.5555.6666.777.888899
    // 00992111777.44.333....5555.6666.....8888..
    // iterate over file ids list from end to begining
    // 0000000000111111111122222222223333333333444444444455555555556666666666777777777788888888889999999999
    // 0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789
    // 00...11..22.334455.....66.....77........88........991010111112121313.......1414.......1515..1616.1717.1818.
    let mut idx_free = 0;
    for (idx_file, id_list) in file_ids_list.iter().enumerate().rev() {
        let mut compacted: Option<(usize, usize)> = None;
        for (idx_free, free_list) in free_space_list.iter_mut().enumerate() {
            if !free_list.is_empty() && free_list[0] > id_list[0] {
                break;
            }
            // dbg!(&id_list);
            // dbg!(&free_list);
            if id_list.len() <= free_list.len() {
                // enough room to move whole file
                for (&if1, &if2) in id_list.iter().zip(free_list.iter()) {
                    // dbg!(if1, if2);
                    blocks.swap(if1, if2);
                }
                compacted = Some((idx_file, idx_free));
                break;
            } else {
                // go to the next free space block available
                continue;
            }
        }
        if let Some((idx_list, idx_free)) = compacted {
            free_space_list[idx_free] =
                free_space_list[idx_free][file_ids_list[idx_list].len()..].to_vec();
        }
        // not enough room, don't move the file, and go to the next one
    }
    blocks
        .iter()
        .enumerate()
        .filter(|(_, d)| **d >= 0)
        .map(|(idx, d)| idx as u64 * *d as u64)
        .sum()
}

pub fn part_two(input: &str) -> Option<u64> {
    let disk_map = input
        .trim_end()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    let mut blocks = get_blocks_from_disk_map(&disk_map)?;

    Some(compute_checksum_whole_file(&mut blocks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
