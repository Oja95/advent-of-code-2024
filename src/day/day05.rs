use crate::day::utils;

pub fn run() {
    let input_string = utils::read_input(5);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}


struct Manual {
    order: Vec<(usize, usize)>,
    updates: Vec<Vec<usize>>
}

fn run_part_one(input_string: &str) -> usize {
    let manual = parse_input(input_string);

    // for each element
    manual.updates.iter()
        .filter(|update| find_fault_index(&update, &manual.order) == -1)
        // Assumes that all manual updates have odd number of items
        .map(|update| update.get(update.len() / 2).unwrap())
        .sum()
}

fn find_fault_index(update: &Vec<usize>, orderings: &Vec<(usize, usize)>) -> isize {
    for i in 0..update.len() {
        let elem = update.get(i).unwrap();

        let (left, right) = update.split_at(i);

        // for every element in left there must NOT be an ordering rule where elem|left_elem
        // for every element in right there must NOT be an ordering rule where right_elem|elem
        for left_elem in left {
            for (before, after) in orderings {
                if before == elem && after == left_elem {
                    return i as isize;
                }
            }
        }

        for right_elem in right {
            for (before, after) in orderings {
                if before == right_elem && after == elem {
                    return i as isize;
                }
            }
        }
    }

    -1
}

fn parse_input(input_string: &str) -> Manual {
    let mut parsing_pages = false;

    let mut manual = Manual {
        order: vec![],
        updates: vec![],
    };

    for line in input_string.lines() {
        match line.trim() {
            "" => parsing_pages = true,
            _ => match parsing_pages {
                false => {
                    let mut split = line.split("|");
                    let first = split.next().unwrap().parse().unwrap();
                    let second = split.next().unwrap().parse().unwrap();
                    manual.order.push((first, second));
                },
                true => {
                    let split = line.split(",");
                    manual.updates.push(split.map(|x| x.parse().unwrap()).collect());
                }
            },
        }
    }

    manual
}

fn run_part_two(input_string: &str) -> usize {
    let manual = parse_input(input_string);

    manual.updates.iter()
        .filter(|update| find_fault_index(&update, &manual.order) != -1)
        .map(|update| fix_update_ordering(&update, &manual.order))
        .map(|update| {
            *update.get(update.len() / 2).unwrap()
        })
        .sum()
}

// turbo slow brute force, almost as bad as randomizing the list each iteration, then checking if order is correct
fn fix_update_ordering(update: &Vec<usize>, orderings: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut new_vec = update.clone();

    let mut i = find_fault_index(&new_vec, orderings);
    while i != -1 {
        let faulty_elem = new_vec[i as usize];

        for left_index in 0..i as usize {
            let left_elem = new_vec[left_index];
            for &(before, after) in orderings {
                if before == faulty_elem && after == left_elem {
                    let index_a = i as usize;

                    // oh no
                    let mut cloned_vec = new_vec.clone();
                    cloned_vec.swap(index_a, left_index);
                    new_vec = cloned_vec;
                    break;
                }
            }
        }

        for right_index in (i as usize + 1)..new_vec.len() {
            let right_elem = new_vec[right_index];
            for &(before, after) in orderings {
                if before == right_elem && after == faulty_elem {
                    let index_a = i as usize;
                    let mut cloned_vec = new_vec.clone();

                    cloned_vec.swap(index_a, right_index);
                    new_vec = cloned_vec;
                    break;
                }
            }
        }

        i = find_fault_index(&new_vec, orderings);
    }

    new_vec
}

#[cfg(test)]
mod tests {
    use crate::day::day05::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 143);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(5)), 5391);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input()), 123);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(5)), 6142);
    }
}
