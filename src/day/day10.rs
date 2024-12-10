use crate::day::utils;
use glam::{IVec2, U8Vec2};
use itertools::Itertools;
use std::collections::HashMap;
use std::vec;

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

struct Trails {
    matrix: Vec<Vec<u8>>,
    trail_heads: Vec<U8Vec2>,
    peaks: Vec<U8Vec2>,
}

impl Trails {
    fn is_legal_move(&self, from: IVec2, delta: IVec2) -> bool {
        let target = from + delta;
        if target.x < 0 || target.y < 0 || target.y >= self.matrix.len() as i32 || target.x >= self.matrix[0].len() as i32  {
            return false;
        }

        let new_pos_topology = self.matrix[target.y as usize][target.x as usize];
        let current_post_topology = self.matrix[from.y as usize][from.x as usize];
        if new_pos_topology <= current_post_topology || new_pos_topology - current_post_topology != 1 {
            return false;
        }

        true
    }
}

fn parse_input(input_string: &str) -> Trails {
    let mut matrix = Vec::new();
    let mut trail_heads = Vec::new();
    let mut peaks = Vec::new();

    input_string.lines().enumerate()
        .for_each(|line| {
            matrix.push(line.1.chars().enumerate().into_iter()
                .map(|ch| {
                    let num = ch.1.to_digit(10).unwrap() as u8;
                    match num {
                        9 => { peaks.push(U8Vec2::new(ch.0 as u8, line.0 as u8)) }
                        0 => { trail_heads.push(U8Vec2::new(ch.0 as u8, line.0 as u8)) }
                        _ => { /* no-op */ }
                    }
                    num
                })
                .collect_vec())
        });

    Trails {matrix, trail_heads, peaks}
}

fn find_distinct_paths(trails: &Trails,
                       current_pos: U8Vec2,
                       current_path: &mut Vec<U8Vec2>,
                       reachable_peaks: &mut HashMap<U8Vec2, usize>)  {

    if trails.peaks.contains(&current_pos) {
        *reachable_peaks.entry(current_pos).or_insert(0) += 1;
        return;
    }

    for delta in MOVEMENT_DELTAS.iter() {
        if !trails.is_legal_move(current_pos.as_ivec2(), *delta) {
            continue;
        }

        let new_pos = (current_pos.as_ivec2() + *delta).as_u8vec2();
        current_path.push(new_pos);
        find_distinct_paths(trails, new_pos, current_path, reachable_peaks);
        current_path.pop();
    }
}

fn run_part_one(input_string: &str) -> usize {
    let trails  = parse_input(input_string);
    let mut trail_score_sum = 0;

    for trail_head in &trails.trail_heads {
        let mut reachable_peaks = HashMap::new();
        find_distinct_paths(&trails, *trail_head, &mut vec![*trail_head], &mut reachable_peaks);

        trail_score_sum += reachable_peaks.len();
    }

    trail_score_sum
}

fn run_part_two(input_string: &str) -> usize {
    let trails  = parse_input(input_string);
    let mut trail_rating_sum = 0;

    for trail_head in &trails.trail_heads {
        let mut reachable_peaks = HashMap::new();
        find_distinct_paths(&trails, *trail_head, &mut vec![*trail_head], &mut reachable_peaks);

        trail_rating_sum += reachable_peaks.iter().map(|x| x.1).sum::<usize>();
    }

    trail_rating_sum
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
        assert_eq!(run_part_two(&utils::read_input(10)), 1255);
    }
}
