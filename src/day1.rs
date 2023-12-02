use core::str::Chars;
use std::fs;

fn get_input() -> String {
    fs::read_to_string("./data/day1.txt").expect("read the file")
}

fn part1(input: &String) -> u32 {
    let mut values: Vec<u32> = vec![];

    for line in input.split("\n").into_iter() {
        let numbers: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();

        if numbers.len() == 0 {
            continue;
        }

        let first = numbers.first().unwrap().to_string();
        let last = numbers.last().unwrap().to_string();

        values.push(format!("{first}{last}").parse::<u32>().unwrap());
    }

    values.iter().sum::<u32>()
}

fn find_word_digits(chars: Chars, i: usize) -> Option<char> {
    let relevant: Vec<char> = chars
        .enumerate()
        .filter(|(k, _)| *k >= i && *k < i + 5)
        .map(|(_, v)| v)
        .collect();

    if relevant.len() < 3 {
        return None;
    }

    let first_three = &relevant[0..3];

    match first_three.into_iter().collect::<String>().as_str() {
        "one" => return Some('1'),
        "two" => return Some('2'),
        "six" => return Some('6'),
        _ => (),
    }

    if relevant.len() < 4 {
        return None;
    }

    let first_four = &relevant[0..4];

    match first_four.into_iter().collect::<String>().as_str() {
        "four" => return Some('4'),
        "five" => return Some('5'),
        "nine" => return Some('9'),
        _ => (),
    }

    if relevant.len() < 5 {
        return None;
    }

    Some(match relevant.into_iter().collect::<String>().as_str() {
        "three" => '3',
        "seven" => '7',
        "eight" => '8',
        _ => return None,
    })
}

fn part2(input: &String) -> u32 {
    let mut values: Vec<u32> = vec![];
    for line in input.split("\n").into_iter() {
        let chars = line.chars();
        let mut numbers: Vec<char> = vec![];
        for (i, c) in chars.clone().enumerate() {
            if c.is_ascii_digit() {
                numbers.push(c);
            } else if let Some(v) = find_word_digits(chars.clone(), i) {
                numbers.push(v);
            }
        }

        if numbers.len() == 0 {
            continue;
        }

        let first = numbers.first().unwrap().to_string();
        let last = numbers.last().unwrap().to_string();

        values.push(format!("{first}{last}").parse::<u32>().unwrap());
    }

    values.iter().sum::<u32>()
}

pub fn answer() {
    let input = get_input();

    println!("Day 1 part 1: {}", part1(&input));
    println!("Day 1 part 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use std::io;

    use super::*;

    #[test]
    fn validate_part1() -> io::Result<()> {
        let input = get_input();

        assert_eq!(part1(&input), 54331);

        Ok(())
    }

    #[test]
    fn validate_part2() -> io::Result<()> {
        let input = get_input();

        assert_eq!(part2(&input), 54518);

        Ok(())
    }
}
