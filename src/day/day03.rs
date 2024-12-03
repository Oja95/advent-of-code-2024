use regex::Regex;
use crate::day::utils;

pub fn run() {
    let input_string = utils::read_input(3);
    let part_one_result = run_part_one(&input_string);
    println!("{}", part_one_result);
    let part_two_result = run_part_two(&input_string);
    println!("{}", part_two_result);
}

fn run_part_one(input_string: &str) -> u64 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    regex.captures_iter(input_string)
        .map(|capture| capture.extract())
        .map(|(_, [first, second])| {
            first.parse::<u64>().unwrap() * second.parse::<u64>().unwrap()
        }).sum()
}

fn run_part_two(input_string: &str) -> u64 {
    let regex = Regex::new(r"(?s)don't\(\).*?do\(\)").unwrap();
    run_part_one(regex.replace_all(input_string, "").as_ref())
}

#[cfg(test)]
mod tests {
    use crate::day::day03::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
    }

    fn example_input_part_two() -> String {
        String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 161);
    }

    #[test]
    fn test_day2_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(3)), 153469856);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input_part_two()), 48);
    }

    #[test]
    fn test_day2_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(3)), 77055967);
    }
}
