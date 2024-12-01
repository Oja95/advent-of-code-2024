use crate::day::utils;
use core::fmt;
use std::collections::HashMap;
use itertools::Itertools;

pub fn run() {
    let input_string = utils::read_input(1);
    let part_one_result = run_part_one(&input_string);
    println!("{}", part_one_result);
    let part_two_result = run_part_two(&input_string);
    println!("{}", part_two_result);
}

fn run_part_one(input_string: &str) -> u64 {
    let input_pairs = input_to_pairs(&input_string);

    input_pairs.iter().map(|input_pair| input_pair.first).sorted()
        .zip(input_pairs.iter().map(|input_pair| input_pair.second).sorted())
        .map(|(first, second)| first.abs_diff(second))
        .sum()
}

fn run_part_two(input_string: &str) -> u64 {
    let input_pairs = input_to_pairs(&input_string);

    let second_researchers_locations_list: Vec<u64> = input_pairs.iter()
        .map(|input_pair| input_pair.second)
        .collect();

    let mut second_researcher_location_counts = HashMap::new();
    for location_id in second_researchers_locations_list {
        let x = second_researcher_location_counts.get(&location_id).unwrap_or_else(|| &0);
        second_researcher_location_counts.insert(location_id, x + 1);
    }

    let mut similarity_score = 0;

    input_pairs.iter()
        .map(|input_pair| input_pair.first)
        .for_each(|first| {
            let first_occurrences_in_second = second_researcher_location_counts.get(&first).unwrap_or_else(|| &0);
            similarity_score += first * first_occurrences_in_second;
        });

    similarity_score
}

fn input_to_pairs(input_string: &str) -> Vec<InputPair> {
    input_string.lines()
        .map(|line| parse_line(line).unwrap())
        .collect()
}

fn parse_line(input: &str) -> Option<InputPair> {
    let mut split_whitespace = input.split_whitespace();
    let first = split_whitespace.next()?.parse().ok()?;
    let second = split_whitespace.next()?.parse().ok()?;
    Some(InputPair { first, second })
}

#[derive(Debug)]
struct InputPair {
    first: u64,
    second: u64,
}

impl fmt::Display for InputPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pair {{ first: {}, second: {} }}", self.first, self.second)
    }
}

#[cfg(test)]
mod tests {
    use crate::day::day01::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
3   4
4   3
2   5
1   3
3   9
3   3")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 11);
    }

    #[test]
    fn test_day1_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(1)), 1938424);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input()), 31);
    }

    #[test]
    fn test_day1_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(1)), 22014209);
    }
}
