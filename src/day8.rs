use std::{
    collections::HashMap,
    fs,
    str::{Chars, Lines},
};

const DAY: &str = "8";

fn get_input() -> String {
    fs::read_to_string(format!("./data/day{DAY}.txt")).expect("read the file")
}

#[derive(Debug)]
struct Node {
    id: String,
    paths: (String, String),
}

impl Node {
    fn lookup(&self, direction: char) -> String {
        match direction {
            'R' => self.paths.1.to_owned(),
            'L' => self.paths.0.to_owned(),
            _ => unreachable!(),
        }
    }
}

fn network_from_lines(mut lines: Lines) -> HashMap<String, Node> {
    let mut map = HashMap::new();

    loop {
        if let Some(node) = lines.next() {
            if let Some((label, paths)) = node.split_once('=') {
                if let Some((left, right)) = paths.trim().replace(['(', ')'], "").split_once(',') {
                    map.insert(
                        label.trim().to_string(),
                        Node {
                            id: label.trim().to_string(),
                            paths: (left.trim().to_string(), right.trim().to_string()),
                        },
                    );
                }
            }
        } else {
            break;
        }
    }

    map
}

#[derive(Debug)]
struct HumanNetwork(HashMap<String, Node>);

impl HumanNetwork {
    fn traverse(
        &self,
        instructions: Chars,
        start_node: Option<&Node>,
        intial_steps: Option<u64>,
    ) -> u64 {
        let mut current = start_node.unwrap_or_else(|| self.0.get("AAA").unwrap());
        let mut steps = intial_steps.unwrap_or_default();

        for c in instructions.clone() {
            let next_path = current.lookup(c);

            steps += 1;

            if next_path == "ZZZ" {
                return steps;
            }

            current = self.0.get(&next_path).unwrap();
        }

        self.traverse(instructions, Some(current), Some(steps))
    }
}

fn part1(input: &String) -> u64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars();

    let map = network_from_lines(lines);

    let network = HumanNetwork(map);

    network.traverse(instructions, None, None)
}

#[derive(Debug)]
struct GhostNetwork(HashMap<String, Node>);

impl GhostNetwork {
    fn traverse(
        &self,
        instructions: Chars,
        start_nodes: Option<Vec<&Node>>,
        mut steps: u64,
    ) -> u64 {
        let mut current = start_nodes.unwrap_or_else(|| {
            self.0
                .iter()
                .filter(|(k, _)| k.ends_with("A"))
                .map(|(_, v)| v)
                .collect()
        });

        println!(
            "steps: {}\ncurrent: {:?}\n",
            steps,
            current
                .iter()
                .map(|node| node.id.clone())
                .collect::<Vec<String>>()
        );

        for c in instructions.clone() {
            let mut all_zs = true;

            current = current
                .into_iter()
                .map(|node| {
                    let path = node.lookup(c);

                    if !path.ends_with("Z") {
                        all_zs = false;
                    }

                    self.0.get(&path).unwrap()
                })
                .collect();

            steps += 1;

            if all_zs {
                return steps;
            }
        }

        self.traverse(instructions, Some(current), steps)
    }
}

fn part2(input: &String) -> u64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars();

    let nodes = network_from_lines(lines);

    let network = GhostNetwork(nodes);

    let start_nodes = vec!["MQM", "CTV", "PDR", "SHL", "MML", "VPH"]
        .into_iter()
        .map(|id| network.0.get(id).unwrap())
        .collect();

    network.traverse(instructions, Some(start_nodes), 112147445)
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

        assert_eq!(part1(&input1), 19637);

        Ok(())
    }

    static INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn validate_part2() -> io::Result<()> {
        let input1 = INPUT.to_string();

        assert_eq!(part2(&input1), 6);

        Ok(())
    }
}
