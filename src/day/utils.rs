use std::fs;

pub fn read_input(day: u8) -> String {
    let file_name = format!("input/day{:02}.txt", day);
    fs::read_to_string(file_name).expect("Failed to read input file")
}

pub type Matrix = Vec<Vec<char>>;

pub fn access(matrix: &Matrix, x: usize, y: usize) -> char {
    // I created this so I don't get confused how to access cartesian coordinates in a matrix :)
    matrix[y][x]
}

pub fn input_into_matrix(input_string: &str) -> Vec<Vec<char>> {
    let mut matrix = Matrix::new();

    input_string.lines()
        .for_each(|line| matrix.push(line.chars().into_iter().collect()));
    matrix
}