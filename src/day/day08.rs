use crate::day::utils;
use itertools::Itertools;
use std::collections::HashSet;

pub fn run() {
    let input_string = utils::read_input(8);
    println!("{}", run_part_one(&input_string));
    println!("{}", run_part_two(&input_string));
}

#[derive(Debug)]
struct Antenna {
    x: isize,
    y: isize,
    label: char,
}

impl Antenna {
    fn position_delta(&self, other: &Antenna) -> (isize, isize) {
        (other.x - self.x, other.y - self.y)
    }

    fn apply_n_times_delta(&self, (dx, dy): (isize, isize), n: isize) -> (isize, isize) {
        (self.x + n * dx, self.y + n * dy)
    }
}

fn simulate(input_string: &str, with_resonance: bool) -> usize {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    let mut antennas: Vec<Antenna> = Vec::new();

    let y_max = input_string.lines().count() as isize;
    let x_max = input_string.lines().nth(0).unwrap().chars().count() as isize;

    for (y, line) in input_string.lines().enumerate() {
        for (x, label) in line.char_indices() {
            if label != '.' {
                antennas.push(Antenna { x: x as isize, y: y as isize, label });
            }
        }
    }

    antennas.iter()
        .map(|antenna| antenna.label)
        .unique()
        .for_each(|antenna_label| {
            // for each unique antenna label, check the antennas for same label antennas, then calculate coordinate deltas between all permutations by 2.
            let permutations = antennas.iter()
                .filter(|antenna| antenna.label == antenna_label)
                .permutations(2);
            for permutation in permutations {
                let position = permutation[0];
                let other_position = permutation[1];
                let delta = position.position_delta(other_position);

                let mut delta_multiplier = if with_resonance { 1 } else { 2 };
                loop {
                    let antinode_pos = position.apply_n_times_delta(delta, delta_multiplier);
                    if antinode_pos.0 >= 0 && antinode_pos.0 < x_max && antinode_pos.1 >= 0 && antinode_pos.1 < y_max {
                        antinodes.insert(antinode_pos);
                    } else {
                        break;
                    }

                    if !with_resonance {
                        break;
                    }
                    delta_multiplier += 1;
                }
            }
        });

    antinodes.len()
}

fn run_part_one(input_string: &str) -> usize {
    simulate(input_string, false)
}

fn run_part_two(input_string: &str) -> usize {
    simulate(input_string, true)
}

#[cfg(test)]
mod tests {
    use crate::day::day08::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 14);
    }

    #[test]
    fn test_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(8)), 392);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input()), 34);
    }

    #[test]
    fn test_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(8)), 1235);
    }
}
