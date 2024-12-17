#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
advent_of_code::solution!(10);

struct TopographicMap(Vec<Vec<i32>>, Vec<(usize, usize)>, Vec<(usize, usize)>);

fn parse_input(input: &str) -> TopographicMap {
    let mut map = Vec::new();
    let mut trail_heads = Vec::new();
    let mut trail_ends = Vec::new();
    for (idx, r) in input.lines().enumerate() {
        map.push(
            r.chars()
                .enumerate()
                .filter_map(|(cidx, c)| {
                    if c == '0' {
                        trail_heads.push((idx, cidx));
                    } else if c == '9' {
                        trail_ends.push((idx, cidx));
                    }
                    if c == '.' {
                        Some(-1)
                    } else {
                        char::to_digit(c, 10).and_then(|d| i32::try_from(d).ok())
                    }
                })
                .collect(),
        );
    }

    TopographicMap(map, trail_heads, trail_ends)
}

fn find_paths(
    map: &[Vec<i32>],
    trail_head: (usize, usize),
    trail_end: (usize, usize),
    path: &mut Vec<(usize, usize)>,
    visited: &mut Vec<Vec<bool>>,
    paths: &mut Vec<Vec<(usize, usize)>>,
) {
    // If current position is the end point, save the path
    if trail_head == trail_end {
        paths.push(path.clone());
        return;
    }

    // Directions: right, down, left, up
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (dx, dy) in directions {
        let new_x = isize::try_from(trail_head.0).unwrap_or(0) + dx;
        let new_y = isize::try_from(trail_head.1).unwrap_or(0) + dy;
        if new_x < 0 || new_y < 0 {
            continue;
        }

        let (new_x, new_y) = (
            usize::try_from(new_x).unwrap_or(0),
            usize::try_from(new_y).unwrap_or(0),
        );

        if new_x < map.len()
            && new_y < map[0].len()
            && !visited[new_x][new_y]
            && map[new_x][new_y] != -1
            && (map[new_x][new_y] - map[trail_head.0][trail_head.1]).abs() == 1
            && map[new_x][new_y] > map[trail_head.0][trail_head.1]
        {
            // Mark as visited and add to path
            visited[new_x][new_y] = true;
            path.push((new_x, new_y));

            // Recurse
            find_paths(map, (new_x, new_y), trail_end, path, visited, paths);

            // Backtrack
            path.pop();
            visited[new_x][new_y] = false;
        }
    }
}

fn find_all_trails(
    map: &[Vec<i32>],
    trail_head: &[(usize, usize)],
    trail_end: &[(usize, usize)],
) -> Vec<Vec<Vec<(usize, usize)>>> {
    let mut all_trails = Vec::new();
    for &s in trail_head {
        for &e in trail_end {
            let mut visited = vec![vec![false; map[0].len()]; map.len()];
            let mut path = vec![s];
            visited[s.0][s.1] = true;
            let mut results = Vec::new();
            find_paths(map, s, e, &mut path, &mut visited, &mut results);
            all_trails.push(results);
        }
    }
    all_trails
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let TopographicMap(map, trail_head, trail_end) = parse_input(input);
    Some(
        find_all_trails(&map, &trail_head, &trail_end)
            .into_iter()
            .filter(|p| !p.is_empty())
            .count(),
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    let TopographicMap(map, trail_head, trail_end) = parse_input(input);
    Some(
        find_all_trails(&map, &trail_head, &trail_end)
            .into_iter()
            .filter(|p| !p.is_empty())
            .map(|p| p.len())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
