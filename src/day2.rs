use std::fs;

fn get_input() -> String {
    fs::read_to_string("./data/day2.txt").expect("read the file")
}

fn part1(input: &String) -> u32 {
    let mut values: Vec<u32> = vec![];

    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();

        if parts.len() < 1 {
            continue;
        }

        let id = match parts.get(0) {
            Some(game) => {
                let str_id: &str = game
                    .trim()
                    .split(' ')
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap();

                str_id.parse::<u32>().unwrap()
            }
            None => continue,
        };

        let sets: Vec<&str> = parts.get(1).unwrap().trim().split(';').collect();
        let mut possible = true;

        for set in sets.into_iter() {
            let grabs: Vec<&str> = set.trim().split(", ").collect();

            if !possible {
                break;
            }

            for grab in grabs {
                if let Some((amount, color)) = grab.split_once(' ') {
                    let amt = amount.parse::<u32>().unwrap();

                    if possible && (color == "red" && amt > 12)
                        || (color == "green" && amt > 13)
                        || (color == "blue" && amt > 14)
                    {
                        possible = false;
                        break;
                    }
                }
            }
        }

        if possible {
            values.push(id);
        }
    }

    values.into_iter().sum::<u32>()
}

pub fn answer() {
    let input = get_input();

    println!("Day 2 part 1: {}", part1(&input));
    // println!("Day 2 part 2: {}", part2(&input));
}
