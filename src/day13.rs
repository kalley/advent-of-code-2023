use std::fs;

use regex::Regex;

const DAY: &str = "13";

fn get_input() -> String {
    fs::read_to_string(format!("./data/day{DAY}.txt")).expect("read the file")
}

fn get_patterns(input: &String) -> Vec<Vec<&str>> {
    let re = Regex::new(r"\n\n").unwrap();

    re.split(input)
        .map(|pattern| pattern.lines().collect())
        .collect()
}

fn part1(input: &String) -> usize {
    let patterns = get_patterns(input);
    let mut result = 0;

    for i in 0..patterns.len() {
        let rows = &patterns[i];
        let row_size = rows[0].len();
        let mut vertical_attempts: Vec<(usize, usize)> = vec![];

        for j in 1..row_size {
            let left = &rows[0][..j];
            let right = &rows[0][j..];
            let size = left.len().min(right.len());

            if &left[(left.len() - size)..]
                == right[..size].chars().rev().collect::<String>().as_str()
            {
                vertical_attempts.push((j, size));
            }
        }

        let mut matched_index = 0;

        if vertical_attempts.into_iter().any(|(index, size)| {
            for j in 1..rows.len() {
                let left = &rows[j][..index];
                let right = &rows[j][index..];

                if &left[(left.len() - size)..]
                    != right[..size].chars().rev().collect::<String>().as_str()
                {
                    return false;
                }
            }

            matched_index = index;
            true
        }) {
            result += matched_index;
            continue;
        }

        let mut found = false;

        'outer: for j in 1..rows.len() {
            let top = &rows[..j];
            let bottom = &rows[j..];
            let size = top.len().min(bottom.len());
            let mut k = 0;

            loop {
                if k == size {
                    break;
                }

                if &rows[j - k - 1] != &rows[j + k] {
                    continue 'outer;
                }

                k += 1;
            }

            found = true;
            result += j * 100;
            break;
        }

        if !found {
            println!("Missed {i}");
        }
    }

    result
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

    static EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn validate_part1_example() -> io::Result<()> {
        let input1 = EXAMPLE.to_string();

        assert_eq!(part1(&input1), 405);

        Ok(())
    }

    #[test]
    fn validate_part1() -> io::Result<()> {
        let input1 = get_input();

        assert_eq!(part1(&input1), 35691);

        Ok(())
    }

    #[test]
    fn validate_part2() -> io::Result<()> {
        let input1 = get_input();

        assert_eq!(part2(&input1), 569052586852);

        Ok(())
    }
}
