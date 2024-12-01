use std::fs;

pub fn read_input(day: u8) -> String {
    let file_name = format!("input/day{:02}.txt", day);
    fs::read_to_string(file_name).expect("Failed to read input file")
}