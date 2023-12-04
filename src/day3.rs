use std::{collections::HashMap, fs, ops::Range};

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
                let next_row = &rows.get(row + 1).unwrap();
                let first = if *start == 0 { *start } else { *start - 1 };
                let last = if *end + 1 >= next_row.len() {
                    next_row.len() - 1
                } else {
                    *end + 1
                };
                let possibilities = &next_row[first..=last];

                if possibilities != ".".repeat(possibilities.len()) {
                    part_numbers.push(*val);
                    continue;
                }
            }
        }
    }

    part_numbers.into_iter().sum::<u32>()
}

fn part2(input: &String) -> u32 {
    let mut gear_ratios: Vec<u32> = vec![];
    let all_lines = input.lines();
    let rows: Vec<&str> = all_lines.clone().collect();
    let mut all_digit_positions: Vec<HashMap<Range<usize>, u32>> = vec![];

    for line in rows.iter() {
        let line_bytes = line.as_bytes();
        let digit_positions: HashMap<Range<usize>, u32> = line
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
                    i..counter,
                    String::from_utf8(str_number)
                        .unwrap()
                        .parse::<u32>()
                        .unwrap(),
                )
            })
            .collect();

        all_digit_positions.push(digit_positions);
    }

    for (row, line) in all_lines.enumerate() {
        let potential_gear_positions: Vec<(usize, char)> = line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '*')
            .collect();
        let cur_digit_pos = all_digit_positions.get(row).unwrap();
        let prev_digit_pos = if row == 0 {
            None
        } else {
            all_digit_positions.get(row - 1)
        };
        let next_digit_pos = all_digit_positions.get(row + 1);

        for (pos, _) in potential_gear_positions.iter() {
            let mut gear_parts: Vec<u32> = vec![];

            if let Some(digit_pos) = prev_digit_pos {
                for (k, v) in digit_pos {
                    if *pos > 0 {
                        if k.contains(&(pos - 1)) {
                            gear_parts.push(*v);

                            if k.end != *pos {
                                break;
                            }
                        }
                    }

                    if k.contains(pos) {
                        gear_parts.push(*v);

                        break;
                    }

                    if *pos + 1 != line.len() {
                        if k.contains(&(pos + 1)) {
                            gear_parts.push(*v);
                        }
                    }
                }
            }

            if *pos != 0 {
                for (k, v) in cur_digit_pos {
                    if k.contains(&(pos - 1)) {
                        gear_parts.push(*v);
                    }
                }
            }

            if *pos + 1 != line.len() {
                for (k, v) in cur_digit_pos {
                    if k.contains(&(pos + 1)) {
                        gear_parts.push(*v);
                    }
                }
            }

            if let Some(digit_pos) = next_digit_pos {
                for (k, v) in digit_pos {
                    if *pos > 0 {
                        if k.contains(&(pos - 1)) {
                            gear_parts.push(*v);

                            if k.end != *pos {
                                break;
                            }
                        }
                    }

                    if k.contains(pos) {
                        gear_parts.push(*v);

                        break;
                    }

                    if *pos + 1 != line.len() {
                        if k.contains(&(pos + 1)) {
                            gear_parts.push(*v);
                        }
                    }
                }
            }

            if gear_parts.len() == 2 {
                gear_ratios.push(gear_parts[0] * gear_parts[1]);
            }
        }
    }

    gear_ratios.into_iter().sum::<u32>()
}

pub fn answer() {
    let input = get_input();

    println!("Day 3 part 1: {}", part1(&input));
    println!("Day 3 part 2: {}", part2(&input));
}
