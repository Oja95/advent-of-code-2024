use crate::day::utils;
use glam::IVec2;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

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

//379a
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
    let direction_map: HashMap<IVec2, char> = [
        (IVec2::new(1, 0), '>'),
        (IVec2::new(0, 1), 'v'),
        (IVec2::new(-1, 0), '<'),
        (IVec2::new(0, -1), '^'),
    ].iter().cloned().collect();

    let input_lines = input_string.lines().collect_vec();
    let mut result = 0;


    let numpad_grid: &[&[char]] = &NUMPAD.iter().map(|row| row.as_ref()).collect::<Vec<_>>();
    let dirpad_grid: &[&[char]] = &DIRPAD.iter().map(|row| row.as_ref()).collect::<Vec<_>>();
    input_lines.iter().for_each(|line| {
        let mut human_inserts = vec![];

        let mut start_pos = NUMPAD_START;
        let mut start_2 = DIRPAD_START;
        let mut start_3 = DIRPAD_START;

        for elem in line.chars() {
            // last also eneds to press a
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
            // let shortest_path = shortest_paths.first().unwrap();

            let mut shortest_sub_list = vec!['\0'; 100];

            for shortest_path in shortest_paths {
                let mut directions = Vec::new();
                for i in 1..shortest_path.len() {
                    let delta = shortest_path[i] - shortest_path[i - 1];
                    let direction = direction_map.get(&delta).unwrap();
                    directions.push(*direction);
                }
                directions.push('A');

                println!("l1 {:?}", directions);

                let mut im_losing_it = Vec::new();
                for shortest_path_direction in directions {
                    // find '<' or whatever shortest paths in the NUMPAD
                    let mut numpad_target = Default::default();
                    DIRPAD.iter().enumerate()
                        .for_each(|(y, row)|
                            row.iter().enumerate()
                                .for_each(|(x, char)| {
                                    if *char == shortest_path_direction {
                                        numpad_target = IVec2::new(x as i32, y as i32);
                                    }
                                }));
                    // let mut new_paths = Vec::new();

                    let mut vec = find_all_shortest_paths(start_2, numpad_target, &dirpad_grid);
                    // doesnt matter here, the same amount of steps for any path
                    let x1 = vec.first().unwrap();
                    let mut vec1 = x1.clone();
                    start_2 = numpad_target;
                    im_losing_it.push(vec1);
                }

                let vec2 = im_losing_it.iter().flatten().collect_vec();
                let mut second_robot_directions = Vec::new();
                for i in 1..vec2.len() {
                    let new_delta = vec2[i] - vec2[i - 1];
                    let mut new_direction;
                    if new_delta == IVec2::new(0, 0) {
                        new_direction = 'A';
                    } else {
                        new_direction = *direction_map.get(&new_delta).unwrap();
                    }
                    second_robot_directions.push(new_direction);
                }
                second_robot_directions.push('A');
                println!("l2 {:?}", second_robot_directions);


                let mut third_robot_directions = Vec::new();

                for second_robot_directions in second_robot_directions {
                    // find '<' or whatever shortest paths in the NUMPAD
                    let mut numpad_target = Default::default();
                    DIRPAD.iter().enumerate()
                        .for_each(|(y, row)|
                            row.iter().enumerate()
                                .for_each(|(x, char)| {
                                    if *char == second_robot_directions {
                                        numpad_target = IVec2::new(x as i32, y as i32);
                                    }
                                }));
                    // let mut new_paths = Vec::new();

                    let mut vec = find_all_shortest_paths(start_3, numpad_target, &dirpad_grid);
                    // doesnt matter here, the same amount of steps for any path
                    let x1 = vec.first().unwrap();
                    let mut vec1 = x1.clone();
                    start_3 = numpad_target;
                    third_robot_directions.push(vec1);
                }

                let vec3 = third_robot_directions.iter().flatten().collect_vec();
                let mut third_robot_direction_fr = Vec::new();
                for i in 1..vec3.len() {
                    let new_delta = vec3[i] - vec3[i - 1];
                    let mut new_direction;
                    if new_delta == IVec2::new(0, 0) {
                        new_direction = 'A';
                    } else {
                        new_direction = *direction_map.get(&new_delta).unwrap();
                    }
                    third_robot_direction_fr.push(new_direction);
                }
                third_robot_direction_fr.push('A');

                println!("l3 {:?}", third_robot_direction_fr);

                if third_robot_direction_fr.len() < shortest_sub_list.len() {
                    shortest_sub_list = third_robot_direction_fr;
                }
            }

            human_inserts.append(&mut shortest_sub_list);


            start_pos = target;
        }

        println!("{:?}", human_inserts);
        println!("{:?}", human_inserts.len());
        println!("{}", human_inserts.iter().join(""));

        let x2 = line.trim_end_matches('A');
        println!("{}", x2.parse::<usize>().unwrap());
        result += human_inserts.len() * x2.parse::<usize>().unwrap();
    });
    // expected: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
    // current : v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA^<A>Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A
// ex: <vA<AA>>^AAvA<^A>AAvA^A | <vA>^AA<A>A | <v<A>A>^AAAvA<^A>A
// cu: v<<A>>^AAv<A<A>>^AAvAA^<A>A | v<A>^AA<A>A | v<A<A>>^AAAvA^<A>A
    result

    // starts A
    // < v A < A A > > ^ A A v A < ^ A > A A v A ^ A
    // v < < A A > ^ A A > A
    // < < ^ ^ A

    // l3 v < < A > > ^ A A v < A < A > > ^ A A v A A ^ < A > A
    // l2 ['<', 'A', 'A', 'v', '<', 'A', 'A', '>', '>', '^', 'A']
    // l1 ['^', '^', '<', '<', 'A']
}

fn run_part_two(input_string: &str) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use crate::day::day21::{run_part_one, run_part_two};
    use crate::day::utils;

//     fn example_input() -> String {
//         String::from("\
// 029A
// ")
//     }

    fn example_input() -> String {
        String::from("\
029A
980A
179A
456A
379A
")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 126384);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(21)), 1406);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input()), 285);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(21)), 1006101);
    }
}
