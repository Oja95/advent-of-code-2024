use crate::day::utils;

pub fn run() {
    let input_string = utils::read_input(4);
    let part_one_result = run_part_one(&input_string);
    println!("{}", part_one_result);
    let part_two_result = run_part_two(&input_string);
    println!("{}", part_two_result);
}

type Matrix = Vec<Vec<char>>;

fn access(matrix: &Matrix, x: usize, y: usize) -> char {
    // I created this so I don't get confused how to access cartesian coordinates in a matrix :)
    matrix[y][x]
}

fn run_part_one(input_string: &str) -> u64 {
    let matrix = input_into_matrix(input_string);

    let mut count = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            count += scan_pos_xmas(&matrix, x, y);
        }
    }

    count
}

fn input_into_matrix(input_string: &str) -> Vec<Vec<char>> {
    let mut matrix = Matrix::new();

    input_string.lines()
        .for_each(|line| matrix.push(line.chars().into_iter().collect()));
    matrix
}

fn scan_pos_xmas(matrix: &Matrix, x: usize, y: usize) -> u64 {
    let mut pos_count = 0;

    // TODO: Consider making it neater by having some Enum dictate the access direction?
    let char_at_coordinates = access(matrix, x, y);
    if x + 3 < matrix[0].len() {
        let char_at_coordinates_2 = access(matrix, x + 1, y);
        let char_at_coordinates_3 = access(matrix, x + 2, y);
        let char_at_coordinates_4 = access(matrix, x + 3, y);
        let result = format!("{}{}{}{}", char_at_coordinates, char_at_coordinates_2, char_at_coordinates_3, char_at_coordinates_4);
        if result == "XMAS" || result == "SAMX" {
            pos_count += 1;
        }
    }

    if x + 3 < matrix[0].len() && y + 3 < matrix.len() {
        let char_at_coordinates_2 = access(matrix, x + 1, y + 1);
        let char_at_coordinates_3 = access(matrix, x + 2, y + 2);
        let char_at_coordinates_4 = access(matrix, x + 3, y + 3);
        let result = format!("{}{}{}{}", char_at_coordinates, char_at_coordinates_2, char_at_coordinates_3, char_at_coordinates_4);
        if result == "XMAS" || result == "SAMX" {
            pos_count += 1;
        }
    }

    if x + 3 < matrix[0].len() && y >= 3 {
        let char_at_coordinates_2 = access(matrix, x + 1, y - 1);
        let char_at_coordinates_3 = access(matrix, x + 2, y - 2);
        let char_at_coordinates_4 = access(matrix, x + 3, y - 3);
        let result = format!("{}{}{}{}", char_at_coordinates, char_at_coordinates_2, char_at_coordinates_3, char_at_coordinates_4);
        if result == "XMAS" || result == "SAMX" {
            pos_count += 1;
        }
    }

    if y + 3 < matrix.len() {
        let char_at_coordinates_2 = access(matrix, x, y + 1);
        let char_at_coordinates_3 = access(matrix, x, y + 2);
        let char_at_coordinates_4 = access(matrix, x, y + 3);
        let result = format!("{}{}{}{}", char_at_coordinates, char_at_coordinates_2, char_at_coordinates_3, char_at_coordinates_4);
        if result == "XMAS" || result == "SAMX" {
            pos_count += 1;
        }
    }

    pos_count
}

fn run_part_two(input_string: &str) -> u64 {
    let matrix = input_into_matrix(input_string);

    let mut count = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            if scan_pos_xmas_part_two(&matrix, x, y) {
                count += 1;
            }
        }
    }

    count
}

fn scan_pos_xmas_part_two(matrix: &Matrix, x: usize, y: usize) -> bool {
    if x + 2 >= matrix[0].len() || y + 2 >= matrix.len() {
        return false;
    }

    if access(matrix, x + 1, y + 1) != 'A' {
        return false;
    }

    if access(matrix, x + 2, y + 2) == access(matrix, x, y) {
        return false;
    }

    let mut chars = [access(matrix, x, y), access(matrix, x + 2, y), access(matrix, x + 2, y + 2),
        access(matrix, x, y + 2)];
    chars.sort();
    chars == ['M', 'M', 'S', 'S']
}

#[cfg(test)]
mod tests {
    use crate::day::day04::{run_part_one, run_part_two};
    use crate::day::utils;

    fn example_input() -> String {
        String::from("\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX")
    }

    #[test]
    fn test_exercise_example_part_one() {
        assert_eq!(run_part_one(&example_input()), 18);
    }

    #[test]
    fn test_day2_input_part_one() {
        assert_eq!(run_part_one(&utils::read_input(4)), 2434);
    }

    #[test]
    fn test_exercise_example_part_two() {
        assert_eq!(run_part_two(&example_input()), 9);
    }

    #[test]
    fn test_day2_input_part_two() {
        assert_eq!(run_part_two(&utils::read_input(4)), 1835);
    }
}
