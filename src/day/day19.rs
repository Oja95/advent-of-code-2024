use crate::day::utils;
use std::collections::HashMap;

pub fn run() {
    let input_string = utils::read_input(19);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}

fn get_input(input_string: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input_string.lines();
    let segments = lines.next().unwrap().split(", ").collect::<Vec<&str>>();
    lines.next();

    let mut towels = vec![];
    while let Some(line) = lines.next() {
        towels.push(line);
    }
    (segments, towels)
}

fn run_part_one(input_string: &str) -> usize {
    let (segments, towels) = get_input(input_string);

    towels.iter()
        .map(|towel| towel_combinations(*towel, &segments, &mut HashMap::new()))
        .filter(|combinations| *combinations > 0)
        .count()
}

fn run_part_two(input_string: &str) -> usize {
    let (segments, towels) = get_input(input_string);

    towels.iter()
        .map(|towel| towel_combinations(*towel, &segments, &mut HashMap::new()))
        .sum()
}

fn towel_combinations<'a>(towel_pattern: &'a str, segments: &Vec<&str>, dp: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(value) = dp.get(towel_pattern) {
        return *value;
    }

    if towel_pattern == "" {
        return 1;
    }

    let mut patterns_count = 0;
    for segment in segments {
        if towel_pattern.starts_with(segment) {
            let sub_pattern = towel_pattern.strip_prefix(segment).unwrap();
            let sub_pattern_combinations_count = towel_combinations(sub_pattern, segments, dp);
            dp.insert(sub_pattern, sub_pattern_combinations_count);
            patterns_count += sub_pattern_combinations_count;
        }
    }

    dp.insert(towel_pattern, patterns_count);
    patterns_count
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
        assert_eq!(run_part_two(&utils::read_input(19)), 571894474468161);
    }

}
