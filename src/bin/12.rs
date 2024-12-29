use std::collections::{BTreeMap, BTreeSet};

advent_of_code::solution!(12);

#[repr(usize)]
#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

use Direction::*;
const DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];

const COORDINATES: [(isize, isize); 4] = [
    (-1isize, 0), // up
    (1, 0),       // down
    (0, -1isize), // left
    (0, 1),       // right
];

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut garden = Vec::new();
    for line in input.lines() {
        garden.push(line.chars().collect());
    }

    garden
}

fn flood_fill(
    garden: &[Vec<char>],
    garden_plot: &mut BTreeSet<(usize, usize)>,
    (x, y): (usize, usize),
    garden_plot_type: char,
) {
    let rows = garden.len();
    let cols = garden[0].len();

    // if the plot is already in the garden plot, return
    if garden_plot.contains(&(x, y)) {
        return;
    }

    // Helper function for recursive filling
    fn fill(
        grid: &[Vec<char>],
        garden_plot: &mut BTreeSet<(usize, usize)>,
        (x, y): (usize, usize),
        rows: usize,
        cols: usize,
        garden_plot_type: char,
    ) {
        // Boundary check
        if x >= rows || y >= cols || grid[x][y] != garden_plot_type || garden_plot.contains(&(x, y))
        {
            return;
        }

        garden_plot.insert((x, y));

        // Recursive calls for 4 directions
        if x > 0 {
            fill(grid, garden_plot, (x - 1, y), rows, cols, garden_plot_type); // Up
        }
        if x < rows - 1 {
            fill(grid, garden_plot, (x + 1, y), rows, cols, garden_plot_type); // Down
        }
        if y > 0 {
            fill(grid, garden_plot, (x, y - 1), rows, cols, garden_plot_type); // Left
        }
        if y < cols - 1 {
            fill(grid, garden_plot, (x, y + 1), rows, cols, garden_plot_type); // Right
        }
    }

    // Start the flood fill from the given position
    fill(garden, garden_plot, (x, y), rows, cols, garden_plot_type)
}

fn get_garden_plot_sides(
    (x, y): (usize, usize),
    garden: &[Vec<char>],
    plot_sides_per_dir: &mut BTreeMap<(usize, usize), BTreeSet<Direction>>,
) {
    plot_sides_per_dir.entry((x, y)).or_default();
    for dir in DIRECTIONS {
        let (dirx, diry) = COORDINATES[dir as usize];
        let dx = x as isize + dirx;
        let dy = y as isize + diry;
        if dx < 0 || dx > garden.len() as isize - 1 || dy < 0 || dy > garden[x].len() as isize - 1 {
            plot_sides_per_dir
                .entry((x, y))
                .and_modify(|ps: &mut BTreeSet<Direction>| {
                    ps.insert(dir);
                })
                .or_insert(BTreeSet::from([dir]));
            continue;
        }

        // if the neighbor garden plot is different, count it
        if garden[dx as usize][dy as usize] != garden[x][y] {
            plot_sides_per_dir
                .entry((x, y))
                .and_modify(|ps: &mut BTreeSet<Direction>| {
                    ps.insert(dir);
                })
                .or_insert(BTreeSet::from([dir]));
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let garden = parse_input(input);
    let garden_plot_type_map = get_garden_regions_per_pos(&garden);

    let mut total_price = 0;
    for garden_plot in garden_plot_type_map.values() {
        let mut plot_sides = BTreeMap::new();
        garden_plot.iter().for_each(|&p| {
            get_garden_plot_sides(p, &garden, &mut plot_sides);
        });
        let mut perimeter = 0;

        for p in garden_plot {
            if let Some(sides) = plot_sides.get(p) {
                perimeter += sides.len();
            }
        }
        total_price += garden_plot.len() * perimeter;
    }

    Some(total_price)
}

pub fn part_two(input: &str) -> Option<usize> {
    let garden = parse_input(input);
    let garden_plot_type_map = get_garden_regions_per_pos(&garden);

    let mut total_price = 0;
    for garden_plot in garden_plot_type_map.values() {
        let GetPlotsOnSameLineColumnResult(plots_same_line, plots_same_column) =
            get_plots_on_same_line_column(garden_plot);

        let mut plot_sides_per_dir = BTreeMap::new();
        garden_plot.iter().for_each(|&p| {
            get_garden_plot_sides(p, &garden, &mut plot_sides_per_dir);
        });

        let mut perimeter = compute_no_of_sides(
            &mut plot_sides_per_dir,
            &plots_same_line,
            &plots_same_column,
        );

        for p in garden_plot {
            if let Some(sides) = plot_sides_per_dir.get(p) {
                perimeter += sides.len();
            }
        }
        total_price += garden_plot.len() * perimeter;
    }

    Some(total_price)
}

struct GetPlotsOnSameLineColumnResult(Vec<Vec<(usize, usize)>>, Vec<Vec<(usize, usize)>>);

fn get_plots_on_same_line_column(
    garden_plot: &BTreeSet<(usize, usize)>,
) -> GetPlotsOnSameLineColumnResult {
    let mut same_line: Vec<(usize, usize)> = Vec::from_iter(garden_plot.iter().cloned());
    same_line.sort_by(|a, b| a.0.cmp(&b.0));
    let same_line = same_line
        .chunk_by(|a, b| a.0 == b.0 && (a.1 as isize - b.1 as isize).abs() == 1)
        .filter(|chunk| chunk.len() > 1)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();
    let mut same_column: Vec<(usize, usize)> = Vec::from_iter(garden_plot.iter().cloned());
    same_column.sort_by(|a, b| a.1.cmp(&b.1));
    let same_column = same_column
        .chunk_by(|a, b| a.1 == b.1 && (a.0 as isize - b.0 as isize).abs() == 1)
        .filter(|chunk| chunk.len() > 1)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();
    GetPlotsOnSameLineColumnResult(same_line, same_column)
}

fn get_garden_regions_per_pos(
    garden: &[Vec<char>],
) -> BTreeMap<(usize, usize), BTreeSet<(usize, usize)>> {
    let mut garden_plot_type_map = BTreeMap::new();
    let mut all_garden_plots = BTreeSet::new();
    for x in 0..garden.len() {
        for y in 0..garden[x].len() {
            let mut garden_plot = BTreeSet::new();
            if all_garden_plots.contains(&(x, y)) {
                continue;
            }
            flood_fill(garden, &mut garden_plot, (x, y), garden[x][y]);
            all_garden_plots.extend(garden_plot.iter().cloned());
            garden_plot_type_map.insert((x, y), garden_plot);
        }
    }

    garden_plot_type_map
}

fn compute_no_of_sides(
    plot_sides_per_dir: &mut BTreeMap<(usize, usize), BTreeSet<Direction>>,
    same_line: &[Vec<(usize, usize)>],
    same_column: &[Vec<(usize, usize)>],
) -> usize {
    let mut no_of_sides = 0;

    for partition in same_line.iter() {
        no_of_sides += count_no_of_sides_per_dir(&Up, plot_sides_per_dir, partition);
        no_of_sides += count_no_of_sides_per_dir(&Down, plot_sides_per_dir, partition);
    }

    for partition in same_column {
        if partition.len() < 2 {
            continue;
        }
        no_of_sides += count_no_of_sides_per_dir(&Left, plot_sides_per_dir, partition);
        no_of_sides += count_no_of_sides_per_dir(&Right, plot_sides_per_dir, partition);
    }

    no_of_sides
}

fn count_no_of_sides_per_dir(
    dir: &Direction,
    plot_sides: &mut BTreeMap<(usize, usize), BTreeSet<Direction>>,
    partition: &[(usize, usize)],
) -> usize {
    let partitions = partition
        .chunk_by(|a, b| plot_sides[a].contains(dir) && plot_sides[b].contains(dir))
        .filter(|p| p.len() > 1)
        .collect::<Vec<_>>();
    let no_of_sides = partitions.len();

    partitions.into_iter().for_each(|partition| {
        partition.iter().for_each(|&p| {
            plot_sides.entry(p).and_modify(|sides| {
                sides.remove(dir);
            });
        });
    });

    no_of_sides
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
