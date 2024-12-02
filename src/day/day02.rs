use crate::day::utils;

pub fn run() {
    let input_string = utils::read_input(2);
    let part_one_result = run_part_one(&input_string);
    println!("{}", part_one_result);
    let part_two_result = run_part_two(&input_string);
    println!("{}", part_two_result);
}

fn run_part_one(input_string: &str) -> u64 {
    parse_input(input_string)
        .iter()
        .filter(|item| is_safe(item))
        .count() as u64
}

fn run_part_two(input_string: &str) -> u64 {
    parse_input(input_string)
        .iter()
        .filter(|report| {
            let mut safe = is_safe(report);

            if !safe {
                for index in 0..report.len() {
                    let dampened_report: Vec<_> = report[..index]
                        .iter()
                        .chain(&report[index + 1..])
                        .copied()
                        .collect();
                    safe = is_safe(&dampened_report);
                    if safe {
                        return safe;
                    }
                }
            }

            safe
        })
        .count() as u64
}

fn is_safe(report: &Vec<usize>) -> bool {
    let direction = if report.get(0).unwrap() > report.get(1).unwrap() {
        Direction::Descending
    } else {
        Direction::Ascending
    };

    for index in 1..report.len() {
        let previous_elem = report[index - 1];
        let current_elem = report[index];
        if direction == Direction::Ascending {
            if previous_elem > current_elem {
                return false;
            }
        } else {
            if previous_elem < current_elem {
                return false;
            }
        }

        let diff = previous_elem.abs_diff(current_elem);
        if diff > 3 || diff < 1 {
            return false;
        }
    }

    true
}

#[derive(PartialEq)]
enum Direction {
    Descending,
    Ascending,
}



fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|line| line.split_whitespace().map(|item| item.parse().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day::day02::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 2);
    }

    #[test]
    fn test_day2_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(2)), 606);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input()), 4);
    }

    #[test]
    fn test_day2_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(2)), 644);
    }
}
