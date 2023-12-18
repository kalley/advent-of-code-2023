use std::{collections::HashMap, fs};

const DAY: &str = "16";

fn get_input() -> String {
    fs::read_to_string(format!("./data/day{DAY}.txt")).expect("read the file")
}

fn to_grid(input: &String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Beam {
    direction: Direction,
    point: (usize, usize),
}

fn get_next_directions(tile: char, from: Direction) -> Vec<Direction> {
    match from {
        Direction::North => match tile {
            '/' => vec![Direction::East],
            '\\' => vec![Direction::West],
            '.' | '|' => vec![Direction::North],
            '-' => vec![Direction::East, Direction::West],
            _ => unreachable!(),
        },
        Direction::East => match tile {
            '/' => vec![Direction::North],
            '\\' => vec![Direction::South],
            '.' | '-' => vec![Direction::East],
            '|' => vec![Direction::North, Direction::South],
            _ => unreachable!(),
        },
        Direction::South => match tile {
            '/' => vec![Direction::West],
            '\\' => vec![Direction::East],
            '.' | '|' => vec![Direction::South],
            '-' => vec![Direction::West, Direction::East],
            _ => unreachable!(),
        },
        Direction::West => match tile {
            '/' => vec![Direction::South],
            '\\' => vec![Direction::North],
            '.' | '-' => vec![Direction::West],
            '|' => vec![Direction::South, Direction::North],
            _ => unreachable!(),
        },
    }
}

fn get_next_steps(
    beam: Beam,
    grid: &Vec<Vec<char>>,
    energized: &mut Vec<Vec<char>>,
    visited: &mut HashMap<Beam, usize>,
) -> Option<((usize, usize), Vec<Direction>)> {
    if let Some(count) = visited.get(&beam) {
        if *count > 1 {
            return None;
        }
    }

    energized[beam.point.0][beam.point.1] = '#';
    *visited.entry(beam.clone()).or_default() += 1;

    Some(match beam.direction {
        Direction::North => {
            if beam.point.0 > 0 {
                let tile = grid[beam.point.0 - 1][beam.point.1];
                let next_point = (beam.point.0 - 1, beam.point.1);

                (next_point, get_next_directions(tile, Direction::North))
            } else {
                return None;
            }
        }
        Direction::East => {
            if let Some(tile) = grid[beam.point.0].get(beam.point.1 + 1) {
                let next_point = (beam.point.0, beam.point.1 + 1);

                (next_point, get_next_directions(*tile, Direction::East))
            } else {
                return None;
            }
        }
        Direction::South => {
            if let Some(row) = grid.get(beam.point.0 + 1) {
                let tile = row[beam.point.1];
                let next_point = (beam.point.0 + 1, beam.point.1);

                (next_point, get_next_directions(tile, Direction::South))
            } else {
                return None;
            }
        }
        Direction::West => {
            if beam.point.1 > 0 {
                let tile = grid[beam.point.0][beam.point.1 - 1];
                let next_point = (beam.point.0, beam.point.1 - 1);

                (next_point, get_next_directions(tile, Direction::West))
            } else {
                return None;
            }
        }
    })
}

fn calculate_energized(
    next_point: (usize, usize),
    directions: Vec<Direction>,
    grid: &Vec<Vec<char>>,
    energized: &mut Vec<Vec<char>>,
    visited: &mut HashMap<Beam, usize>,
) -> usize {
    let mut next_steps = vec![(next_point, directions)];

    loop {
        next_steps = next_steps
            .iter()
            .flat_map(|(next_point, directions)| {
                directions
                    .into_iter()
                    .map(|direction| {
                        get_next_steps(
                            Beam {
                                direction: *direction,
                                point: *next_point,
                            },
                            grid,
                            energized,
                            visited,
                        )
                    })
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .collect::<Vec<_>>()
            })
            .collect();

        if next_steps.is_empty() {
            break;
        }
    }

    energized.into_iter().fold(0, |total, row| {
        total + row.into_iter().filter(|c| **c == '#').count()
    })
}

fn part1(input: &String) -> usize {
    let grid = to_grid(input);
    let mut energized = vec![vec!['.'; grid[0].len()]; grid.len()];
    let mut visited = HashMap::new();
    let next_point = (0, 0);
    let directions = match grid[0][0] {
        '/' => vec![Direction::North],
        '\\' => vec![Direction::South],
        '.' | '-' => vec![Direction::East],
        '|' => vec![Direction::North, Direction::South],
        _ => unreachable!(),
    };

    let result = calculate_energized(next_point, directions, &grid, &mut energized, &mut visited);

    // energized
    //     .iter()
    //     .for_each(|row| println!("{}", row.iter().collect::<String>()));

    result
}

fn part2(input: &String) -> usize {
    let grid = to_grid(input);
    let row_count = grid.len();
    let row_size = grid[0].len();
    let mut max_energized = 0;

    // East/West
    for i in 0..row_count {
        [0, row_size - 1].iter().for_each(|col| {
            let mut energized = vec![vec!['.'; grid[0].len()]; grid.len()];
            let mut visited = HashMap::new();
            let next_point = (i, *col);
            let direction = if *col == 0 {
                Direction::East
            } else {
                Direction::West
            };
            let directions = get_next_directions(grid[next_point.0][next_point.1], direction);
            let result =
                calculate_energized(next_point, directions, &grid, &mut energized, &mut visited);

            if result > max_energized {
                max_energized = result;
            }
        })
    }

    // North/South
    for i in 0..row_size {
        [0, row_count - 1].iter().for_each(|row| {
            let mut energized = vec![vec!['.'; grid[0].len()]; grid.len()];
            let mut visited = HashMap::new();
            let next_point = (*row, i);
            let direction: Direction = if *row == 0 {
                Direction::South
            } else {
                Direction::North
            };
            let directions = get_next_directions(grid[next_point.0][next_point.1], direction);
            let result =
                calculate_energized(next_point, directions, &grid, &mut energized, &mut visited);

            if result > max_energized {
                max_energized = result;
            }
        })
    }

    max_energized
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

    static EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn validate_part1_example() -> io::Result<()> {
        let input1 = EXAMPLE.to_string();

        assert_eq!(part1(&input1), 46);

        Ok(())
    }

    #[test]
    fn validate_part1() -> io::Result<()> {
        let input1 = get_input();

        assert_eq!(part1(&input1), 6622);

        Ok(())
    }

    #[test]
    fn validate_part2_example() -> io::Result<()> {
        let input1 = EXAMPLE.to_string();

        assert_eq!(part2(&input1), 51);

        Ok(())
    }

    #[test]
    fn validate_part2() -> io::Result<()> {
        let input1 = get_input();

        assert_eq!(part2(&input1), 7130);

        Ok(())
    }
}
