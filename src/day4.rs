use std::{collections::HashMap, fs};

fn get_input() -> String {
    fs::read_to_string("./data/day4.txt").expect("read the file")
}

fn get_matches(card: &str) -> Option<(u32, Vec<&str>)> {
    let (card_number, numbers) = if let Some((card, numbers)) = card.split_once(':') {
        (card.trim(), numbers.trim())
    } else {
        return None;
    };
    let (winning, mine): (Vec<&str>, Vec<&str>) =
        if let Some((winning, mine)) = numbers.trim().split_once('|') {
            (
                winning.trim().split(' ').filter(|v| *v != "").collect(),
                mine.trim().split(' ').filter(|v| *v != "").collect(),
            )
        } else {
            return None;
        };

    Some((
        card_number
            .replace("Card", "")
            .trim()
            .parse::<u32>()
            .unwrap(),
        mine.into_iter()
            .filter(|num| winning.contains(num))
            .collect::<Vec<_>>(),
    ))
}

fn part1(input: &String) -> u32 {
    let mut points: Vec<u32> = vec![];

    for line in input.lines() {
        let (_, matches) = if let Some(m) = get_matches(line) {
            m
        } else {
            continue;
        };

        let mut value = if matches.len() > 0 { 1 } else { 0 };

        for _ in 1..matches.len() {
            value = value * 2;
        }

        points.push(value);
    }

    points.into_iter().sum::<u32>()
}

fn part2(input: &String) -> u32 {
    let mut scratchcards: HashMap<u32, u32> = HashMap::new();

    for card in input.lines() {
        let (card_number, matches) = if let Some(m) = get_matches(card) {
            m
        } else {
            continue;
        };

        if !scratchcards.contains_key(&card_number) {
            scratchcards.insert(card_number, 1);
        }

        let current_scratchcards = scratchcards.clone();
        let times = current_scratchcards.get(&card_number).unwrap();
        let mut counter = 1;

        for _ in matches {
            let number = card_number + counter;
            let current_count = scratchcards.get(&number).unwrap_or(&1);

            scratchcards.insert(number, current_count + times);
            counter += 1;
        }
    }

    scratchcards.values().into_iter().sum::<u32>()
}

pub fn answer() {
    let input = get_input();

    println!("Day 4 part 1: {}", part1(&input));
    println!("Day 4 part 2: {}", part2(&input));
}
