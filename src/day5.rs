use std::{collections::HashMap, fs, ops::Range, str::Lines};

fn get_input() -> String {
    fs::read_to_string("./data/day5.txt").expect("read the file")
}

fn get_maps(mut lines: Lines) -> Vec<HashMap<u64, Range<u64>>> {
    let mut maps = vec![];

    loop {
        let line = match lines.next() {
            Some("") => continue,
            Some(val) => val,
            None => break,
        };

        // Map identifier
        if line.contains(':') {
            let mut map: HashMap<u64, Range<u64>> = HashMap::new();

            loop {
                match lines.next() {
                    Some("") => break,
                    Some(v) => {
                        if let Some((dest_start, rest)) = v.split_once(' ') {
                            if let Some((src_start, length)) = rest.split_once(' ') {
                                let range_len = length.trim().parse::<u64>().unwrap();
                                let src_range_start = src_start.trim().parse::<u64>().unwrap();
                                let dest_range_start = dest_start.trim().parse::<u64>().unwrap();

                                map.insert(
                                    dest_range_start,
                                    src_range_start..(src_range_start + range_len),
                                );
                            }
                        } else {
                            break;
                        }
                    }
                    None => break,
                };
            }
            maps.push(map);
        }
    }

    maps
}

fn part1(input: &String) -> u64 {
    let mut lines = input.lines();

    let mut seeds: Vec<u64> = if let Some((_, sources)) = lines.next().unwrap().split_once(':') {
        sources
            .trim()
            .split(' ')
            .map(|source| source.trim().parse::<u64>().unwrap())
            .collect()
    } else {
        return 0;
    };

    let maps = get_maps(lines);

    for map in maps.iter() {
        for i in 0..seeds.len() {
            let seed = seeds.get(i).unwrap();
            for (d, s) in map.iter() {
                if s.contains(seed) {
                    let dest = d + (*seed - s.start);

                    seeds[i] = dest;
                    break;
                }
            }
        }
    }

    *seeds.iter().min().unwrap()
}

fn part2(input: &String) -> u64 {
    let mut lines = input.lines();

    let mut seeds: Vec<u64> = if let Some((_, sources)) = lines.next().unwrap().split_once(':') {
        let mut sources_expanded = vec![];
        let mut sources_iter = sources.trim().split(' ');

        loop {
            let start = match sources_iter.next() {
                Some("") => break,
                Some(v) => v.trim().parse::<u64>().unwrap(),
                None => break,
            };
            let length = match sources_iter.next() {
                Some("") => break,
                Some(v) => v.trim().parse::<u64>().unwrap(),
                None => break,
            };

            for i in 0..length {
                sources_expanded.push(start + i);
            }
        }

        sources_expanded
    } else {
        return 0;
    };

    let maps = get_maps(lines);

    for i in 0..seeds.len() {
        for map in maps.iter() {
            for (d, s) in map.iter() {
                let seed = seeds.get(i).unwrap();

                if s.contains(&seed) {
                    let dest = d + (*seed - s.start);

                    seeds[i] = dest;
                    break;
                }
            }
        }
    }

    *seeds.iter().min().unwrap()
}

pub fn answer() {
    let input = get_input();

    println!("Day 5 part 1: {}", part1(&input));
    // Commenting out because this is super slow
    // println!("Day 5 part 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use std::io;

    use super::*;

    #[test]
    fn validate_part1() -> io::Result<()> {
        let input = get_input();

        assert_eq!(part1(&input), 389056265);

        Ok(())
    }

    #[test]
    fn validate_part2() -> io::Result<()> {
        let input = get_input();

        assert_eq!(part2(&input), 137516820);

        Ok(())
    }
}
