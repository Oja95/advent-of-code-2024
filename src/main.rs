use crate::day::day01;

mod day;

fn main() {
    let day_to_run = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "1".to_string());

    match day_to_run.as_str() {
        "1" => day01::run(),
        _ => println!("Day not implemented!"),
    }
}
