use crate::day::utils;
use crate::day::utils::Matrix;
use glam::IVec2;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

pub fn run() {
    let input_string = utils::read_input(15);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}

fn run_part_one(input_string: &str) -> usize {
    let mut map: Matrix = Matrix::new();
    let mut directions: Vec<char> = Vec::new();

    let mut parsing_directions = false;
    let mut lines = input_string.lines();

    let mut current_pos = Default::default();
    let mut y = 0;

    while let Some(line) = lines.next() {
        if line == "" {
            parsing_directions = true;
            continue;
        }

        if parsing_directions {
            directions.append(&mut line.chars().collect_vec());
        } else {
            map.push(line.chars().enumerate()
                .map(|elem| {
                    if elem.1 == '@' {
                        current_pos = IVec2::new(elem.0 as i32, y);
                    }
                    elem.1
                })
                .collect_vec());
            y += 1;
        }
    }

    let deltas = HashMap::from([
        ('^', IVec2::from((0, -1))),
        ('>', IVec2::from((1, 0))),
        ('<', IVec2::from((-1, 0))),
        ('v', IVec2::from((0, 1)))
    ]);

    for direction in directions {
        let delta = deltas.get(&direction).unwrap();
        let new_pos = current_pos + delta;

        // assume walls around input so index cant be negative
        match map[new_pos.y as usize][new_pos.x as usize] {
            '.' => {
                map[current_pos.y as usize][current_pos.x as usize] = '.';
                map[new_pos.y as usize][new_pos.x as usize] = '@';
                current_pos = new_pos;
            },
            'O' => {
                let mut pushable_blocks = Vec::new();
                pushable_blocks.push(new_pos);
                let mut subsequent_pos = new_pos;
                let mut can_push = false;
                loop {
                    subsequent_pos = subsequent_pos + delta;
                    match map[subsequent_pos.y as usize][subsequent_pos.x as usize] {
                        '#' => {
                            // can't push
                            break;
                        },
                        'O' => {
                            pushable_blocks.push(subsequent_pos);
                        },
                        '.' => {
                            can_push = true;
                            break;
                        },
                        _ => {
                            panic!("unexpected element in map");
                        }
                    }
                }

                if can_push {
                    for pushable_block in pushable_blocks {
                        map[(pushable_block.y + delta.y) as usize][(pushable_block.x + delta.x) as usize] = 'O';
                    }
                    map[new_pos.y as usize][new_pos.x as usize] = '@';
                    map[current_pos.y as usize][current_pos.x as usize] = '.';
                    current_pos = new_pos;
                }
            }
            '#' => {
                // no-op
            },
            elem => {
                panic!("unexpected element in map: {}", elem);
            }
        }
    }

    let mut result = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            if *elem == 'O' {
                result += 100 * y + x;
            }
        }
    }

    result
}

fn run_part_two(input_string: &str) -> usize {
    let mut map: Matrix = Matrix::new();
    let mut directions: Vec<char> = Vec::new();

    let mut parsing_directions = false;
    let mut lines = input_string.lines();

    let mut current_pos = Default::default();
    let mut y = 0;

    while let Some(line) = lines.next() {
        if line == "" {
            parsing_directions = true;
            continue;
        }

        if parsing_directions {
            directions.append(&mut line.chars().collect_vec());
        } else {
            map.push(line.chars().enumerate()
                .flat_map(|elem| {
                    match elem.1 {
                        '#' => {vec!['#', '#']},
                        '@' => {
                            current_pos = IVec2::new((2 * elem.0) as i32, y);
                            vec!['@', '.']
                        },
                        '.' => {vec!['.', '.']}
                        'O' => {vec!['[', ']']}
                        wat => {
                            panic!("unexpected element in input: {}", wat);
                        }
                    }
                })
                .collect_vec());
            y += 1;
        }
    }

    let deltas = HashMap::from([
        ('^', IVec2::from((0, -1))),
        ('>', IVec2::from((1, 0))),
        ('<', IVec2::from((-1, 0))),
        ('v', IVec2::from((0, 1)))
    ]);

    for direction in directions {
        let delta = deltas.get(&direction).unwrap();
        let new_pos = current_pos + delta;

        // assume walls around input so index cant be negative
        match map[new_pos.y as usize][new_pos.x as usize] {
            '.' => {
                map[current_pos.y as usize][current_pos.x as usize] = '.';
                map[new_pos.y as usize][new_pos.x as usize] = '@';
                current_pos = new_pos;
            },
            '[' | ']' => {
                let mut pushable_blocks = Vec::new();
                let mut can_push;

                // if delta horizontal, push like in p1
                if delta.y == 0 {
                    pushable_blocks.push(new_pos);
                    can_push = false;
                    let mut subsequent_pos = new_pos;
                    loop {
                        subsequent_pos = subsequent_pos + delta;
                        match map[subsequent_pos.y as usize][subsequent_pos.x as usize] {
                            '#' => {
                                // can't push
                                break;
                            },
                            '[' | ']' => {
                                pushable_blocks.push(subsequent_pos);
                            },
                            '.' => {
                                can_push = true;
                                break;
                            },
                            wat => {
                                panic!("unexpected element in map: {}", wat);
                            }
                        }
                    }
                } else {
                    // if delta vertical, do BFS to find IF it is pushable and what are the blocks to push
                    let mut visited = Vec::new();
                    let mut queue = VecDeque::new();
                    queue.push_back(new_pos);
                    can_push = true; // by default assume can push until proven otherwise
                    while let Some(position) = queue.pop_front() {
                        visited.push(position);
                        match map[position.y as usize][position.x as usize] {
                            '#' => {
                                // cant push any crates
                                can_push = false;
                                break;
                            },
                            '[' => {
                                pushable_blocks.push(position);
                                // also add its partner at x + 1 to queue and position at +delta
                                let neighbour = position + IVec2::from((1, 0));
                                if !visited.contains(&neighbour) {
                                    queue.push_back(neighbour);
                                }
                                let movable_position = position + delta;
                                if !visited.contains(&movable_position) {
                                    queue.push_back(movable_position);
                                }
                            },
                            ']' => {
                                pushable_blocks.push(position);
                                // also add its partner at x - 1 and + position at +delta
                                let neighbour = position + IVec2::from((-1, 0));
                                if !visited.contains(&neighbour) {
                                    queue.push_back(neighbour);
                                }
                                let movable_position = position + delta;
                                if !visited.contains(&movable_position) {
                                    queue.push_back(movable_position);
                                }
                            },
                            '.' => {
                                // all good
                            }
                            wat => {
                                panic!("unexpected element in map: {}", wat);
                            }
                        }

                        if !can_push {
                            break;
                        }
                    }
                }

                if can_push {
                    // reverse so it will process the boxes further away first so it doesnt overwrite
                    let mut new_block_positions = Vec::new();
                    for pushable_block in pushable_blocks.iter().rev() {
                        map[(pushable_block.y + delta.y) as usize][(pushable_block.x + delta.x) as usize] = map[pushable_block.y as usize][pushable_block.x as usize];
                        new_block_positions.push(IVec2::new(pushable_block.x + delta.x, pushable_block.y + delta.y));
                    }

                    // if any of the pushable blocks is not coinciding with the new pos, it needs a '.'
                    for pushable_block in pushable_blocks {
                        if !new_block_positions.contains(&pushable_block) {
                            map[pushable_block.y as usize][pushable_block.x as usize] = '.';
                        }
                    }

                    map[new_pos.y as usize][new_pos.x as usize] = '@';
                    map[current_pos.y as usize][current_pos.x as usize] = '.';
                    current_pos = new_pos;
                }
            },
            '#' => {
                // no-op
            },
            elem => {
                panic!("unexpected element in map: {}", elem);
            }
        }
    }

    let mut result = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            if *elem == '[' {
                result += 100 * y + x;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::day::day15::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv
<v>>v<<")
    }

    fn example_input_2() -> String {
        String::from("\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^")
    }

//     fn example_input_3() -> String {
//         String::from("\
// #######
// #...#.#
// #.....#
// #..OO@#
// #..O..#
// #.....#
// #######
//
// <vv<<^^<<^^")
//     }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 2028);
        assert_eq!(run_part_one(&example_input_2()), 10092);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(15)), 1478649);
    }

    #[test]
    fn test_exercise_example_part_two() {
        // assert_eq!(run_part_two(&example_input_3()), 1);
        assert_eq!(run_part_two(&example_input_2()), 9021);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(15)), 1495455);
    }
}
