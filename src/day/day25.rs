use crate::day::utils;
use itertools::Itertools;

pub fn run() {
    let input_string = utils::read_input(25);
    println!("{}", run_part_one(&input_string)); }

fn run_part_one(input_string: &str) -> usize {
    let mut keys: Vec<Vec<usize>> = vec![];
    let mut locks: Vec<Vec<usize>> = vec![];

    let keys_and_locks_input: Vec<Vec<String>> = input_string.lines()
        .map(String::from)
        .chunks(8)
        .into_iter()
        .map(|chunk| chunk.take(7).collect())
        .collect();

    for element in &keys_and_locks_input {
        let checkable_ch = element[0].chars().nth(0).unwrap();

        let mut heights = [0usize; 5];
        for i in 1..element.len() {
            let row = element[i].clone();
            for (j, ch) in row.chars().enumerate() {
                if ch == checkable_ch {
                    heights[j] += 1;
                }
            }
        }

        if checkable_ch == '#' {
            locks.push(heights.to_vec());
        } else {
            heights = <[usize; 5]>::try_from(heights.iter().map(|i| 5 - i).collect_vec()).unwrap();
            keys.push(heights.to_vec());
        }
    }
    let mut res = 0;

    for lock in &locks {
        for key in &keys {
            let column_match_count = lock.iter().zip(key.iter())
                .map(|(x, y)| x + y)
                .filter(|x| *x < 6)
                .count();

            if column_match_count == 5 {
                res += 1;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::day::day25::run_part_one;
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 3);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(25)), 3690);
    }
}
