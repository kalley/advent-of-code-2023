use std::{collections::HashMap, fs};

fn get_input() -> String {
    fs::read_to_string("./data/day6.txt").expect("read the file")
}

fn map_time_to_distance(input: &String) -> HashMap<u64, u64> {
    let mut lines = input.lines();

    let times = if let Some((label, times)) = lines.next().unwrap().split_once(':') {
        if label != "Time" {
            vec![]
        } else {
            times
                .trim()
                .split(' ')
                .filter(|v| *v != "")
                .map(|time| time.trim().parse::<u64>().unwrap())
                .collect()
        }
    } else {
        vec![]
    };

    let distances = if let Some((label, distances)) = lines.next().unwrap().split_once(':') {
        if label != "Distance" {
            vec![]
        } else {
            distances
                .trim()
                .split(' ')
                .filter(|v| *v != "")
                .map(|distance| distance.trim().parse::<u64>().unwrap())
                .collect()
        }
    } else {
        vec![]
    };

    times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| (*t, *d))
        .collect()
}

fn find_ways_to_win(time: u64, distance: u64) -> u64 {
    let mut ways_to_win = 0;

    for ms in 1..time {
        let traveled = ms * (time - ms);

        if traveled > distance {
            ways_to_win += 1;
        }
    }

    ways_to_win
}

fn part1(input: &String) -> u64 {
    let mut margin = vec![];

    for (t, d) in map_time_to_distance(input) {
        margin.push(find_ways_to_win(t, d));
    }

    margin
        .into_iter()
        .fold(0, |acc, val| if acc == 0 { val } else { acc * val })
}

fn part2(input: &String) -> u64 {
    let mut lines = input.lines();

    let time = if let Some((label, time)) = lines.next().unwrap().split_once(':') {
        if label != "Time" {
            return 0;
        }

        time.replace(" ", "").parse::<u64>().unwrap()
    } else {
        return 0;
    };

    let distance = if let Some((label, distance)) = lines.next().unwrap().split_once(':') {
        if label != "Distance" {
            return 0;
        }

        distance.replace(" ", "").parse::<u64>().unwrap()
    } else {
        return 0;
    };

    find_ways_to_win(time, distance)
}

pub fn answer() {
    let input = get_input();

    println!("Day 6 part 1: {}", part1(&input));
    println!("Day 6 part 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use std::io;

    use super::*;

    #[test]
    fn validate_part1() -> io::Result<()> {
        let input = get_input();

        assert_eq!(part1(&input), 633080);

        Ok(())
    }

    #[test]
    fn validate_part2() -> io::Result<()> {
        let input = get_input();

        assert_eq!(part2(&input), 20048741);

        Ok(())
    }
}
