use crate::day::utils;

pub fn run() {
    let input_string = utils::read_input(9);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}

fn run_part_one(input_string: &str) -> usize {
    let mut memory = parse_memory(input_string);

    let mut end_index = memory.len() - 1;
    let mut start_index = 0;

    loop {
        loop {
            if memory[start_index] != -1 && start_index < memory.len() {
                start_index += 1;
            } else {
                break;
            }
        }
        // start index now a free memory pos

        loop {
            if memory[end_index] == -1 {
                end_index -= 1;
            } else {
                break;
            }
        }
        // end index now first non-empty space elem at end

        if start_index >= end_index {
            break;
        }

        memory.swap(start_index, end_index);
    }

    calculate_checksum(&mut memory)
}

fn calculate_checksum(memory: &mut Vec<isize>) -> usize {
    memory.iter().enumerate()
        .filter(|(_, value)| **value != -1)
        .fold(0, |acc, (index, block)| acc + index * *block as usize)
}

fn parse_memory(input_string: &str) -> Vec<isize> {
    let mut memory: Vec<isize> = Vec::new();
    let mut file = true;
    let mut file_id = 0;
    input_string.chars().for_each(|c| {
        let next_input_char = c.to_digit(10).unwrap() as i8;
        match file {
            true => {
                memory.extend(vec![file_id; next_input_char as usize]);
                file_id += 1;
            }
            false => {
                memory.extend(vec![-1; next_input_char as usize]);
            }
        }
        file = !file;
    });
    memory
}

fn run_part_two(input_string: &str) -> usize {
    let mut memory = parse_memory(input_string);

    let mut end_index = memory.len() - 1;
    let mut file_length;
    let mut start_index;
    let mut empty_space_length;

    loop {
        start_index = 0;

        loop {
            if memory[end_index] == -1 {
                end_index -= 1;
            } else {
                break;
            }
        }
        // end index now first non-empty space elem at end
        file_length = 1;
        loop {
            // determine moveable file len
            if file_length <= end_index && memory[end_index - file_length] == memory[end_index] {
                file_length += 1;
            } else {
                break;
            }
        }

        // find starting place that could fit the file len
        loop {
            loop {
                if start_index < memory.len() && memory[start_index] != -1 {
                    start_index += 1;
                } else {
                    break;
                }
            }
            // start index now a free memory pos, check now many spaces are free
            empty_space_length = 1;
            loop {
                if start_index + empty_space_length < memory.len() && memory[start_index + empty_space_length] == -1 {
                    empty_space_length += 1
                } else {
                    break;
                }
            }
            if empty_space_length >= file_length {
                break;
            } else if start_index >= memory.len() {
                end_index -= file_length;
                file_length = 0;
                break;
            } else {
                start_index += empty_space_length;
            }
        }
        // found empty pos at start that fits end?

        // only swap if it isn't moved forward in the memory
        if end_index < start_index {
            if file_length > end_index {
                break;
            }
            end_index -= file_length;
            continue;
        }

        // swap file length amount of blocks
        for i in 0..file_length {
            memory.swap(start_index + i, end_index - i);
        }
    }

    calculate_checksum(&mut memory)
}

#[cfg(test)]
mod tests {
    use crate::day::day09::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("2333133121414131402")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 1928);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(9)), 6432869891895);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input()), 2858);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(9)), 6467290479134);
    }
}
