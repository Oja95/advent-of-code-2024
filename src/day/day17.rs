use crate::day::utils;
use itertools::Itertools;
use regex::Regex;

pub fn run() {
    let input_string = utils::read_input(17);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}

fn run_part_one(input_string: &str) -> usize {
    let re: Regex = Regex::new(r"Register .: (\d+)").unwrap();

    let mut lines = input_string.lines();
    let register_a_str = lines.next().unwrap();
    let mut a = re.captures(register_a_str).unwrap()[1].parse::<usize>().unwrap();

    let register_b_str = lines.next().unwrap();
    let mut b = re.captures(register_b_str).unwrap()[1].parse::<usize>().unwrap();

    let register_c_str = lines.next().unwrap();
    let mut c = re.captures(register_c_str).unwrap()[1].parse::<usize>().unwrap();

    lines.next();
    let program_re: Regex = Regex::new(r"Program: (.*)").unwrap();
    let program = program_re.captures(lines.next().unwrap()).unwrap()[1].split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect_vec();

    let mut instruction_pointer = 0;
    let mut out = 0;

    loop {
        if instruction_pointer >= program.len() {
            break;
        }

        let opcode = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];
        match opcode {
            0 => { a = (a as u32 / 2_u32.pow(operand_value(operand, a, b, c) as u32)) as usize; },
            1 => { b = b ^ operand; },
            2 => { b = operand_value(operand, a, b, c) % 8; },
            3 => {
                if a != 0 {
                    instruction_pointer = operand;
                }
            },
            4 => { b = b ^ c; },
            5 => { out = out * 10 + (operand_value(operand, a, b, c) % 8); },
            6 => { b = (a as u32 / 2_u32.pow(operand_value(operand, a, b, c) as u32)) as usize; },
            7 => { c = (a as u32 / 2_u32.pow(operand_value(operand, a, b, c) as u32)) as usize; }
            _ => { panic!("Unknown opcode {}", opcode); }
        }

        if opcode != 3 {
            instruction_pointer += 2;
        } else {
            if a == 0 {
                instruction_pointer += 2;
            }
        }
    }

    out
}

fn operand_value(operand: usize, a: usize, b: usize, c: usize) -> usize {
    match operand {
        4 => a,
        5 => b,
        6 => c,
        7 => {panic!("illegal operand: 7")}
        op => {
            if op <= 3 {
                op
            } else {
                panic!("illegal operand: {}", op);
            }
        }
    }
}

fn run_part_two(input_string: &str) -> usize {
    // Output: 2,4,1,2,7,5,0,3,4,7,1,7,5,5,3,0
    // A % 8 -> B
    // B ^ 2 -> B
         // (A % 8) ^ 2
    // A / 2**B -> C
         // A / 2**((A % 8) ^ 2) -> C
    // B ^ C -> B
         // ((A % 8) ^ 2) ^ (A / 2**((A % 8) ^ 2))
    // B ^ 7 -> B
         // (((A % 8) ^ 2) ^ (A / 2**((A % 8) ^ 2))) ^ 7
    // B % 8 -> OUT
         // ((((A % 8) ^ 2) ^ (A / 2**((A % 8) ^ 2))) ^ 7) % 8 == NUM

    // A / 8 -> 0 -> A < 8
    // 2nd round: A != 0, but 8 <= A < 8**2
    // 3nd round: A != 0, but 8**2 <= A < 8**3

    let mut lines = input_string.lines();
    lines.next(); lines.next(); lines.next(); lines.next();
    let program_re: Regex = Regex::new(r"Program: (.*)").unwrap();
    let mut program = program_re.captures(lines.next().unwrap()).unwrap()[1].split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect_vec();

    program.reverse();

    let mut accumulator: usize = 0;
    for res in program {
        for i in 0..8 {
            let j = 8 * accumulator + i;
            // puzzle input specific solution, should generalize by running program in reverse?
            // ((((A % 8) ^ 2) ^ (A / 2**((A % 8) ^ 2))) ^ 7) % 8 == NUM
            let i1 = ((((j % 8) ^ 2) ^ (j / 2_usize.pow(((j % 8) ^ 2) as u32))) ^ 7) % 8;
            if i1 == res {
                accumulator = 8 * accumulator + i;
                break;
            }
            if i == 7 {
                panic!("No solution!");
            }
        }
    }

    // 190384113204239
    accumulator
}

#[cfg(test)]
mod tests {
    use crate::day::day17::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 4635635210);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(17)), 713751034);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(17)), 190384113204239);
    }
}
