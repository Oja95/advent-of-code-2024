use std::collections::HashMap;
use itertools::Itertools;
use crate::day::utils;

pub fn run() {
    let input_string = utils::read_input(11);
    // println!("{}", run_part_one(&input_string, 6));
    // println!("{}", run_part_one(&input_string, 25));
    println!("{}", run_part_two(&input_string, 75));
}


fn run_part_one(input_string: &str, blinks: usize) -> usize {
    // run part 1 with iteration to get the hang of the logic, then use maths :TM: or DP to solve
    // 2nd as it likely wants to get input after 1 billion iterations or something

    let mut stones = input_string.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    for _ in 0..blinks {
        println!("{:?} {}", stones, stones.len());

        let mut j = 0;
        while j < stones.len() {
            let mut stone = stones[j];
            if stone == 0 {
                stones[j] = 1;
                j += 1;
                continue;
            }

            let digits_count = num_digits(stone);
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

    println!("{:?}", stones);
    stones.len()
}

fn run_part_two(input_string: &str, blinks: u128) -> u128 {
    let mut stones = input_string.split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect_vec();

    // Use a hashmap for DP lookup to map (elem, blinks) -> how many stones it will produce
    // mapping

    let mut stones_sum = 0;

    let mut dp_map = HashMap::new();

    for stone in stones {
        stones_sum += dp(stone, blinks, &mut dp_map);
    }

    println!("{:?}", dp_map);

    stones_sum
}

// (elem, blinks) -> (stones count)?
// calculate stones count for a elem after given amount of iterations
fn dp(elem: u128, iterations: u128, dp_map: &mut HashMap<(u128, u128), u128>) -> u128 {
    if iterations == 0 {
        return 1;
    }

    if let Some(result) = dp_map.get(&(elem, iterations)) {
        return *result;
    }

    let result: u128;
    if elem == 0 {
        result = dp(1, iterations - 1, dp_map);
    } else {
        let digits_count = num_digits_u128(elem);
        if digits_count % 2 == 0 {
            let split = 10_u128.pow((digits_count / 2) as u32);
            let first = elem / split;
            let second = elem % split;
            result = dp(first, iterations - 1, dp_map) + dp(second, iterations - 1, dp_map);
        } else {
            result = dp(elem * 2024, iterations - 1, dp_map);
        }
    }

    dp_map.insert((elem, iterations), result);

    // println!("{:?}", dp_map);

    result
}

fn num_digits(num: usize) -> usize {
    (num as f64).log10().floor() as usize + 1
}

fn num_digits_u128(num: u128) -> usize {
    (num as f64).log10().floor() as usize + 1
}

#[cfg(test)]
mod tests {
    use crate::day::day11::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("125 17")
    }


    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input(), 6), 22);
        assert_eq!(run_part_one(&example_input(), 25), 55312);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(11), 25), 550);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input(), 6), 22);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(11), 75), 1255);
    }
}

/*
0
1
2024
20 24
2 0 2 4
4048, 1, 4048, 8096
40, 48, 2024, 40, 48, 80, 96
4, 0, 4, 8, 20, 24, 4, 0, 4, 8, 8, 0, 9, 6
8096, 1, 8096, 16192, 2, 0, 2, 4, 8096, 1, 8096, 16192, 16192, 1, 18216, 12144
80, 96, 2024, 80, 96, 32772608, 4048, 1, 4048, 8096, 80, 96, 2024, 80, 96, 32772608, 32772608, 2024, 36869184, 24579456
8, 0, 9, 6, 20, 24, 8, 0, 9, 6, 3277, 2608, 40, 48, 2024, 40, 48, 80, 96, 8, 0, 9, 6, 20, 24, 8, 0, 9, 6, 3277, 2608, 3277, 2608, 20, 24, 3686, 9184, 2457, 9456
*/

