use crate::day::utils;
use glam::{quat, IVec2};
use itertools::Itertools;
use std::collections::VecDeque;

pub fn run() {
    let input_string = utils::read_input(18);
    println!("{}", run_part_one(&input_string, 71, 1024));
    println!("{}", run_part_two(&input_string, 71, 1024));
}

static MOVEMENT_DELTAS: [IVec2; 4] = [
    IVec2::new(0, 1),
    IVec2::new(1, 0),
    IVec2::new(0, -1),
    IVec2::new(-1, 0),
];

fn run_part_one(input_string: &str, grid_size: usize, max_bytes: usize) -> usize {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; grid_size]; grid_size];

    let mut current_bytes = 0;

    input_string.lines()
        .for_each(|line| {
            if current_bytes < max_bytes {
                let mut split = line.split(',');
                let x = split.next().unwrap().parse::<usize>().unwrap();
                let y = split.next().unwrap().parse::<usize>().unwrap();
                grid[y][x] = '#';
                current_bytes += 1;
            }
        });

    let start_pos = IVec2 { x: 0, y: 0 };
    let end_pos = IVec2 { x: (grid_size - 1) as i32, y: (grid_size - 1) as i32 };

    // bfs
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid_size]; grid_size];
    let mut deque = VecDeque::new();
    deque.push_back((start_pos, 0));
    visited[0][0] = true;


    while let Some((current_pos, steps)) = deque.pop_front() {

        if current_pos == end_pos {
            return steps;
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

    panic!("no path found");
}

fn run_part_two(input_string: &str, grid_size: usize, known_safe_bytes: usize) -> String {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; grid_size]; grid_size];

    let mut current_bytes = 0;
    let mut additional_bytes = vec![];

    input_string.lines()
        .for_each(|line| {
            let mut split = line.split(',');
            let x = split.next().unwrap().parse::<usize>().unwrap();
            let y = split.next().unwrap().parse::<usize>().unwrap();
            if current_bytes < known_safe_bytes {
                grid[y][x] = '#';
                current_bytes += 1;
            } else {
                additional_bytes.push(IVec2 {x: x as i32, y: y as i32});
            }
        });

    let start_pos = IVec2 { x: 0, y: 0 };
    let end_pos = IVec2 { x: (grid_size - 1) as i32, y: (grid_size - 1) as i32 };

    for additional_byte in additional_bytes {
        grid[additional_byte.y as usize][additional_byte.x as usize] = '#';

        // bfs
        let mut visited: Vec<Vec<bool>> = vec![vec![false; grid_size]; grid_size];
        let mut deque = VecDeque::new();
        deque.push_back((start_pos, 0));
        visited[0][0] = true;

        let mut path_found = false;

        while let Some((current_pos, steps)) = deque.pop_front() {
            if current_pos == end_pos {
                path_found = true;
                break;
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

        if !path_found {
            return format!("{},{}", additional_byte.x, additional_byte.y);
        }
    }

    panic!("No obstacle blocks exit!");
}

#[cfg(test)]
mod tests {
    use crate::day::day18::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input(), 7, 12), 22);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(18), 71, 1024), 234);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input(), 7, 12), "6,1");
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(18), 71, 1024), "58,19");
    }

}
