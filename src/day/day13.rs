use crate::day::utils;
use glam::UVec2;
use regex::Regex;
use std::cmp::min;

pub fn run() {
    let input_string = utils::read_input(13);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}

fn run_part_one(input_string: &str) -> usize {
    let re: Regex = Regex::new(r"(?ms)X[+=](\d+), Y[+=](\d+).*X[+=](\d+), Y[+=](\d+).*X[+=](\d+), Y[+=](\d+)").unwrap();
    let mut input = Vec::new();

    let mut blocks = input_string.split("\n\n");
    while let Some(block) = blocks.next() {
        let caps = re.captures(block).unwrap();
        let a = UVec2 {x: caps[1].parse().unwrap(), y: caps[2].parse().unwrap()};
        let b = UVec2 {x: caps[3].parse().unwrap(), y: caps[4].parse().unwrap()};
        let target = UVec2 {x: caps[5].parse().unwrap(), y: caps[6].parse().unwrap()};
        input.push((a, b, target));
    }

    let mut token_cost_total = 0;
    for case in input.iter() {
        let mut min_token_cost = usize::MAX;

        for i in 0..100 { // press A
            for j in 0..100 { // press B
                let cost = i * 3 + j;

                let reached_pos = (case.0 * i) + (case.1 * j);
                if reached_pos == case.2 {
                    min_token_cost = min(min_token_cost, cost as usize);
                }
            }
        }

        if min_token_cost < usize::MAX {
            token_cost_total += min_token_cost;
        }
    }

    token_cost_total
}

fn run_part_two(input_string: &str) -> i128 {
    let re: Regex = Regex::new(r"(?ms)X[+=](\d+), Y[+=](\d+).*X[+=](\d+), Y[+=](\d+).*X[+=](\d+), Y[+=](\d+)").unwrap();
    let mut input = Vec::new();

    let mut blocks = input_string.split("\n\n");
    while let Some(block) = blocks.next() {
        let caps = re.captures(block).unwrap();
        let a: (i128, i128) =  (caps[1].parse().unwrap(), caps[2].parse().unwrap());
        let b: (i128, i128) = (caps[3].parse().unwrap(), caps[4].parse().unwrap());
        let target = (
            caps[5].parse::<i128>().unwrap() + 10000000000000,
            caps[6].parse::<i128>().unwrap() + 10000000000000
        );
        input.push((a, b, target));
    }

    input.iter()
        .flat_map(|inst| solve(*inst))
        .map(|res| res.0 * 3 + res.1)
        .sum()
}

fn solve(input: ((i128, i128), (i128, i128), (i128, i128))) -> Option<(i128, i128)> {
    let a_x = input.0.0;
    let b_x = input.1.0;
    let target_x = input.2.0;
    let a_y = input.0.1;
    let b_y = input.1.1;
    let target_y = input.2.1;

    // Solving system like
    // a_x * X + b_x * Y = target_x
    // a_y * X + b_y * Y = target_y

    // coefficients
    // a_y * b_x * Y = a_y * target_x
    // a_x * b_y * Y = a_x * target_y

    // a_y * b_x * Y - a_x * b_y * Y = a_y * target_x - a_x * target_y
    // Y = (a_y * target_x - a_x * target_y) / (a_y * b_x - a_x * b_y )
    // X = (target_x - b_x * Y) / a_x

    let y = (a_y * target_x - a_x * target_y) % (a_y * b_x - a_x * b_y );
    let y2 = (a_y * target_x - a_x * target_y) / (a_y * b_x - a_x * b_y );
    let x = (target_x - b_x * y2) % a_x;
    let x2 = (target_x - b_x * y2) / a_x;

    if y != 0 || x != 0 {
        None
    } else {
        Some((x2, y2))
    }
}

#[cfg(test)]
mod tests {
    use crate::day::day13::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 480);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(13)), 36954);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input()), 875318608908);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(13)), 79352015273424);
    }
}
