use crate::day::utils;
use itertools::Itertools;
use regex::Regex;
use std::cmp::PartialEq;
use std::collections::{HashMap, VecDeque};

pub fn run() {
    let input_string = utils::read_input(24);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Operator {
    AND,
    OR,
    XOR,
}

impl Operator {

    fn from_str(string: String) -> Operator {
        match string.as_str() {
            "AND" => Operator::AND,
            "OR" => Operator::OR,
            "XOR" => Operator::XOR,
            &_ => {panic!("Unknown operator")}
        }
    }

    fn apply(&self, left: usize, right: usize) -> usize {
        match self {
            Operator::AND => { left & right }
            Operator::OR => { left | right }
            Operator::XOR => { left ^ right }
        }
    }
}

fn run_part_one(input_string: &str) -> usize {
    let (mut memory, operations) = parse_input(input_string);

    let mut deque = VecDeque::from(operations);
    while let Some((first, operator, second, target)) = deque.pop_front() {
        let first_value = memory.get(&first);
        let second_value = memory.get(&second);

        if first_value.is_none() || second_value.is_none() {
            deque.push_back((first, operator, second, target));
            continue;
        }

        memory.insert(target, operator.apply(*first_value.unwrap(), *second_value.unwrap()));
    }

    bits_to_decimal("z", &memory)
}

fn bits_to_decimal(variable_name: &str, memory: &HashMap<String, usize>) -> usize {
    let mut keys = memory.keys()
        .filter(|&key| key.starts_with(variable_name))
        .cloned()
        .collect_vec();
    keys.sort();

    let mut result = 0;
    keys.iter().enumerate()
        .for_each(|(i, key)| {
            result += memory.get(key).unwrap() << i;
        });

    result
}

fn parse_input(input_string: &str) -> (HashMap<String, usize>, Vec<(String, Operator, String, String)>) {
    let mut memory: HashMap<String, usize> = HashMap::new();
    let mut operations: Vec<(String, Operator, String, String)> = vec![];

    let re: Regex = Regex::new(r"^(.*?): (\d+)$").unwrap();
    let re2: Regex = Regex::new(r"^(.*?) (.*?) (.*?) -> (.*?)$").unwrap();

    let mut process_operations = false;
    input_string.lines().for_each(|line| {
        match line {
            "" => {
                process_operations = true;
            }
            val => {
                if process_operations {
                    let captures = re2.captures(val).unwrap();
                    operations.push((captures[1].parse().unwrap(), Operator::from_str(captures[2].parse().unwrap()),
                                     captures[3].parse().unwrap(), captures[4].parse().unwrap()));
                } else {
                    let captures = re.captures(val).unwrap();
                    memory.insert(captures[1].parse().unwrap(), captures[2].parse::<usize>().unwrap());
                }
            },
        }
    });
    (memory, operations)
}

fn decimal_to_bits(mut decimal: usize) -> Vec<u8> {
    let mut bits = Vec::new();

    while decimal > 0 {
        bits.push((decimal % 2) as u8);
        decimal /= 2;
    }

    bits.reverse();
    bits
}

fn run_part_two(input_string: &str) -> String {
    let (mut memory, operations) = parse_input(input_string);

    let y = bits_to_decimal("y", &memory);
    let x = bits_to_decimal("x", &memory);
    let z = x+y;

    println!("{} {} {}", x, y, z);
    // numbers in input 18519035633327 + 22805924725791 = 41324960359118

    println!("   {:?}", decimal_to_bits(x));
    println!("   {:?}", decimal_to_bits(y));
    println!("{:?}", decimal_to_bits(z));

    // let mut result = Vec::new();
    //
    // let mut carry = 0;
    // for i in 0..45 {
    //     let variable_x_name = format!("x{:02}", i);
    //     let variable_y_name = format!("y{:02}", i);
    //
    //     let x = memory.get(&variable_x_name).unwrap();
    //     let y = memory.get(&variable_y_name).unwrap();
    //
    //     let partial_result = *x ^ *y;
    //     let new_carry_1 = *x & *y;
    //
    //     let res = partial_result ^ carry;
    //     let new_carry_2 = partial_result & carry;
    //     result.push(res);
    //
    //     carry = new_carry_1 | new_carry_2;
    // }

    let mut faulty_targets = vec![];
    for (first, operator, second, target) in &operations {
        if target.starts_with("z") && *operator != Operator::XOR {
            if target != "z45" {
                println!("fault 1: {} {:?} {} -> {}", first, operator, second, target);
                faulty_targets.push(target.clone());
            }
        }

        if !target.starts_with("z")
            && (!first.starts_with("y") && !first.starts_with("x"))
            && *operator == Operator::XOR {
            println!("fault 2: {} {:?} {} -> {}", first, operator, second, target);
            faulty_targets.push(target.clone());
        }
    }

    // manually switch known faults in input, run p1 simulation and detect anomaly manually
    let mut deque = VecDeque::from(operations);
    while let Some((first, operator, second, target)) = deque.pop_front() {
        let first_value = memory.get(&first);
        let second_value = memory.get(&second);

        if first_value.is_none() || second_value.is_none() {
            deque.push_back((first, operator, second, target));
            continue;
        }

        memory.insert(target, operator.apply(*first_value.unwrap(), *second_value.unwrap()));
    }

    // 14 bit off, carry over fail?
    // y12 XOR x12 -> rbp (12 partial_resu)
    // y12 AND x12 -> bdd (12 new carry_1)
    // snr XOR rbp -> z12 (carry from 11 ^ part res)
    // snr AND rbp -> jjf (jjf is 12 new_carry_2)
    // bdd OR jjf -> fmk (fmk new carry for 13)
    // --
    // y13 XOR x13 -> nfq (13 partial resu)
    // y13 AND x13 -> pdq (13 new_carry_1)
    // fmk XOR nfq -> z13
    // nfq AND fmk -> hmt (13 new_carry_2)
    // hmt OR pdq -> scs (scs new carry for 14)
    // --
    // x14 XOR y14 -> jss (14 partial resu) FAULT!
    // x14 AND y14 -> rds (14 new_carry_1)
    // scs AND rds -> dcv
    // scs XOR rds -> z14 (rds has to be partial result x14 XOR y14)
    //
    // dcv OR jss -> tsg
    // jss <-> rds!

    faulty_targets.push("jss".to_string());
    faulty_targets.push("rds".to_string());

    faulty_targets.sort();
    let faulty_targets_string = faulty_targets.join(",");
    println!("{:?}", faulty_targets_string);

    // let i = bits_to_decimal("z", &memory);
    // let vec1 = decimal_to_bits(i);
    // println!("{}", i);
    // println!("{:?}", vec1);
    //
    // "1".to_string()
    faulty_targets_string
}

#[cfg(test)]
mod tests {
    use crate::day::day24::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02")
    }

    fn example_input_2() -> String {
        String::from("\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 4);
        assert_eq!(run_part_one(&example_input_2()), 2024);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(24)), 41324968993486);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(24)), "bmn,jss,mvb,rds,wss,z08,z18,z23");
    }
}
