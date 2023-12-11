use std::fs;

const DAY: &str = "10";

fn get_input() -> String {
    fs::read_to_string(format!("./data/day{DAY}.txt")).expect("read the file")
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
struct Point {
    next: Direction,
    kind: char,
    vertex: (usize, usize),
}

#[derive(Debug)]
struct Maze {
    maze: Vec<Vec<char>>,
    col_size: usize,
    row_size: usize,
    start: (usize, usize),
}

impl Maze {
    fn new(text: &String) -> Self {
        let mut maze = vec![];
        let mut row = 0;
        let mut start: (usize, usize) = (0, 0);
        let mut col_size = 0;

        for line in text.lines() {
            col_size = line.len();
            if let Some(col) = line.chars().position(|c| c == 'S') {
                start = (row, col.try_into().unwrap());
            }

            maze.push(line.chars().collect::<Vec<char>>());
            row += 1;
        }

        Self {
            maze,
            col_size,
            row_size: row,
            start,
        }
    }

    fn next_step(&self, path: &Vec<Point>) -> Option<Point> {
        let point = &path[path.len() - 1];

        // Going north
        if point.next == Direction::North {
            let vertex = (point.vertex.0 - 1, point.vertex.1);

            match self.maze[vertex.0][vertex.1] {
                '|' => {
                    return Some(Point {
                        next: Direction::North,
                        kind: '|',
                        vertex,
                    })
                }
                '7' => {
                    return Some(Point {
                        next: Direction::West,
                        kind: '7',
                        vertex,
                    })
                }
                'F' => {
                    return Some(Point {
                        next: Direction::East,
                        kind: 'F',
                        vertex,
                    })
                }
                _ => (),
            }
        }

        // Going west
        if point.next == Direction::West {
            let vertex = (point.vertex.0, point.vertex.1 - 1);

            match self.maze[vertex.0][vertex.1] {
                '-' => {
                    return Some(Point {
                        next: Direction::West,
                        kind: '-',
                        vertex,
                    })
                }
                'L' => {
                    return Some(Point {
                        next: Direction::North,
                        kind: 'L',
                        vertex,
                    })
                }
                'F' => {
                    return Some(Point {
                        next: Direction::South,
                        kind: 'F',
                        vertex,
                    })
                }
                _ => (),
            }
        }

        // Going east
        if point.next == Direction::East {
            let vertex = (point.vertex.0, point.vertex.1 + 1);

            match self.maze[vertex.0][vertex.1] {
                '-' => {
                    return Some(Point {
                        next: Direction::East,
                        kind: '-',
                        vertex,
                    })
                }
                '7' => {
                    return Some(Point {
                        next: Direction::South,
                        kind: 'L',
                        vertex,
                    })
                }
                'J' => {
                    return Some(Point {
                        next: Direction::North,
                        kind: 'F',
                        vertex,
                    })
                }
                _ => (),
            }
        }

        // Going south
        if point.next == Direction::South {
            let vertex = (point.vertex.0 + 1, point.vertex.1);

            match self.maze[vertex.0][vertex.1] {
                '|' => {
                    return Some(Point {
                        next: Direction::South,
                        kind: '|',
                        vertex,
                    })
                }
                'L' => {
                    return Some(Point {
                        next: Direction::East,
                        kind: 'L',
                        vertex,
                    })
                }
                'J' => {
                    return Some(Point {
                        next: Direction::West,
                        kind: 'J',
                        vertex,
                    })
                }
                _ => (),
            }
        }

        None
    }

    fn find_farthest(&self) -> usize {
        let mut path: Vec<Point> = vec![Point {
            next: Direction::West,
            kind: 'S',
            vertex: (self.start.0, self.start.1),
        }];
        // let mut map = vec![vec!['.'; self.col_size]; self.row_size];

        loop {
            if let Some(next) = self.next_step(&path) {
                // map[next.vertex.0][next.vertex.1] = next.kind;
                path.push(next);
            } else {
                break;
            }
        }

        // map.iter()
        //     .for_each(|p| println!("{}", p.iter().collect::<String>()));

        path.len() / 2
    }
}

fn part1(input: &String) -> usize {
    let maze = Maze::new(input);

    maze.find_farthest()
}

fn part2(input: &String) -> i64 {
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

    #[test]
    fn validate_part1() -> io::Result<()> {
        let input1 = get_input();

        assert_eq!(part1(&input1), 6882);

        Ok(())
    }

    static INPUT: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn validate_part2() -> io::Result<()> {
        let input1 = INPUT.to_string();

        assert_eq!(part2(&input1), 10);

        Ok(())
    }
}
