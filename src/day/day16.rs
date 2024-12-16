use crate::day::utils;
use glam::IVec2;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

pub fn run() {
    let input_string = utils::read_input(16);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    cost: usize,
    coords: IVec2,
    direction: IVec2,
    path: Vec<IVec2>,
}

// Make Rust std max-heap implementation into min heap instead
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn run_part_one(input_string: &str) -> usize {
    search(input_string).0
}

fn run_part_two(input_string: &str) -> usize {
    search(input_string).1.into_iter()
         .flatten()
         .unique()
         .collect_vec()
         .len()
}

fn search(input_string: &str) -> (usize, Vec<Vec<IVec2>>) {
    let mut start_pos = Default::default();
    let mut end_pos = Default::default();
    let starting_delta = IVec2::from((1, 0));

    let deltas = vec![IVec2::from((0, -1)), IVec2::from((1, 0)), IVec2::from((0, 1)), IVec2::from((-1, 0))];

    let map = input_string.lines().enumerate()
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

    let mut min_heap = BinaryHeap::new();
    min_heap.push(State {cost: 0, coords: start_pos, direction: starting_delta, path: vec![start_pos]});

    let mut costs: HashMap<(IVec2, IVec2), usize> = HashMap::new();
    costs.insert((start_pos, starting_delta), 0);

    let mut min_cost = usize::MAX;
    let mut best_paths = Vec::new();

    while let Some(state) = min_heap.pop() {
        if state.coords == end_pos {
            if state.cost < min_cost {
                min_cost = state.cost;
                best_paths.clear();
            }

            if state.cost == min_cost {
                best_paths.push(state.path.clone());
            }

            continue;
        }

        if let Some(&min_cost) = costs.get(&(state.coords, state.direction)) {
            if state.cost > min_cost {
                continue;
            }
        }

        let x1 = deltas.iter().position(|n| *n == state.direction).unwrap() as isize;
        let new_directions = vec![deltas.get(x1 as usize).unwrap(), deltas.get((4 - (x1 + 1)) as usize).unwrap(), deltas.get(((4 - (x1 - 1)) % 4) as usize).unwrap()];

        for new_direction in new_directions {
            let new_pos = state.coords + new_direction;

            if map[new_pos.y as usize][new_pos.x as usize] == '#' {
                continue;
            }

            let turn_penalty = if state.direction != *new_direction { 1000 } else { 0 };
            let new_cost = state.cost + 1 + turn_penalty;

            if new_cost <= *costs.get(&(new_pos, *new_direction)).unwrap_or(&usize::MAX) {
                costs.insert((new_pos, *new_direction), new_cost);
                let mut new_path = state.path.clone();
                new_path.push(new_pos);
                min_heap.push(State {cost: new_cost, coords: new_pos, direction: *new_direction, path: new_path });
            }
        }
    }

    (min_cost, best_paths)
}

#[cfg(test)]
mod tests {
    use crate::day::day16::{run_part_one, run_part_two};
    use crate::day::utils;


    fn example_input() -> String {
        String::from("\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############")
    }

    fn example_input_2() -> String {
        String::from("\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 7036);
        assert_eq!(run_part_one(&example_input_2()), 11048);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(16)), 98520);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input()), 45);
        assert_eq!(run_part_two(&example_input_2()), 64);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(16)), 609);
    }
}
