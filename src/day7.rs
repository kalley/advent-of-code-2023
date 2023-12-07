use std::{cmp::Ordering, collections::HashMap, fs, str::FromStr, string::ParseError};

const DAY: &str = "7";

static mut J_VALUE: Option<u8> = None;

fn get_input() -> String {
    fs::read_to_string(format!("./data/day{DAY}.txt")).expect("read the file")
}

#[derive(Debug, Eq, Ord)]
struct Card(char);

impl Card {
    fn new(face: char) -> Self {
        Self(face)
    }

    fn value(&self) -> u8 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => unsafe { J_VALUE.unwrap() },
            'T' => 10,
            n => n.to_string().parse::<u8>().unwrap(),
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, Ord)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
    kind: HandKind,
}

impl FromStr for Hand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_string, bid) = s.trim().split_once(' ').unwrap();
        let cards: Vec<Card> = cards_string.chars().map(Card::new).collect();
        let mut card_counts: HashMap<char, u8> = HashMap::new();

        for card in cards.iter() {
            let count = card_counts.entry(card.0).or_default().clone();

            card_counts.insert(card.0, count + 1);
        }

        let values = card_counts.values();
        let num_uniqs = values.len();

        let j_is_joker = unsafe { J_VALUE.unwrap() == 1 };

        let kind = if num_uniqs == 1 {
            HandKind::FiveOfAKind
        } else if num_uniqs == 2 {
            // 1) xxxxy
            // 2) xxxyy
            if *values.max().unwrap() == 4 {
                // 1)
                if j_is_joker {
                    if let Some(_) = card_counts.get(&'J') {
                        HandKind::FiveOfAKind
                    } else {
                        HandKind::FourOfAKind
                    }
                } else {
                    HandKind::FourOfAKind
                }
            } else {
                // 2)
                if j_is_joker {
                    if let Some(_) = card_counts.get(&'J') {
                        HandKind::FiveOfAKind
                    } else {
                        HandKind::FullHouse
                    }
                } else {
                    HandKind::FullHouse
                }
            }
        } else if num_uniqs == 3 {
            // 1) xxxyz
            // 2) xxyyz
            if *values.max().unwrap() == 3 {
                // 1)
                if j_is_joker {
                    if let Some(_) = card_counts.get(&'J') {
                        HandKind::FourOfAKind
                    } else {
                        HandKind::ThreeOfAKind
                    }
                } else {
                    HandKind::ThreeOfAKind
                }
            } else {
                // 2)
                if j_is_joker {
                    if let Some(j_count) = card_counts.get(&'J') {
                        if *j_count == 2 {
                            HandKind::FourOfAKind
                        } else {
                            HandKind::FullHouse
                        }
                    } else {
                        HandKind::TwoPair
                    }
                } else {
                    HandKind::TwoPair
                }
            }
        } else if num_uniqs == 4 {
            // 1) xxyza
            if j_is_joker {
                if let Some(_) = card_counts.get(&'J') {
                    HandKind::ThreeOfAKind
                } else {
                    HandKind::OnePair
                }
            } else {
                HandKind::OnePair
            }
        } else {
            if j_is_joker {
                if let Some(_) = card_counts.get(&'J') {
                    HandKind::OnePair
                } else {
                    HandKind::HighCard
                }
            } else {
                HandKind::HighCard
            }
        };

        Ok(Self {
            cards,
            bid: bid.parse::<u64>().unwrap(),
            kind,
        })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards
            .iter()
            .enumerate()
            .all(|(i, card)| *card == other.cards[i])
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.kind == other.kind {
            for i in 0..self.cards.len() {
                match self.cards[i].partial_cmp(&other.cards[i]) {
                    Some(Ordering::Equal) => continue,
                    Some(order) => return Some(order),
                    None => return None,
                }
            }
        } else {
            return self.kind.partial_cmp(&other.kind);
        }

        Some(Ordering::Equal)
    }
}

fn part1(input: &String) -> u64 {
    let mut hands: Vec<Hand> = vec![];

    unsafe {
        J_VALUE = Some(11);
    }

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        hands.push(line.parse::<Hand>().unwrap());
    }

    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1) * hand.bid)
        .sum::<u64>()
}

fn part2(input: &String) -> u64 {
    let mut hands: Vec<Hand> = vec![];

    unsafe {
        J_VALUE = Some(1);
    }

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        hands.push(line.parse::<Hand>().unwrap());
    }

    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1) * hand.bid)
        .sum::<u64>()
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

    static INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn validate_part1() -> io::Result<()> {
        let input = get_input();

        assert_eq!(part1(&input), 250254244);

        Ok(())
    }

    #[test]
    fn validate_part2() -> io::Result<()> {
        let input = get_input();

        assert_eq!(part2(&input), 250087440);

        Ok(())
    }
}
