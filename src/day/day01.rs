use crate::day::utils;
use core::fmt;

pub fn run() {
    let input_string = utils::read_input(1);
    let i = run_part_one(input_string);
    println!("{}", i);
}

fn run_part_one(input_string: String) -> u64 {
    let input_pairs: Vec<InputPair> = input_string.lines()
        .map(|line| parse_line(line).unwrap())
        .collect();

    let mut first_reserachers_list: Vec<u64> = input_pairs.iter()
        .map(|input_pair| input_pair.first)
        .collect::<Vec<_>>();
    first_reserachers_list.sort();

    let mut second_reserachers_list: Vec<u64> = input_pairs.into_iter()
        .map(|input_pair| input_pair.second)
        .collect::<Vec<_>>();
    second_reserachers_list.sort();

    if first_reserachers_list.len() != second_reserachers_list.len() {
        panic!("Expected the two list to be equal in size!");
    }

    let mut id_diffs = Vec::new();
    for i in 0..first_reserachers_list.len() {
        id_diffs.push(first_reserachers_list[i].abs_diff(second_reserachers_list[i]));
    }

    id_diffs.iter().sum()
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
    use crate::day::day01::run_part_one;
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
        assert_eq!(run_part_one(example_input()), 11);
    }

    #[test]
    fn test_day1_input_part_one() {
        assert_eq!(run_part_one(utils::read_input(1)), 1938424);
    }
}
