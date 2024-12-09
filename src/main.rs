use crate::day::*;

mod day;

fn main() {
    let day_to_run = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "1".to_string());

    match day_to_run.as_str() {
        "1" => day01::run(),
        "2" => day02::run(),
        "3" => day03::run(),
        "4" => day04::run(),
        "5" => day05::run(),
        "6" => day06::run(),
        "7" => day07::run(),
        "8" => day08::run(),
        "9" => day09::run(),
        _ => println!("Day not implemented!"),
    }
}
