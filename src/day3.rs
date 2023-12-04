use core::str::Chars;
use std::fs;

fn get_input() -> String {
    fs::read_to_string("./data/day3.txt").expect("read the file")
}

fn part1(_input: &String) -> u32 {
    0
}

fn part2(_input: &String) -> u32 {
    0
}

pub fn answer() {
    let input = get_input();

    println!("Day 3 part 1: {}", part1(&input));
    println!("Day 3 part 2: {}", part2(&input));
}
