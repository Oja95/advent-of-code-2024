use crate::day::utils;
use glam::IVec2;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::env::var;

pub fn run() {
    let input_string = utils::read_input(21);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}

static MOVEMENT_DELTAS: [IVec2; 4] = [
    IVec2::new(0, 1),
    IVec2::new(1, 0),
    IVec2::new(0, -1),
    IVec2::new(-1, 0),
];

static NUMPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['\0', '0', 'A']
];

static DIRPAD: [[char; 3]; 2] = [
    ['\0', '^', 'A'],
    ['<', 'v', '>']
];

static NUMPAD_START: IVec2 = IVec2 { x: 2, y: 3 };
static DIRPAD_START: IVec2 = IVec2 { x: 2, y: 0 };

fn find_all_shortest_paths(start_pos: IVec2, end_pos: IVec2, grid: &[&[char]]) -> Vec<Vec<IVec2>> {
    let mut visited: HashMap<IVec2, usize> = HashMap::new();
    let mut result = vec![];

    let mut deque = VecDeque::new();
    deque.push_back((start_pos, vec![start_pos]));
    visited.insert(start_pos, 0);

    while let Some((current_pos, path)) = deque.pop_front() {
        if current_pos == end_pos {
            result.push(path.clone());
        }

        for delta in MOVEMENT_DELTAS {
            let new_pos = current_pos + delta;
            if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= grid[0].len() as i32 || new_pos.y >= grid.len() as i32 {
                continue;
            }

            if grid[new_pos.y as usize][new_pos.x as usize] == '\0' {
                continue;
            }

            let new_dist = path.len();
            if !visited.contains_key(&new_pos) || new_dist <= visited[&new_pos] {
                visited.insert(new_pos, new_dist);
                let mut vec1 = path.clone();
                vec1.push(new_pos);
                deque.push_back((new_pos, vec1));
            }
        }
    }

    result
}

fn run_part_one(input_string: &str) -> usize {
    run_simulation(input_string, 2)
}

fn run_simulation(input_string: &str, robot_layers: usize) -> usize {
    let direction_map: HashMap<IVec2, char> = [
        (IVec2::new(1, 0), '>'),
        (IVec2::new(0, 1), 'v'),
        (IVec2::new(-1, 0), '<'),
        (IVec2::new(0, -1), '^'),
    ].iter().cloned().collect();

    // Map (bot layer, from, to) -> no_of_instructions
    let mut dp: HashMap<(usize, char, char), usize> = HashMap::new();

    let input_lines = input_string.lines().collect_vec();
    let mut result = 0;

    let numpad_grid: &[&[char]] = &NUMPAD.iter().map(|row| row.as_ref()).collect::<Vec<_>>();
    let dirpad_grid: &[&[char]] = &DIRPAD.iter().map(|row| row.as_ref()).collect::<Vec<_>>();

    input_lines.iter().for_each(|line| {
        let mut human_inserts_count = 0;
        let mut start_pos = NUMPAD_START;

        for elem in line.chars() {
            let mut target = Default::default();
            NUMPAD.iter().enumerate()
                .for_each(|(y, row)|
                    row.iter().enumerate()
                        .for_each(|(x, char)| {
                            if *char == elem {
                                target = IVec2::new(x as i32, y as i32);
                            }
                        }));

            let shortest_paths = find_all_shortest_paths(start_pos, target, numpad_grid);
            let mut shortest_sub_list_len = usize::MAX;

            for shortest_path in shortest_paths {
                let mut directions = Vec::new();
                for i in 1..shortest_path.len() {
                    let delta = shortest_path[i] - shortest_path[i - 1];
                    let direction = direction_map.get(&delta).unwrap();
                    directions.push(*direction);
                }
                directions.push('A');

                let result_len = solve(&direction_map, &dirpad_grid, &mut directions, robot_layers, &mut dp);
                if result_len < shortest_sub_list_len {
                    shortest_sub_list_len = result_len;
                }
            }

            human_inserts_count += shortest_sub_list_len;
            start_pos = target;
        }

        let x2 = line.trim_end_matches('A');
        println!("{}", x2.parse::<usize>().unwrap());
        println!("{}", human_inserts_count);
        result += human_inserts_count * x2.parse::<usize>().unwrap();
    });

    result
}

fn solve(direction_map: &HashMap<IVec2, char>, dirpad_grid: &&[&[char]], mut directions:
&mut Vec<char>, robot_layers: usize, dp: &mut HashMap<(usize, char, char), usize>) -> usize {
    if robot_layers == 0 {
        return directions.len();
    }

    let mut result = 0;

    let mut dirpad_pos = DIRPAD_START;
    for shortest_path_direction in directions {
        let mut numpad_target = Default::default();
        DIRPAD.iter().enumerate()
            .for_each(|(y, row)|
                row.iter().enumerate()
                    .for_each(|(x, char)| {
                        if *char == *shortest_path_direction {
                            numpad_target = IVec2::new(x as i32, y as i32);
                        }
                    }));

        if let Some(value) = dp.get(&(robot_layers, dirpad_grid[dirpad_pos.y as usize][dirpad_pos.x as usize], *shortest_path_direction)) {
            result += value;

            dirpad_pos = numpad_target;
            continue;
        }

        let direction_pad_shortest_paths = find_all_shortest_paths(dirpad_pos, numpad_target, &dirpad_grid);
        // let x1 = direction_pad_shortest_paths.iter()
        //     .min_by(|x, y|
        //         coords_to_path(&direction_map, x).len().cmp(&coords_to_path(&direction_map, y).len())).unwrap();

        let x1 = direction_pad_shortest_paths.iter()
            .map(|x| coords_to_path(&direction_map, x))
            .map(|mut x| solve(direction_map, dirpad_grid, &mut x, robot_layers - 1, dp))
            .min().unwrap();

        // let mut new_path = coords_to_path(&direction_map, x1);

        // let subcalc = solve(direction_map, dirpad_grid, &mut new_path, robot_layers - 1, dp);
        dp.insert((robot_layers, dirpad_grid[dirpad_pos.y as usize][dirpad_pos.x as usize],
                   *shortest_path_direction), x1);
        result += x1;

        dirpad_pos = numpad_target;
    }

    result
}

fn coords_to_path(direction_map: &HashMap<IVec2, char>, vec2: &Vec<IVec2>) -> Vec<char> {
    if vec2.len() == 1 {
        return vec!['A'];
    }
    let mut second_robot_directions = Vec::new();
    for i in 1..vec2.len() {
        let new_delta = vec2[i] - vec2[i - 1];
        let new_direction;
        if new_delta == IVec2::new(0, 0) {
            new_direction = 'A';
        } else {
            new_direction = *direction_map.get(&new_delta).unwrap();
        }
        second_robot_directions.push(new_direction);
    }
    second_robot_directions.push('A');
    second_robot_directions
}

fn run_part_two(input_string: &str) -> usize {
    run_simulation(input_string, 25)
}

#[cfg(test)]
mod tests {
    use crate::day::day21::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
029A
980A
179A
456A
379A")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 126384);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(21)), 162740);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input()), 154115708116294);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(21)), 0);
    }
}
