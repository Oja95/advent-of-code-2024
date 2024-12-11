use std::collections::HashMap;
use itertools::Itertools;
use crate::day::utils;

pub fn run() {
    let input_string = utils::read_input(11);
    println!("{}", run_part_one(&input_string, 25));
    println!("{}", run_part_two(&input_string, 75));
}

fn run_part_one(input_string: &str, blinks: usize) -> usize {
    // run part 1 with iteration to get the hang of the logic, then use maths :TM: or DP to solve
    // 2nd as it likely wants to get input after 1 billion iterations or something

    let mut stones = input_string.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    for _ in 0..blinks {
        let mut j = 0;
        while j < stones.len() {
            let stone = stones[j];
            if stone == 0 {
                stones[j] = 1;
                j += 1;
                continue;
            }

            let digits_count = (stone as f64).log10().floor() as usize + 1;
            if digits_count % 2 == 0 {
                let split = 10_usize.pow((digits_count / 2) as u32);
                stones.insert(j + 1, stone % split); // linear time
                stones[j] = stone / split;
                j += 2;
            } else {
                stones[j] = stone * 2024;
                j += 1;
            }

        }
    }

    stones.len()
}

fn run_part_two(input_string: &str, blinks: usize) -> usize {
    // Memoization map storing (elem, blinks) -> (resulting stones count)
    let mut dp_map = HashMap::new();

    input_string.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .map(|stone| find_stone_count(stone, blinks, &mut dp_map))
        .sum()
}

fn find_stone_count(elem: usize, iterations: usize, dp_map: &mut HashMap<(usize, usize), usize>) -> usize {
    if iterations == 0 {
        return 1;
    }

    if let Some(result) = dp_map.get(&(elem, iterations)) {
        return *result;
    }

    let result;
    if elem == 0 {
        result = find_stone_count(1, iterations - 1, dp_map);
    } else {
        let digits_count = (elem as f64).log10().floor() as usize + 1;
        if digits_count % 2 == 0 {
            let split = 10_usize.pow((digits_count / 2) as u32);
            let first = elem / split;
            let second = elem % split;
            result = find_stone_count(first, iterations - 1, dp_map)
                + find_stone_count(second, iterations - 1, dp_map);
        } else {
            result = find_stone_count(elem * 2024, iterations - 1, dp_map);
        }
    }

    dp_map.insert((elem, iterations), result);
    result
}

#[cfg(test)]
mod tests {
    use crate::day::day11::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("125 17")
    }

    #[test]
    fn test_exercise_example_input() {
        assert_eq!(run_part_one(&example_input(), 6), 22);
        assert_eq!(run_part_one(&example_input(), 25), 55312);
        assert_eq!(run_part_two(&example_input(), 6), 22);
        assert_eq!(run_part_two(&example_input(), 25), 55312);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(11), 25), 204022);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(11), 75), 241651071960597);
    }
}
