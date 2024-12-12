use std::collections::{HashMap, VecDeque};
use glam::{IVec2, UVec2};
use crate::day::utils;
use crate::day::utils::Matrix;

pub fn run() {
    let input_string = utils::read_input(12);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}

static NEIGHBOUR_DELTAS: [IVec2; 4] = [
    IVec2::new(0, 1),
    IVec2::new(1, 0),
    IVec2::new(0, -1),
    IVec2::new(-1, 0),
];

fn run_part_one(input_string: &str) -> usize {
    let mut matrix = Matrix::new();

    input_string.lines()
        .for_each(|line| matrix.push(line.chars().into_iter().collect()));

    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];

    let mut regions = Vec::new();

    for (y, row) in matrix.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {

            if !visited[y][x] {
                let mut section_area = 0;
                let mut section_perimeter = 0;

                let mut queue = VecDeque::new();
                queue.push_back(IVec2 { x: x as i32, y: y as i32 });

                while let Some(position) = queue.pop_front() {
                    if visited[position.y as usize][position.x as usize] {
                        continue;
                    }
                    visited[position.y as usize][position.x as usize] = true;
                    section_area += 1;

                    for delta in NEIGHBOUR_DELTAS {
                        let neighbour = position + delta;
                        if neighbour.x < 0 || neighbour.y < 0 || neighbour.y >= matrix.len() as i32 || neighbour.x >= matrix[0].len() as i32 {
                            section_perimeter += 1;
                            continue;
                        }

                        if matrix[neighbour.y as usize][neighbour.x as usize] == matrix[position.y as usize][position.x as usize] {
                            if !visited[neighbour.y as usize][neighbour.x as usize] {
                                queue.push_back(neighbour);
                            }
                        } else {
                            section_perimeter += 1;
                        }
                    }
                }

                regions.push((section_area, section_perimeter));
            }
        }
    }

    regions.iter().map(|(area, perimeter)| {
        area * perimeter
    }).sum()
}

fn run_part_two(input_string: &str) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use crate::day::day12::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
AAAA
BBCD
BBCC
EEEC")
    }

    fn example_input_2() -> String {
        String::from("\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO")
    }

    fn example_input_3() -> String {
        String::from("\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 140);
        assert_eq!(run_part_one(&example_input_2()), 772);
        assert_eq!(run_part_one(&example_input_3()), 1930);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(12)), 1434856);
    }

    fn example_input_4() -> String {
        String::from("\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE")
    }

    fn example_input_5() -> String {
        String::from("\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA")
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input()), 80);
        assert_eq!(run_part_two(&example_input_2()), 436);
        assert_eq!(run_part_two(&example_input_3()), 1206);
        assert_eq!(run_part_two(&example_input_4()), 236);
        assert_eq!(run_part_two(&example_input_5()), 368);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(12)), 1255);
    }
}
