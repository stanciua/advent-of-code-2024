advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle = parse_input(input);

    Some(find_words(&puzzle, "XMAS"))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;
    let puzzle = parse_input(input);
    let rows = puzzle.len();
    let cols = puzzle[0].len();
    for row in 0..rows {
        for col in 0..cols {
            if search_x_mas(&puzzle, row, col) {
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
fn search_x_mas(grid: &[Vec<char>], start_row: usize, start_col: usize) -> bool {
    let directions = [
        (1, 1),   // Down-Right
        (1, -1),  // Down-Left
        (-1, 1),  // Up-Right
        (-1, -1), // Up-Left
    ];

    if grid[start_row][start_col] != 'A' {
        return false;
    }

    // look into 4 diagonal directions if they are out of bounds
    for dir in directions {
        let mut row = start_row as isize;
        let mut col = start_col as isize;
        row += dir.0;
        col += dir.1;
        if row < 0 || col < 0 || row >= grid.len() as isize || col >= grid[0].len() as isize {
            return false;
        }
    }

    let row = start_row;
    let col = start_col;

    let down_right = grid[row + 1][col + 1];
    let up_left = grid[row - 1][col - 1];
    let up_right = grid[row - 1][col + 1];
    let down_left = grid[row + 1][col - 1];

    // look now for each direction to form MAS/SAM
    if ((up_left == 'M' && down_right == 'S') || (up_left == 'S' && down_right == 'M'))
        && ((up_right == 'M' && down_left == 'S') || (up_right == 'S' && down_left == 'M'))
    {
        return true;
    }

    false
}

// Function to search in a specific direction
fn search_direction(
    grid: &[Vec<char>],
    word: &str,
    start_row: usize,
    start_col: usize,
    row_step: isize,
    col_step: isize,
) -> bool {
    let mut row = start_row as isize;
    let mut col = start_col as isize;
    for ch in word.chars() {
        if row < 0 || col < 0 || row >= grid.len() as isize || col >= grid[0].len() as isize {
            return false;
        }
        if grid[row as usize][col as usize] != ch {
            return false;
        }
        row += row_step;
        col += col_step;
    }
    true
}

fn find_words(grid: &[Vec<char>], word: &str) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();

    // Directions: (row_step, col_step)
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

    // Check for each word
    let mut count = 0;
    for row in 1..rows {
        for col in 0..cols {
            for &(row_step, col_step) in &directions {
                if search_direction(grid, word, row, col, row_step, col_step) {
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
