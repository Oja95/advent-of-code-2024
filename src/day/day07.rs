use std::collections::HashMap;
use itertools::Itertools;
use crate::day::utils;

pub fn run() {
    let input_string = utils::read_input(7);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}

#[derive(Debug)]
struct Calibration {
    result: u64,
    operands: Vec<u64>
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn apply(&self, left: u64, right: u64) -> u64 {
        match self {
            Operator::Add => {left + right}
            Operator::Multiply => {left * right}
            Operator::Concatenate => {
                let digit_count = match right {
                    0 => 1,
                    _ => (right as f64).log10().floor() as u32 + 1
                };

                left * 10_u64.pow(digit_count) + right
            }
        }
    }
}

fn produce_combinations(size: usize, with_concat: bool) -> Vec<Vec<Operator>> {
    _produce_combinations(size, with_concat, vec![])
}

fn _produce_combinations(size: usize, with_concat: bool, accumulator: Vec<Operator>) -> Vec<Vec<Operator>> {
    if size == 0 {
        return vec![accumulator];
    }

    let mut results = Vec::new();

    let mut vec1 = accumulator.clone();
    vec1.push(Operator::Multiply);
    results.append(&mut _produce_combinations(size - 1, with_concat, vec1));

    let mut vec2 = accumulator.clone();
    vec2.push(Operator::Add);
    results.append(&mut _produce_combinations(size - 1, with_concat, vec2));

    if with_concat {
        let mut vec3 = accumulator.clone();
        vec3.push(Operator::Concatenate);
        results.append(&mut _produce_combinations(size - 1, with_concat, vec3));
    }

    results
}

fn run_calibration_check(calibration_inputs: Vec<Calibration>, with_concat: bool) -> u64 {
    let mut sum = 0;
    let mut operator_combinations_map = HashMap::new();

    for calibration_input in calibration_inputs {
        let operators_count = calibration_input.operands.len() - 1;
        let operator_combinations = operator_combinations_map
            .entry(operators_count)
            .or_insert_with_key(|count| produce_combinations(*count, with_concat));

        for operator_combination in operator_combinations {
            let mut combination_iterator = operator_combination.iter();
            let combination_total = calibration_input.operands.iter()
                .skip(1)
                .fold(calibration_input.operands[0], |acc, val| {
                    combination_iterator.next().unwrap().apply(acc, *val)
                });

            if combination_total == calibration_input.result {
                sum += combination_total;
                break;
            }
        }
    }
    sum
}

fn parse_input(input_string: &str) -> Vec<Calibration> {
    input_string.lines()
        .map(|line| {
            let mut split = line.split(":");
            let result = split.next().unwrap().parse().unwrap();
            let operands = split.next().unwrap()
                .split_whitespace()
                .map(|operand| operand.parse().unwrap())
                .collect();
            Calibration { result, operands }
        }).collect_vec()
}

fn run_part_one(input_string: &str) -> usize {
    let calibration_inputs = parse_input(input_string);
    let sum = run_calibration_check(calibration_inputs, false);
    sum.try_into().unwrap()
}

fn run_part_two(input_string: &str) -> usize {
    let calibration_inputs = parse_input(input_string);
    let sum = run_calibration_check(calibration_inputs, true);
    sum.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day::day07::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 3749);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(7)), 2314935962622);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input()), 11387);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(7)), 401477450831495);
    }
}
