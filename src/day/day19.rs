use crate::day::utils;
use glam::IVec2;
use std::collections::{HashMap, VecDeque};

pub fn run() {
    let input_string = utils::read_input(19);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}

fn run_part_one(input_string: &str) -> usize {
    let mut lines = input_string.lines();
    let segments = lines.next().unwrap().split(", ").collect::<Vec<&str>>();
    println!("{:?}", segments);
    lines.next();

    let mut towels = vec![];
    while let Some(line) = lines.next() {
        towels.push(line);
    }

    let mut dp = HashMap::new();
    let mut possible = 0;
    for towel in &towels {
        let can_combine = check_towel(*towel, &segments, &mut dp);
        if can_combine {
            possible += 1;
        }
    }

    possible
}

fn check_towel<'a>(towel_pattern: &'a str, segments: &Vec<&str>, dp: &mut HashMap<&'a str, bool>) -> bool {
    // println!("{:?}", dp);
    if let Some(value) = dp.get(towel_pattern) {
        return *value;
    }

    if towel_pattern == "" {
        return true;
    }

    for segment in segments {
        if towel_pattern.starts_with(segment) {
            let new = towel_pattern.strip_prefix(segment).unwrap();
            // println!("towel pat {}", towel_pattern);
            // println!("segment {:?}", segments);
            // println!("new {}", new);
            let can_build = check_towel(new, segments, dp);
            if can_build {
                dp.insert(towel_pattern, true);
                return true;
            }
        }
    }

    dp.insert(towel_pattern, false);
    false

}

fn run_part_two(input_string: &str) -> usize {
    let mut lines = input_string.lines();
    let segments = lines.next().unwrap().split(", ").collect::<Vec<&str>>();
    println!("{:?}", segments);
    lines.next();

    let mut towels = vec![];
    while let Some(line) = lines.next() {
        towels.push(line);
    }

    let mut dp = HashMap::new();
    let mut possible_combinations = 0;
    for towel in &towels {
        println!("{:?}", towel);
        possible_combinations += check_towel_2(*towel, &segments, &mut dp);
    }

    possible_combinations
}

fn check_towel_2<'a>(towel_pattern: &'a str, segments: &Vec<&str>, dp: &mut HashMap<&'a str, usize>) -> usize {
    // println!("{:?}", dp);
    if let Some(value) = dp.get(towel_pattern) {
        return *value;
    }

    if towel_pattern == "" {
        return 1;
    }

    let mut patterns_count = 0;

    for segment in segments {
        if towel_pattern.starts_with(segment) {
            let new = towel_pattern.strip_prefix(segment).unwrap();
            let new_count = check_towel_2(new, segments, dp);
            dp.insert(new, new_count);
            patterns_count += new_count;
        }
    }

    dp.insert(towel_pattern, patterns_count);
    patterns_count

    // dp.insert(towel_pattern, patterns_count);
    // patterns_count
}

#[cfg(test)]
mod tests {
    use crate::day::day19::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 6);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(19)), 287);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input()), 16);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(19)), 1495455);
    }

}
