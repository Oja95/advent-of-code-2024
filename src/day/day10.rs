use std::collections::HashSet;
use std::ops::Add;
use std::vec;
use glam::{IVec2, U8Vec2, UVec2};
use itertools::Itertools;
use crate::day::utils;
use crate::day::utils::Matrix;

pub fn run() {
    let input_string = utils::read_input(10);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}

static MOVEMENT_DELTAS: [IVec2; 4] = [
    IVec2::new(0, 1),
    IVec2::new(1, 0),
    IVec2::new(0, -1),
    IVec2::new(-1, 0),
];

fn run_part_one(input_string: &str) -> usize {
    let mut matrix = Vec::new();

    input_string.lines()
        .for_each(|line| {
            matrix.push(line.chars().into_iter()
                    .map(|ch| ch.to_digit(10).or(Some(100)).unwrap() as u8)
                    .collect_vec())
        });

    let mut trail_heads = Vec::new();
    let mut peaks = Vec::new();



    for (y, row) in matrix.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            match col {
                &9 => {peaks.push(U8Vec2::new(x as u8, y as u8))}
                &0 => {trail_heads.push(U8Vec2::new(x as u8, y as u8))}
                &_ => { /* no-op */ }
            }
        }
    }

    let mut trail_score_sum = 0;

    // find distinct paths count from each trailhead to peaks, using backtracking perhaps?
    for trail_head in trail_heads {
        let mut reachable_peaks = HashSet::new();
        let mut current_path = vec![trail_head];
        let mut visited = vec![trail_head];
        find_distinct_paths(&matrix, trail_head, &peaks, &mut current_path, &mut reachable_peaks, &mut visited);

        trail_score_sum += reachable_peaks.len();
    }


    trail_score_sum
}

fn find_distinct_paths(matrix: &Vec<Vec<u8>>,
                       current_pos: U8Vec2,
                       peaks: &Vec<U8Vec2>,
                       current_path: &mut Vec<U8Vec2>,
                       reachable_peaks: &mut HashSet<U8Vec2>,
                       visited: &mut Vec<U8Vec2>)  {

    if peaks.contains(&current_pos) {
        reachable_peaks.insert(current_pos);
        return;
    }

    for delta in MOVEMENT_DELTAS.iter() {
        // for each direction, check if can be traversed (diff +- 1 and within bounds and ahsnt
        // been visited before?)
        let vec2 = current_pos.as_ivec2();
        let new_pos_candidate = vec2 + delta;

        // out of bounds
        if new_pos_candidate.x < 0 || new_pos_candidate.y < 0 || new_pos_candidate.y >= matrix.len() as i32 || new_pos_candidate.x >= matrix[0].len() as i32  {
            continue;
        }

        let new_pos = new_pos_candidate.as_u8vec2();
        if visited.contains(&new_pos) {
            continue;
        }

        // Next step must be exactly 1 unit higher!
        let new_pos_topology = matrix[new_pos.y as usize][new_pos.x as usize];
        let current_post_topology = matrix[current_pos.y as usize][current_pos.x as usize];
        if new_pos_topology <= current_post_topology || new_pos_topology - current_post_topology != 1 {
            continue;
        }

        visited.push(new_pos);
        current_path.push(new_pos);
        find_distinct_paths(matrix, new_pos, peaks, current_path, reachable_peaks, visited);
        current_path.pop();
        visited.pop();
    }
}

fn run_part_two(input_string: &str) -> usize {
    let mut matrix = Vec::new();

    input_string.lines()
        .for_each(|line| {
            matrix.push(line.chars().into_iter()
                .map(|ch| ch.to_digit(10).or(Some(100)).unwrap() as u8)
                .collect_vec())
        });

    let mut trail_heads = Vec::new();
    let mut peaks = Vec::new();



    for (y, row) in matrix.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            match col {
                &9 => {peaks.push(U8Vec2::new(x as u8, y as u8))}
                &0 => {trail_heads.push(U8Vec2::new(x as u8, y as u8))}
                &_ => { /* no-op */ }
            }
        }
    }

    let mut trail_score_sum = 0;

    // find distinct paths count from each trailhead to peaks, using backtracking perhaps?
    for trail_head in trail_heads {
        let mut paths = HashSet::new();
        let mut current_path = vec![trail_head];
        let mut visited = vec![trail_head];
        find_distinct_paths_2(&matrix, trail_head, &peaks, &mut current_path, &mut paths, &mut
            visited);

        trail_score_sum += paths.len();
    }


    trail_score_sum
}

fn find_distinct_paths_2(matrix: &Vec<Vec<u8>>,
                       current_pos: U8Vec2,
                       peaks: &Vec<U8Vec2>,
                       current_path: &mut Vec<U8Vec2>,
                       paths: &mut HashSet<Vec<U8Vec2>>,
                       visited: &mut Vec<U8Vec2>)  {

    if peaks.contains(&current_pos) {
        paths.insert(current_path.clone());
        return;
    }

    for delta in MOVEMENT_DELTAS.iter() {
        // for each direction, check if can be traversed (diff +- 1 and within bounds and ahsnt
        // been visited before?)
        let vec2 = current_pos.as_ivec2();
        let new_pos_candidate = vec2 + delta;

        // out of bounds
        if new_pos_candidate.x < 0 || new_pos_candidate.y < 0 || new_pos_candidate.y >= matrix.len() as i32 || new_pos_candidate.x >= matrix[0].len() as i32  {
            continue;
        }

        let new_pos = new_pos_candidate.as_u8vec2();
        if visited.contains(&new_pos) {
            continue;
        }

        // Next step must be exactly 1 unit higher!
        let new_pos_topology = matrix[new_pos.y as usize][new_pos.x as usize];
        let current_post_topology = matrix[current_pos.y as usize][current_pos.x as usize];
        if new_pos_topology <= current_post_topology || new_pos_topology - current_post_topology != 1 {
            continue;
        }

        visited.push(new_pos);
        current_path.push(new_pos);
        find_distinct_paths_2(matrix, new_pos, peaks, current_path, paths, visited);
        current_path.pop();
        visited.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::day::day10::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732")
    }


    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 36);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(10)), 550);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input()), 81);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(10)), 6467290479134);
    }
}
