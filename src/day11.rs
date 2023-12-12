use std::fs;

const DAY: &str = "11";

fn get_input() -> String {
    fs::read_to_string(format!("./data/day{DAY}.txt")).expect("read the file")
}

#[derive(Debug)]
struct Image {
    empty_cols: Vec<usize>,
    empty_rows: Vec<usize>,
    map: Vec<Vec<char>>,
    space: usize,
}

impl Image {
    fn new(input: String, space: usize) -> Self {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut empty_cols = vec![];
        let mut empty_rows = vec![];

        // Expand rows
        for i in 0..map.len() {
            let row = &map[i];

            if row.iter().all(|c| *c == '.') {
                empty_rows.push(i);
            }
        }

        for i in 0..map[0].len() {
            if map.iter().map(|row| row[i]).all(|c| c == '.') {
                empty_cols.push(i);
            }
        }

        Self {
            empty_cols,
            empty_rows,
            map,
            space,
        }
    }

    fn get_galaxy_positions(&self) -> Vec<(usize, usize)> {
        self.map
            .iter()
            .enumerate()
            .fold(vec![], |mut vertices, (i, row)| {
                let empties = self.empty_rows.iter().filter(|j| **j < i).count();
                let row_index = i + ((self.space * empties) - empties);

                row.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == '#')
                    .for_each(|(j, _)| {
                        let empties = self.empty_cols.iter().filter(|k| **k < j).count();
                        let col_index = j + ((self.space * empties) - empties);

                        vertices.push((row_index, col_index))
                    });

                vertices
            })
    }
}

fn part1(input: &String) -> usize {
    let image = Image::new(input.clone(), 2);

    let mut galaxy_positions = image.get_galaxy_positions().clone();

    let mut result = 0;

    loop {
        if let Some(pos) = galaxy_positions.pop() {
            for i in 0..galaxy_positions.len() {
                let comp = galaxy_positions[i];

                result += (pos.0.max(comp.0) - pos.0.min(comp.0))
                    + (pos.1.max(comp.1) - pos.1.min(comp.1));
            }
        } else {
            break;
        }
    }

    result
}

fn part2(input: &String) -> usize {
    let image = Image::new(input.clone(), 1000000);

    let mut galaxy_positions = image.get_galaxy_positions().clone();

    let mut result = 0;

    loop {
        if let Some(pos) = galaxy_positions.pop() {
            for i in 0..galaxy_positions.len() {
                let comp = galaxy_positions[i];

                result += (pos.0.max(comp.0) - pos.0.min(comp.0))
                    + (pos.1.max(comp.1) - pos.1.min(comp.1));
            }
        } else {
            break;
        }
    }

    result
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

    static EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn validate_part1_example() -> io::Result<()> {
        let input1 = EXAMPLE.to_string();

        assert_eq!(part1(&input1), 374);

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
