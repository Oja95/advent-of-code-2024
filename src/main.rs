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
        "10" => day10::run(),
        "11" => day11::run(),
        "12" => day12::run(),
        "13" => day13::run(),
        "14" => day14::run(),
        "15" => day15::run(),
        "16" => day16::run(),
        "17" => day17::run(),
        "18" => day18::run(),
        "19" => day19::run(),
        "20" => day20::run(),
        "21" => day21::run(),
        "22" => day22::run(),
        "23" => day23::run(),
        "24" => day24::run(),
        _ => println!("Day not implemented!"),
    }
}
