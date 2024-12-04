#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
advent_of_code::solution!(4);

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let puzzle = parse_input(input);
    Some(find_word(&puzzle, "XMAS"))
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;
    let puzzle = parse_input(input);
    for row in 0..puzzle.len() {
        for col in 0..puzzle[0].len() {
            if search_x_mas(&puzzle, row, col).is_some() {
                count += 1;
            }
        }
    }

    Some(count)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut puzzle = Vec::new();

    for line in input.lines() {
        let mut row = Vec::with_capacity(line.len());
        for c in line.chars() {
            row.push(c);
        }
        puzzle.push(row);
    }

    puzzle
}

// Function to search in a specific direction
fn search_x_mas(grid: &[Vec<char>], start_row: usize, start_col: usize) -> Option<()> {
    let directions = [
        (1, 1),   // Down-Right
        (1, -1),  // Down-Left
        (-1, 1),  // Up-Right
        (-1, -1), // Up-Left
    ];

    if grid[start_row][start_col] != 'A' {
        return None;
    }

    // look into 4 diagonal directions if they are out of bounds
    for dir in directions {
        let row = isize::try_from(start_row).ok()? + dir.0;
        let col = isize::try_from(start_col).ok()? + dir.1;
        if row < 0
            || col < 0
            || row >= isize::try_from(grid.len()).ok()?
            || col >= isize::try_from(grid[0].len()).ok()?
        {
            return None;
        }
    }

    // reset the row, col
    let row = start_row;
    let col = start_col;

    // compute the X endpoints
    let down_right = grid[row + 1][col + 1];
    let up_left = grid[row - 1][col - 1];
    let up_right = grid[row - 1][col + 1];
    let down_left = grid[row + 1][col - 1];

    // look now for each direction to form MAS/SAM
    if ((up_left, down_right) == ('M', 'S') || (up_left, down_right) == ('S', 'M'))
        && ((up_right, down_left) == ('M', 'S') || (up_right, down_left) == ('S', 'M'))
    {
        return Some(());
    }

    None
}

// Function to search in a specific direction
fn search_direction(
    grid: &[Vec<char>],
    word: &str,
    start_row: usize,
    start_col: usize,
    row_step: isize,
    col_step: isize,
) -> Option<()> {
    let mut row = isize::try_from(start_row).ok()?;
    let mut col = isize::try_from(start_col).ok()?;
    for ch in word.chars() {
        if row < 0
            || col < 0
            || row >= isize::try_from(grid.len()).ok()?
            || col >= isize::try_from(grid[0].len()).ok()?
        {
            return None;
        }
        if grid[usize::try_from(row).ok()?][usize::try_from(col).ok()?] != ch {
            return None;
        }
        row += row_step;
        col += col_step;
    }
    Some(())
}

fn find_word(grid: &[Vec<char>], word: &str) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();

    // Directions
    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Down-Right
        (1, -1),  // Down-Left
        (-1, 1),  // Up-Right
        (-1, -1), // Up-Left
    ];

    // Check for word
    let mut count = 0;
    for row in 1..rows {
        for col in 0..cols {
            for &(row_step, col_step) in &directions {
                if search_direction(grid, word, row, col, row_step, col_step).is_some() {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
