use std::env;

mod day1;
mod day10;
mod day11;
// mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[2];

    match day.as_str() {
        "1" => day1::answer(),
        "2" => day2::answer(),
        "3" => day3::answer(),
        "4" => day4::answer(),
        "5" => day5::answer(),
        "6" => day6::answer(),
        "7" => day7::answer(),
        "8" => day8::answer(),
        "9" => day9::answer(),
        "10" => day10::answer(),
        "11" => day11::answer(),
        // "12" => day12::answer(),
        "13" => day13::answer(),
        "14" => day14::answer(),
        "15" => day15::answer(),
        "16" => day16::answer(),
        // "17" => day17::answer(),
        // "18" => day18::answer(),
        // "19" => day19::answer(),
        // "20" => day20::answer(),
        // "21" => day21::answer(),
        // "22" => day22::answer(),
        // "23" => day23::answer(),
        // "24" => day24::answer(),
        _ => unreachable!(),
    }
}
