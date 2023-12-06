use std::fs;

fn get_input() -> String {
    fs::read_to_string("./data/day6.txt").expect("read the file")
}

fn map_times_and_distances<T>(input: &String, parse: fn(&str) -> T) -> (Option<T>, Option<T>) {
    let mut lines = input.lines();

    let times = if let Some((label, times)) = lines.next().unwrap().split_once(':') {
        if label != "Time" {
            None
        } else {
            Some(parse(times))
        }
    } else {
        None
    };

    let distances = if let Some((label, distances)) = lines.next().unwrap().split_once(':') {
        if label != "Distance" {
            None
        } else {
            Some(parse(distances))
        }
    } else {
        None
    };

    (times, distances)
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
    let (times, distances) = map_times_and_distances(input, |values| {
        values
            .trim()
            .split(' ')
            .filter(|v| *v != "")
            .map(|v: &str| v.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    });

    for (t, d) in times.unwrap().iter().zip(distances.unwrap().iter()) {
        margin.push(find_ways_to_win(*t, *d));
    }

    margin
        .into_iter()
        .fold(0, |acc, val| if acc == 0 { val } else { acc * val })
}

fn part2(input: &String) -> u64 {
    let (time, distance) = map_times_and_distances(input, |value| {
        value.replace(" ", "").parse::<u64>().unwrap()
    });

    find_ways_to_win(time.unwrap(), distance.unwrap())
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
