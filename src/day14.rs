use std::{collections::HashMap, fs};

const DAY: &str = "14";

fn get_input() -> String {
    fs::read_to_string(format!("./data/day{DAY}.txt")).expect("read the file")
}

fn tilt_north(rows: &mut Vec<Vec<char>>) {
    for i in 1..rows.len() {
        let row = &rows[i];
        let clone = row.clone();

        for index in 0..row.len() {
            let maybe_rock = clone[index];

            if maybe_rock == 'O' {
                for j in (0..i).rev() {
                    if rows[j][index] == '.' {
                        rows[j][index] = 'O';
                        rows[j + 1][index] = '.';
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_west(rows: &mut Vec<Vec<char>>) {
    for i in 0..rows.len() {
        let row: &Vec<char> = &rows[i];
        let clone = row.clone();

        for index in 1..row.len() {
            let maybe_rock = clone[index];

            if maybe_rock == 'O' {
                for j in (0..index).rev() {
                    if rows[i][j] == '.' {
                        rows[i][j] = 'O';
                        rows[i][j + 1] = '.';
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_south(rows: &mut Vec<Vec<char>>) {
    for i in (0..(rows.len() - 1)).rev() {
        let row = &rows[i];
        let clone = row.clone();

        for index in 0..row.len() {
            let maybe_rock = clone[index];

            if maybe_rock == 'O' {
                for j in (i + 1)..rows.len() {
                    if rows[j][index] == '.' {
                        rows[j][index] = 'O';
                        rows[j - 1][index] = '.';
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_east(rows: &mut Vec<Vec<char>>) {
    for i in 0..rows.len() {
        let row: &Vec<char> = &rows[i];
        let clone = row.clone();

        for index in (0..(row.len() - 1)).rev() {
            let maybe_rock = clone[index];
            if maybe_rock == 'O' {
                for j in (index + 1)..clone.len() {
                    if rows[i][j] == '.' {
                        rows[i][j] = 'O';
                        rows[i][j - 1] = '.';
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn part1(input: &String) -> usize {
    let mut rows: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let row_count = rows.len();

    tilt_north(&mut rows);

    rows.into_iter().enumerate().fold(0, |acc, (i, row)| {
        let multiplier = row_count - i;

        acc + (multiplier * row.into_iter().filter(|c| *c == 'O').count())
    })
}

fn part2(input: &String) -> usize {
    let mut rows: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let row_count = rows.len();
    let mut prev_loads: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..1_500 {
        tilt_north(&mut rows);
        tilt_west(&mut rows);
        tilt_south(&mut rows);
        tilt_east(&mut rows);

        let load = rows.iter().enumerate().fold(0, |acc, (i, row)| {
            let multiplier = row_count - i;

            acc + (multiplier * row.into_iter().filter(|c| **c == 'O').count())
        });

        prev_loads.entry(load).or_insert_with(|| vec![]).push(i + 1);
    }

    prev_loads
        .into_iter()
        .filter(|(_, iterations)| iterations.len() > 1)
        .find(|(_, iterations)| {
            let mut diffs = vec![];

            for i in 1..iterations.len() {
                diffs.push(iterations[i] - iterations[i - 1]);
            }

            let mut cur = iterations[0];

            for diff in diffs.iter().cycle() {
                if cur == 1_000_000_000 {
                    return true;
                }

                if cur > 1_000_000_000 {
                    return false;
                }

                cur += diff;
            }

            false
        })
        .unwrap()
        .0
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

    static EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn validate_part1_example() -> io::Result<()> {
        let input1 = EXAMPLE.to_string();

        assert_eq!(part1(&input1), 136);

        Ok(())
    }

    #[test]
    fn validate_part1() -> io::Result<()> {
        let input1 = get_input();

        assert_eq!(part1(&input1), 110407);

        Ok(())
    }

    #[test]
    fn validate_part2_example() -> io::Result<()> {
        let input1 = EXAMPLE.to_string();

        assert_eq!(part2(&input1), 64);

        Ok(())
    }

    #[test]
    fn validate_part2() -> io::Result<()> {
        let input1 = get_input();

        assert_eq!(part2(&input1), 87273);

        Ok(())
    }
}
