use crate::day::utils;
use itertools::Itertools;
use std::collections::HashMap;

pub fn run() {
    let input_string = utils::read_input(22);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}

fn run_part_one(input_string: &str) -> usize {
    input_string.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .map(|line| {
            let mut res = line;
            for _ in 0..2000 {
                res = calculate_next_secret(res);
            }
            res
        }).sum()
}

fn calculate_next_secret(mut secret: usize) -> usize {
    secret = ((secret << 6) ^ secret) & 0xFFFFFF;
    secret = ((secret >> 5) ^ secret) & 0xFFFFFF;
    (secret << 11 ^ secret) & 0xFFFFFF
}

fn run_part_two(input_string: &str) -> usize {
    let mut cumulative_pattern_bananas_map = HashMap::new();

    input_string.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .for_each(|line| {
            let mut pattern_bananas_map = HashMap::new();
            let mut last_digits = vec![];
            let mut secret = line;
            last_digits.push((line % 10) as isize);
            for _ in 0..2000 {
                secret = calculate_next_secret(secret);
                last_digits.push((secret % 10) as isize);
            }

            last_digits.windows(5)
                .for_each(|pattern_sequence_window| {
                    let pattern: (isize, isize, isize, isize) = pattern_sequence_window.windows(2)
                        .map(|sub_window| sub_window[1] - sub_window[0])
                        .collect_tuple().unwrap();

                    let price = pattern_sequence_window[4];
                    pattern_bananas_map.entry(pattern).or_insert(price);
                });

            for (pattern, bananas) in pattern_bananas_map {
                cumulative_pattern_bananas_map.entry(pattern).or_insert(vec![]).push(bananas);
            }
        });

    cumulative_pattern_bananas_map.values()
        .map(|bananas| bananas.iter().sum::<isize>())
        .max().unwrap() as usize
}

#[cfg(test)]
mod tests {
    use crate::day::day22::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
1
10
100
2024
")
    }

    fn example_input_2() -> String {
        String::from("\
1
2
3
2024
")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 37327623);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(22)), 20068964552);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input_2()), 23);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(22)), 2246);
    }
}
