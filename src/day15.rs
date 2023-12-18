use std::{collections::HashMap, fs, str::Chars};

const DAY: &str = "15";

fn get_input() -> String {
    fs::read_to_string(format!("./data/day{DAY}.txt")).expect("read the file")
}

fn hash(c: Chars) -> u32 {
    c.fold(0, |total, c| ((total + (c as u32)) * 17) % 256)
}

fn part1(input: &String) -> u32 {
    input
        .trim_matches('\n')
        .split(',')
        .map(|step| hash(step.chars()))
        .sum::<u32>()
}

#[derive(Clone, Copy, Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u32,
}

fn part2(input: &String) -> u32 {
    let mut boxes: HashMap<u32, Vec<Lens>> = HashMap::new();

    input.trim_matches('\n').split(',').for_each(|step| {
        if let Some((label, focal_length)) = step.split_once(['=', '-']) {
            let box_number = hash(label.chars());

            if focal_length.is_empty() {
                if let Some(lenses) = boxes.get(&box_number) {
                    boxes.insert(
                        box_number,
                        lenses
                            .into_iter()
                            .filter(|lens| lens.label != label)
                            .map(|lens| *lens)
                            .collect(),
                    );
                }
            } else {
                let lenses = boxes.entry(box_number).or_insert(vec![]);

                if lenses.is_empty() {
                    lenses.push(Lens {
                        label,
                        focal_length: focal_length.parse::<u32>().unwrap(),
                    });
                } else {
                    if let Some((i, _)) = lenses
                        .iter()
                        .enumerate()
                        .find(|(_, lens)| lens.label == label)
                    {
                        lenses[i] = Lens {
                            label,
                            focal_length: focal_length.parse::<u32>().unwrap(),
                        };
                    } else {
                        lenses.push(Lens {
                            label,
                            focal_length: focal_length.parse::<u32>().unwrap(),
                        });
                    }
                }
            }
        }
    });

    boxes
        .into_iter()
        .map(|(box_number, lenses)| {
            lenses.into_iter().enumerate().fold(0, |power, (i, lens)| {
                power + (box_number + 1) * (i as u32 + 1) * lens.focal_length
            })
        })
        .sum::<u32>()
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

    static EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn validate_part1_example() -> io::Result<()> {
        let input1 = EXAMPLE.to_string();

        assert_eq!(part1(&input1), 1320);

        Ok(())
    }

    #[test]
    fn validate_part1() -> io::Result<()> {
        let input1 = get_input();

        assert_eq!(part1(&input1), 505427);

        Ok(())
    }

    #[test]
    fn validate_part2_example() -> io::Result<()> {
        let input1 = EXAMPLE.to_string();

        assert_eq!(part2(&input1), 145);

        Ok(())
    }

    #[test]
    fn validate_part2() -> io::Result<()> {
        let input1 = get_input();

        assert_eq!(part2(&input1), 243747);

        Ok(())
    }
}
