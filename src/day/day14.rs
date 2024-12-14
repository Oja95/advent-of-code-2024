use crate::day::utils;
use glam::IVec2;
use itertools::Itertools;
use regex::Regex;
use std::cmp::max;

pub fn run() {
    let input_string = utils::read_input(14);
    println!("{}", run_part_one(&input_string, IVec2::new(101, 103)));
    println!("{}", run_part_two(&input_string, IVec2::new(101, 103)));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

fn run_part_one(input_string: &str, bathroom_dimensions: IVec2) -> usize {
    let re: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut input: Vec<Robot> = Vec::new();

    input_string.lines().for_each(|line| {
        let caps = re.captures(line).unwrap();
        let position = IVec2::new(caps[1].parse().unwrap(), caps[2].parse().unwrap());
        let velocity = IVec2::new(caps[3].parse().unwrap(), caps[4].parse().unwrap());
        let robot = Robot { position, velocity };
        input.push(robot);
    });

    for _ in 0..100 {
        input.iter_mut().for_each(|robot| {
            let mut new_pos = robot.position + robot.velocity;
            if new_pos.x < 0 {
                new_pos.x = bathroom_dimensions.x + new_pos.x;
            }
            if new_pos.y < 0 {
                new_pos.y = bathroom_dimensions.y + new_pos.y;
            }

            robot.position = IVec2::new(new_pos.x % (bathroom_dimensions.x), new_pos.y %
                (bathroom_dimensions.y))
        })
    }

    let mid_x = bathroom_dimensions.x / 2;
    let mid_y = bathroom_dimensions.y / 2;

    let mut q_1 = 0;
    let mut q_2 = 0;
    let mut q_3 = 0;
    let mut q_4 = 0;

    input.iter()
        .for_each(|robot| {
            if robot.position.x < mid_x && robot.position.y < mid_y {
                q_1 += 1;
            } else if robot.position.x < mid_x && robot.position.y > mid_y {
                q_2 += 1;
            } else if robot.position.x > mid_x && robot.position.y < mid_y {
                q_3 += 1;
            } else if robot.position.x > mid_x && robot.position.y > mid_y {
                q_4 += 1;
            } else {
            }
        });

    q_1 * q_2 * q_3 * q_4
}

fn run_part_two(input_string: &str, bathroom_dimensions: IVec2) -> i128 {
    let re: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut input: Vec<Robot> = Vec::new();

    input_string.lines().for_each(|line| {
        let caps = re.captures(line).unwrap();
        let position = IVec2::new(caps[1].parse().unwrap(), caps[2].parse().unwrap());
        let velocity = IVec2::new(caps[3].parse().unwrap(), caps[4].parse().unwrap());
        let robot = Robot { position, velocity };
        input.push(robot);
    });

    for iteration in 0.. {
        input.iter_mut().for_each(|robot| {
            let mut new_pos = robot.position + robot.velocity;
            if new_pos.x < 0 {
                new_pos.x = bathroom_dimensions.x + new_pos.x;
            }
            if new_pos.y < 0 {
                new_pos.y = bathroom_dimensions.y + new_pos.y;
            }

            robot.position = IVec2::new(new_pos.x % (bathroom_dimensions.x), new_pos.y %
                (bathroom_dimensions.y))
        });

        // if there is a pattern, assume that it'll have a bunch of elements new to each other.

        let mut possible_candidate = false;

        let mut x_coords = input.iter().map(|robot| robot.position.x).collect_vec();
        x_coords.sort();
        let mut max_chain = 0;
        let mut chain_len = 0;
        for i in 1..x_coords.len() {
            if x_coords[i - 1].abs_diff(x_coords[i]) == 1 {
                chain_len += 1;
            } else {
                max_chain = max(max_chain, chain_len);
            }
        }

        if chain_len < 70 && chain_len > 10 {
            possible_candidate = true;
        }


        if possible_candidate {
            for i in 0..bathroom_dimensions.y {
                for j in 0..bathroom_dimensions.x {
                    let has_robot = input.iter().any(|robot| { robot.position.x == j && robot.position.y == i });
                    if has_robot {
                        print!("\x1b[42m  \x1b[0m");
                    } else {
                        print!("\x1b[41m  \x1b[0m");
                    }
                }
                println!("");
            }
            println!("Iteration: {}", iteration);
        }
    }

    6243
}

#[cfg(test)]
mod tests {
    use crate::day::day14::run_part_one;
    use crate::day::utils;
    use glam::IVec2;

    fn example_input() -> String {
        String::from("\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input(), IVec2::new(11, 7)), 12);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(14), IVec2::new(101, 103)), 222901875);
    }

}
