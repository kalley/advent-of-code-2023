use std::{collections::VecDeque, fs};

const DAY: &str = "12";

fn get_input() -> String {
    fs::read_to_string(format!("./data/day{DAY}.txt")).expect("read the file")
}

fn part1(input: &String) -> usize {
    let re = regex::Regex::new(r"\.+").unwrap();
    let mut num_arrangements = 0;

    for (i, line) in input.lines().enumerate() {
        if let Some((record, info)) = line.split_once(' ') {
            let mut possibly_damaged: VecDeque<&str> = re.split(record.trim_matches('.')).collect();
            let mut contiguous_group_counts: VecDeque<usize> = info
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect();

            loop {
                let possibility = possibly_damaged[0];
                let size = possibility.len();
                let is_known = possibility.chars().position(|c| c == '?').is_none();
                let expected_size = contiguous_group_counts[0];
                let pieces_align = size == contiguous_group_counts.len();

                if size < expected_size {
                    // Remove pieces that won't work at all from front
                    possibly_damaged.pop_front();
                } else if (is_known || pieces_align) && size == expected_size {
                    // Remove pieces that are definitely known (no variance) from front
                    possibly_damaged.pop_front();
                    contiguous_group_counts.pop_front();
                } else {
                    break;
                }
            }

            'outer: loop {
                let possibility = possibly_damaged[possibly_damaged.len() - 1];
                let size = possibility.len();
                let is_known = possibility.chars().position(|c| c == '?').is_none();
                let expected_size = contiguous_group_counts[contiguous_group_counts.len() - 1];
                let pieces_align = size == contiguous_group_counts.len();

                if size < expected_size {
                    // Remove pieces that won't work at all from back
                    possibly_damaged.pop_back();
                } else if (is_known || pieces_align) && size == expected_size {
                    // Remove pieces that are definitely known (no variance) from back
                    possibly_damaged.pop_back();
                    contiguous_group_counts.pop_back();
                } else {
                    break;
                }
            }

            let possible_size = possibly_damaged.len();
            let group_size = contiguous_group_counts.len();
            let search_size = possibly_damaged.iter().fold(0, |acc, s| acc + s.len())
                + (possible_size - 1)
                - (group_size - 1);

            if search_size == contiguous_group_counts.iter().sum::<usize>() {
                num_arrangements += 1;
                continue;
            }

            if possible_size == group_size {
                for j in 0..possible_size {
                    let possible_group = possibly_damaged[j];
                    let possible_group_size = possible_group.len();
                    let group_size = contiguous_group_counts[j];
                    let known_count = possible_group.chars().filter(|c| *c == '#').count();

                    // There is only one possibility
                    if known_count == group_size || possible_group_size == group_size {
                        num_arrangements += 1;
                        continue;
                    }

                    if let Some(index) = possible_group.find('#') {
                        // Anchored to the beginning or the end
                        if index == 0 || index == possible_group_size - 1 {
                            num_arrangements += 1;
                            continue;
                        }

                        if let Some(last_index) = possible_group.rfind('#') {
                            if last_index == possible_group_size - 1 {
                                num_arrangements += 1;
                                continue;
                            }
                        }

                        println!("i: {i}");
                        println!("Found index: {index}");
                        println!("possibly_damaged: {:?}", possibly_damaged);
                        println!("contiguous_groups_counts: {:?}", contiguous_group_counts);
                    } else {
                        num_arrangements += possible_group.len() - group_size + 1;
                    }
                }

                continue;
            }

            for j in 0..possible_size {}

            println!("i: {i}");
            println!("possibly_damaged: {:?}", possibly_damaged);
            println!("contiguous_groups_counts: {:?}", contiguous_group_counts);
        }
    }

    num_arrangements
}

fn part2(input: &String) -> usize {
    0
}

pub fn answer() {
    let input = get_input();

    println!("Day {DAY} part 1: {}", part1(&input));
    println!("Day {DAY} part 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use std::io;

    use super::*;

    static EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn validate_part1_example() -> io::Result<()> {
        let input1 = EXAMPLE.to_string();

        assert_eq!(part1(&input1), 21);

        Ok(())
    }

    #[test]
    fn validate_part1() -> io::Result<()> {
        let input1 = get_input();

        assert_eq!(part1(&input1), 9724940);

        Ok(())
    }

    #[test]
    fn validate_part2() -> io::Result<()> {
        let input1 = get_input();

        assert_eq!(part2(&input1), 569052586852);

        Ok(())
    }
}
