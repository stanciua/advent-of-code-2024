#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

advent_of_code::solution!(9);

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    let disk_map = input
        .trim_end()
        .chars()
        .filter_map(|c| c.to_digit(10).map(|c| i32::try_from(c).ok()))
        .flatten()
        .collect::<Vec<_>>();
    let mut blocks = get_blocks_from_disk_map(&disk_map)?;
    Some(compute_checksum(&mut blocks))
}

fn get_blocks_from_disk_map(disk_map: &[i32]) -> Option<Vec<i32>> {
    let mut blocks = Vec::new();
    let mut i = 0;
    for (idx, d) in disk_map.iter().enumerate() {
        let times = usize::try_from(*d).ok()?;
        if idx % 2 != 0 {
            blocks.extend(std::iter::repeat_n(-1, times));
        } else {
            blocks.extend(std::iter::repeat_n(i, times));
            i += 1;
        }
    }

    Some(blocks)
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
    let mut moved_spaces = blocks.iter().rev().take_while(|&&c| c == -1).count();

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
                }
                blocks.swap(idx, pos);
                moved_spaces += 1;
                break;
            }
        }
    }
    blocks
        .iter()
        .enumerate()
        .filter(|&(_, &d)| d >= 0)
        .map(|(idx, &d)| idx as u64 * u64::try_from(d).unwrap_or(0))
        .sum()
}

fn compute_checksum_whole_file(blocks: &mut [i32]) -> u64 {
    let file_ids_list = blocks
        .iter()
        .enumerate()
        .collect::<Vec<_>>()
        .as_slice()
        .chunk_by(|a, b| a.1 == b.1)
        .filter(|chunk| *chunk[0].1 != -1)
        .map(|chunk| {
            chunk.iter()
                .map(|&(idx,& _)| idx)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut free_space_list = blocks
        .iter()
        .enumerate()
        .collect::<Vec<_>>()
        .as_slice()
        .chunk_by(|a, b| a.1 == b.1)
        .filter(|chunk| *chunk[0].1 == -1)
        .map(|chunk| {
            chunk.iter()
                .map(|&(idx,& _)| idx)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for (idx_file, id_list) in file_ids_list.iter().enumerate().rev() {
        let mut compacted: Option<(usize, usize)> = None;
        for (idx_free, free_list) in free_space_list.iter_mut().enumerate() {
            if !free_list.is_empty() && free_list[0] > id_list[0] {
                break;
            }
            if id_list.len() <= free_list.len() {
                // enough room to move whole file
                for (&if1, &if2) in id_list.iter().zip(free_list.iter()) {
                    blocks.swap(if1, if2);
                }
                compacted = Some((idx_file, idx_free));
                break;
            }
        }
        if let Some((idx_list, idx_free)) = compacted {
            // if file blocks have been compacted, shring the freespace available
            free_space_list[idx_free] =
                free_space_list[idx_free][file_ids_list[idx_list].len()..].to_vec();
        }
    }
    blocks
        .iter()
        .enumerate()
        .filter(|&(_, &d)| d >= 0)
        .map(|(idx, &d)| idx as u64 * u64::try_from(d).unwrap_or(0))
        .sum()
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    let disk_map = input
        .trim_end()
        .chars()
        .filter_map(|c| c.to_digit(10).map(|c| i32::try_from(c).ok()))
        .flatten()
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
