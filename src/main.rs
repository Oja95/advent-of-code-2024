use crate::day::{day01, day02};

mod day;

fn main() {
    let day_to_run = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "2".to_string());

    match day_to_run.as_str() {
        "1" => day01::run(),
        "2" => day02::run(),
        _ => println!("Day not implemented!"),
    }
}
