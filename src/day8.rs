use std::{
    fs,
    iter::Cycle,
    str::{Chars, Lines},
};

const DAY: &str = "8";

fn get_input() -> String {
    fs::read_to_string(format!("./data/day{DAY}.txt")).expect("read the file")
}

type NodeHandle = usize;

#[derive(Debug)]
struct Node {
    id: String,
    left: NodeHandle,
    right: NodeHandle,
}

impl Node {
    fn lookup(&self, direction: char) -> usize {
        match direction {
            'R' => self.right,
            'L' => self.left,
            _ => unreachable!(),
        }
    }
}

fn network_from_lines(mut lines: Lines) -> Vec<Node> {
    let mut nodes: Vec<Node> = vec![];

    loop {
        if let Some(node) = lines.next() {
            if let Some((label, paths)) = node.split_once('=') {
                if let Some((left, right)) = paths.trim().replace(['(', ')'], "").split_once(',') {
                    let id = label.trim().to_string();
                    let left = left.trim().to_string();
                    let right = right.trim().to_string();

                    let left_pos = if let Some(pos) = nodes.iter().position(|node| node.id == left)
                    {
                        pos
                    } else {
                        let node = Node {
                            id: left,
                            right: 0,
                            left: 0,
                        };

                        nodes.push(node);

                        nodes.len() - 1
                    };

                    let right_pos =
                        if let Some(pos) = nodes.iter().position(|node| node.id == right) {
                            pos
                        } else {
                            let node = Node {
                                id: right,
                                right: 0,
                                left: 0,
                            };

                            nodes.push(node);

                            nodes.len() - 1
                        };

                    if let Some(pos) = nodes.iter().position(|node| node.id == id) {
                        nodes[pos].right = right_pos;
                        nodes[pos].left = left_pos;
                    } else {
                        let node = Node {
                            id,
                            right: right_pos,
                            left: left_pos,
                        };

                        nodes.push(node);
                    }
                }
            }
        } else {
            break;
        }
    }

    nodes
}

#[derive(Debug)]
struct HumanNetwork(Vec<Node>);

impl HumanNetwork {
    fn traverse(
        &self,
        instructions: Cycle<Chars>,
        start_node: Option<&Node>,
        intial_steps: Option<u64>,
    ) -> u64 {
        let mut current =
            start_node.unwrap_or_else(|| self.0.iter().find(|node| node.id == "AAA").unwrap());
        let mut steps = intial_steps.unwrap_or_default();

        for c in instructions {
            current = &self.0[current.lookup(c)];

            steps += 1;

            if current.id == "ZZZ" {
                return steps;
            }
        }

        0
    }
}

fn part1(input: &String) -> u64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().cycle();

    let map = network_from_lines(lines);

    let network = HumanNetwork(map);

    network.traverse(instructions, None, None)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}

#[derive(Debug)]
struct GhostNetwork(Vec<Node>);

impl GhostNetwork {
    fn traverse<'a>(
        &'a self,
        instructions: Cycle<Chars>,
        mut current: Vec<&'a Node>,
        mut steps: u64,
    ) -> u64 {
        let mut num_steps: Vec<u64> = vec![0; current.len()];

        for c in instructions {
            let mut all_zs = true;

            current = current
                .into_iter()
                .enumerate()
                .map(|(i, node)| {
                    let next = &self.0[node.lookup(c)];

                    if !next.id.ends_with("Z") {
                        all_zs = false;
                    } else {
                        num_steps[i] = steps + 1;
                    }

                    next
                })
                .collect();

            steps += 1;

            if !num_steps.iter().any(|steps| *steps == 0) {
                break;
            }
        }

        num_steps
            .into_iter()
            .fold(0, |l, steps| if l == 0 { steps } else { lcm(steps, l) })
    }
}

fn part2(input: &String) -> u64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().cycle();
    let nodes = network_from_lines(lines);
    let network = GhostNetwork(nodes);
    let start_nodes = network
        .0
        .iter()
        .filter(|node| node.id.ends_with("A"))
        .collect();

    network.traverse(instructions, start_nodes, 0)
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

    #[test]
    fn validate_part2() -> io::Result<()> {
        let input1 = get_input();

        assert_eq!(part2(&input1), 8811050362409);

        Ok(())
    }
}
