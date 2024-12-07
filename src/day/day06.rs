use std::collections::{HashSet};
use indexmap::IndexSet;
use itertools::Itertools;
use crate::day::utils;
use crate::day::utils::{access, input_into_matrix, Matrix};

pub fn run() {
    let input_string = utils::read_input(6);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}

fn get_movement_delta(current_direction: char) -> (i8, i8) {
    match current_direction {
        '^' => (0, -1),
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        _ => panic!("Not allowed!")
    }
}

fn turn(current_direction: char) -> char {
    match current_direction {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("Not allowed!")
    }
}

fn run_part_one(input_string: &str) -> usize {
    let mut matrix: Matrix = input_into_matrix(input_string);

    let mut visited_coordinates = HashSet::new();

    let mut current_pos = (0, 0);
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            // Assume starting pos is up
            if matrix[y][x] == '^' {
                current_pos = (x, y);
                visited_coordinates.insert(current_pos);
            }
        }
    }

    loop {
        let delta = get_movement_delta(access(&matrix, current_pos.0, current_pos.1));
        let new_pos = (current_pos.0 as isize + delta.0 as isize, current_pos.1 as isize + delta.1 as isize);

        if new_pos.1 >= matrix[0].len() as isize || new_pos.0 >= matrix.len() as isize || new_pos.0 < 0 || new_pos.1 < 0 {
            break;
        }

        match access(&matrix, new_pos.0 as usize, new_pos.1 as usize) {
            '#' => {
                let new_direction = turn(access(&matrix, current_pos.0, current_pos.1));
                matrix[current_pos.1][current_pos.0] = new_direction;
            },
            '.' => {
                let existing_direction = access(&matrix, current_pos.0, current_pos.1);
                matrix[current_pos.1][current_pos.0] = '.';
                matrix[new_pos.1 as usize][new_pos.0 as usize] = existing_direction;
                visited_coordinates.insert((new_pos.0 as usize, new_pos.1 as usize));
                current_pos = (new_pos.0 as usize, new_pos.1 as usize);
            }
            _ => {
                panic!("Unexpected char");
            }
        }
    }

    visited_coordinates.len()
}

fn run_part_two(input_string: &str) -> usize {
    let mut matrix: Matrix = input_into_matrix(input_string);

    let mut p1_visited_coordinates = IndexSet::new();

    let mut current_pos = (0, 0, '^');
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            // Assume starting pos is up
            if matrix[y][x] == '^' {
                current_pos = (x, y, '^');
                p1_visited_coordinates.insert(current_pos);
            }
        }
    }
    let start_pos = current_pos.clone();

    loop {
        let delta = get_movement_delta(current_pos.2);
        let new_pos = (current_pos.0 as isize + delta.0 as isize, current_pos.1 as isize + delta.1 as isize);

        if new_pos.1 >= matrix[0].len() as isize || new_pos.0 >= matrix.len() as isize || new_pos.0 < 0 || new_pos.1 < 0 {
            break;
        }

        match access(&matrix, new_pos.0 as usize, new_pos.1 as usize) {
            '#' => {
                let new_direction = turn(current_pos.2);
                p1_visited_coordinates.insert((current_pos.0, current_pos.1, new_direction));
                current_pos = (current_pos.0, current_pos.1, new_direction);
            },
            _ => {
                p1_visited_coordinates.insert((new_pos.0 as usize, new_pos.1 as usize, current_pos.2));
                current_pos = (new_pos.0 as usize, new_pos.1 as usize, current_pos.2);
            }
        }
    }

    // this time it also has unique coords + position as well
    // println!("{:?}", p1_visited_coordinates);
    let mut blockades = HashSet::new();

    for i in 1..p1_visited_coordinates.len() {
        let mut visited_so_far = p1_visited_coordinates.clone();
        visited_so_far.split_off(i);

        let current = p1_visited_coordinates.iter().nth(i).unwrap();

        print!("{}\n", i);

        // for each element try putting a blockade at the location and then simulate the iteration
        // to see if it runs into a position already visited.
        let blockade_pos = (current.0, current.1);

        let blockade_in_past_path = visited_so_far.iter()
            .any(|visited| visited.0 == blockade_pos.0 && visited.1 == blockade_pos.1);
        if blockade_in_past_path {
            // we've tried it before, dont attempt again
            continue;
        }

        // Disallow blockade at spawn point
        if blockade_pos.0 == start_pos.0 && blockade_pos.1 == start_pos.1 {
            continue;
        }

        matrix[blockade_pos.1][blockade_pos.0] = '#';

        let mut current_pos = visited_so_far.iter().last().unwrap().clone();
        current_pos = (current_pos.0, current_pos.1, turn(current_pos.2));

        loop {

            // println!("{:?}", visited_so_far);

            let delta = get_movement_delta(current_pos.2);
            let new_pos = (current_pos.0 as isize + delta.0 as isize, current_pos.1 as isize + delta.1 as isize);

            if new_pos.1 >= matrix[0].len() as isize || new_pos.0 >= matrix.len() as isize || new_pos.0 < 0 || new_pos.1 < 0 {
                break;
            }

            match access(&matrix, new_pos.0 as usize, new_pos.1 as usize) {
                '#' => {
                    let new_direction = turn(current_pos.2);
                    if visited_so_far.iter().contains(&(current_pos.0, current_pos.1, new_direction)) {
                        blockades.insert(blockade_pos);
                        break;
                    }

                    visited_so_far.insert((current_pos.0, current_pos.1, new_direction));
                    current_pos = (current_pos.0, current_pos.1, new_direction);
                },
                _ => {
                    if visited_so_far.iter().contains(&(new_pos.0 as usize, new_pos.1 as usize, current_pos.2)) {
                        blockades.insert(blockade_pos);
                        break;
                    }

                    visited_so_far.insert((new_pos.0 as usize, new_pos.1 as usize, current_pos.2));
                    current_pos = (new_pos.0 as usize, new_pos.1 as usize, current_pos.2);
                }
            }


        }

        matrix[blockade_pos.1][blockade_pos.0] = '.';
    }

    println!("{:?}", blockades);

    blockades.len()
}

#[cfg(test)]
mod tests {
    use crate::day::day06::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 41);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(6)), 5331);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input()), 6);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(6)), 1812);
    }
}
