use crate::day::utils;
use glam::IVec2;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() {
    let input_string = utils::read_input(20);
    println!("{}", run_part_one(&input_string, 100));
    println!("{}", run_part_two(&input_string, 100));
}

static MOVEMENT_DELTAS: [IVec2; 4] = [
    IVec2::new(0, 1),
    IVec2::new(1, 0),
    IVec2::new(0, -1),
    IVec2::new(-1, 0),
];

fn run_part_one(input_string: &str, skip_threshold: usize) -> usize {
    let mut start_pos = Default::default();
    let mut end_pos = Default::default();

    let grid = input_string.lines().enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().map(|(x, c)| {
                match c {
                    'S' => { start_pos = IVec2::new(x as i32, y as i32) },
                    'E' => { end_pos = IVec2::new(x as i32, y as i32) },
                    _ => {}
                }
                c
            }).collect_vec()
        }).collect_vec();

    let original_shortest_path = shortest_path_lenght(start_pos, &mut end_pos, &grid, usize::MAX).unwrap();

    let mut cheats: HashSet<IVec2> = HashSet::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            if *elem == '#' && x > 0 &&  y > 0 && x < row.len() - 1  && y < grid.len() - 1 {
                let current = IVec2::new(x as i32, y as i32);
                for delta in MOVEMENT_DELTAS {
                    let new_pos = current + delta;
                    if grid[new_pos.y as usize][new_pos.x as usize] == '.' || grid[new_pos.y as usize][new_pos.x as usize] == 'E'|| grid[new_pos.y as usize][new_pos.x as usize] == 'S' {
                        cheats.insert(current);
                    }
                }
            }
        }
    }

    let mut new_grid = grid.clone();

    let mut results = HashMap::new();
    for cheat in cheats {
        new_grid[cheat.y as usize][cheat.x as usize] = '.';
        if let Some(value) = shortest_path_lenght(start_pos, &mut end_pos, &new_grid, original_shortest_path) {
            *results.entry(original_shortest_path - value).or_insert(0) += 1;
        }
        new_grid[cheat.y as usize][cheat.x as usize] = grid[cheat.y as usize][cheat.x as usize];
    }

    results.iter()
        .filter(|(a, _)| **a >= skip_threshold)
        .map(|(_, b)| b)
        .sum()
}

fn shortest_path_lenght(start_pos: IVec2, end_pos: &mut IVec2, grid: &Vec<Vec<char>>, cutoff: usize) -> Option<usize> {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut deque = VecDeque::new();
    deque.push_back((start_pos, 0));
    visited[0][0] = true;

    while let Some((current_pos, steps)) = deque.pop_front() {
        if steps >= cutoff {
            continue;
        }
        if current_pos == *end_pos {
            return Some(steps);
        }

        for delta in MOVEMENT_DELTAS {
            let new_pos = current_pos + delta;
            if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= grid[0].len() as i32 || new_pos.y >= grid.len() as i32 {
                continue;
            }

            if visited[new_pos.y as usize][new_pos.x as usize] {
                continue;
            }

            if grid[new_pos.y as usize][new_pos.x as usize] == '#' {
                continue;
            }

            visited[new_pos.y as usize][new_pos.x as usize] = true;
            deque.push_back((new_pos, steps + 1));
        }
    }
    None
}

fn run_part_two(input_string: &str, skip_threshold: usize) -> usize {
    let mut start_pos = Default::default();
    let mut end_pos = Default::default();

    let grid = input_string.lines().enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().map(|(x, c)| {
                match c {
                    'S' => { start_pos = IVec2::new(x as i32, y as i32) },
                    'E' => { end_pos = IVec2::new(x as i32, y as i32) },
                    _ => {}
                }
                c
            }).collect_vec()
        }).collect_vec();

    let original_shortest_path = shortest_path(start_pos, &mut end_pos, &grid, usize::MAX);
    let mut skips = HashMap::new();

    for i in 0..original_shortest_path.len() - 1 {
        for j in i+1..original_shortest_path.len() {
            let start = original_shortest_path.get(i).unwrap();
            let end = original_shortest_path.get(j).unwrap();

            let distance = (start.x.abs_diff(end.x) + start.y.abs_diff(end.y)) as usize;
            if distance <= 20 {
                let new_path_length = original_shortest_path.len() - j + i + distance; // off by one?
                skips.entry(new_path_length).or_insert(HashSet::new()).insert((start, end));
            }
        }
    }

    let mut sum = 0;
    for skip in skips {
        if skip.0 <= original_shortest_path.len() - skip_threshold {
            sum += skip.1.len();
        }
    }

    sum
}

fn shortest_path(start_pos: IVec2, end_pos: &mut IVec2, grid: &Vec<Vec<char>>, cutoff: usize)
                 -> Vec<IVec2> {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut deque = VecDeque::new();
    deque.push_back((start_pos, vec![start_pos]));
    visited[0][0] = true;

    while let Some((current_pos, path)) = deque.pop_front() {
        if path.len() >= cutoff {
            continue;
        }
        if current_pos == *end_pos {
            return path;
        }

        for delta in MOVEMENT_DELTAS {
            let new_pos = current_pos + delta;
            if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= grid[0].len() as i32 || new_pos.y >= grid.len() as i32 {
                continue;
            }

            if visited[new_pos.y as usize][new_pos.x as usize] {
                continue;
            }

            if grid[new_pos.y as usize][new_pos.x as usize] == '#' {
                continue;
            }

            visited[new_pos.y as usize][new_pos.x as usize] = true;
            let mut vec1 = path.clone();
            vec1.push(new_pos);
            deque.push_back((new_pos, vec1));
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use crate::day::day20::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input(), 100), 0);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(20), 100), 1406);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input(), 50), 285);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(20), 100), 1006101);
    }

}
