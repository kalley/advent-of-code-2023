use std::fs;

const DAY: &str = "9";

fn get_input() -> String {
    fs::read_to_string(format!("./data/day{DAY}.txt")).expect("read the file")
}

fn get_differences(history: &Vec<i64>) -> (Vec<i64>, bool) {
    let mut next: Vec<i64> = vec![];
    let mut all_zeros = true;

    for i in 1..history.len() {
        let diff = history[i] - history[i - 1];

        if diff != 0 {
            all_zeros = false;
        }

        next.push(diff);
    }

    (next, all_zeros)
}

fn predict_future(line: &str) -> i64 {
    let initial: Vec<i64> = line.split(' ').map(|v| v.parse::<i64>().unwrap()).collect();

    let mut extrapolated = vec![initial.clone()];

    loop {
        let i = extrapolated.len() - 1;
        let values = &mut extrapolated[i];
        let (next, all_zeros) = get_differences(&values);

        if all_zeros {
            let mut inc = values.pop().unwrap();

            for j in (0..i).rev() {
                inc = &extrapolated[j].pop().unwrap() + inc;
            }

            return inc;
        }

        extrapolated.push(next);
    }
}

fn part1(input: &String) -> i64 {
    let lines = input.lines();
    let mut result: i64 = 0;

    for line in lines {
        result += predict_future(line);
    }

    result
}

fn extrapolate_history(line: &str) -> i64 {
    let initial: Vec<i64> = line.split(' ').map(|v| v.parse::<i64>().unwrap()).collect();

    let mut extrapolated = vec![initial.clone()];

    loop {
        let i = extrapolated.len() - 1;
        let values = &mut extrapolated[i];
        let (next, all_zeros) = get_differences(&values);

        if all_zeros {
            let mut inc = values[0];

            for j in (0..i).rev() {
                inc = &extrapolated[j][0] - inc;
            }

            return inc;
        }

        extrapolated.push(next);
    }
}

fn part2(input: &String) -> i64 {
    let lines = input.lines();
    let mut result: i64 = 0;

    for line in lines {
        result += extrapolate_history(line);
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

    #[test]
    fn validate_part1() -> io::Result<()> {
        let input1 = get_input();

        assert_eq!(part1(&input1), 1974232246);

        Ok(())
    }

    #[test]
    fn validate_part2() -> io::Result<()> {
        let input1 = get_input();

        assert_eq!(part2(&input1), 928);

        Ok(())
    }
}
