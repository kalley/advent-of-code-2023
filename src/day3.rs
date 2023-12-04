use std::fs;

fn get_input() -> String {
    fs::read_to_string("./data/day3.txt").expect("read the file")
}

fn part1(input: &String) -> u32 {
    let mut part_numbers: Vec<u32> = vec![];
    let all_lines = input.lines();
    let rows: Vec<&str> = all_lines.clone().collect();

    for (row, line) in all_lines.enumerate() {
        let line_bytes = line.as_bytes();
        let digit_positions: Vec<(u32, usize, usize)> = line
            .chars()
            .enumerate()
            .filter(|(i, c)| {
                c.is_ascii_digit() && (*i == 0 || !line_bytes[(*i - 1)].is_ascii_digit())
            })
            .map(|(i, _)| {
                let mut counter = i;
                let mut str_number: Vec<u8> = vec![];

                loop {
                    if counter >= line_bytes.len() {
                        break;
                    }

                    let c = line_bytes[counter];
                    if c.is_ascii_digit() {
                        str_number.push(c);
                    } else {
                        break;
                    }

                    counter += 1;
                }

                (
                    String::from_utf8(str_number)
                        .unwrap()
                        .parse::<u32>()
                        .unwrap(),
                    i,
                    counter - 1,
                )
            })
            .collect();

        for (val, start, end) in digit_positions.iter() {
            if row != 0 {
                let prev_row = &rows.get(row - 1).unwrap();
                let first = if *start == 0 { *start } else { *start - 1 };
                let last = if *end + 1 >= prev_row.len() {
                    prev_row.len() - 1
                } else {
                    *end + 1
                };
                let possibilities = &prev_row[first..=last];

                if possibilities != ".".repeat(possibilities.len()) {
                    part_numbers.push(*val);
                    continue;
                }
            }

            if *start != 0 {
                if &line[(start - 1)..*start] != "." {
                    part_numbers.push(*val);
                    continue;
                }
            }

            if *end + 1 != line.len() {
                if &line[*end + 1..*end + 2] != "." {
                    part_numbers.push(*val);
                    continue;
                }
            }

            if row != rows.len() - 1 {
                let prev_row = &rows.get(row + 1).unwrap();
                let first = if *start == 0 { *start } else { *start - 1 };
                let last = if *end + 1 >= prev_row.len() {
                    prev_row.len() - 1
                } else {
                    *end + 1
                };
                let possibilities = &prev_row[first..=last];

                if possibilities != ".".repeat(possibilities.len()) {
                    part_numbers.push(*val);
                    continue;
                }
            }
        }
    }

    part_numbers.into_iter().sum::<u32>()
}

fn part2(_input: &String) -> u32 {
    0
}

pub fn answer() {
    let input = get_input();

    println!("Day 3 part 1: {}", part1(&input));
    println!("Day 3 part 2: {}", part2(&input));
}
