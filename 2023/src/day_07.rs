use advent_of_code::read_lines;
use log::{debug, log_enabled, Level};
use std::path::Path;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;

pub fn part_one(input_path: &Path) -> u32 {
    load_hands(input_path, false, true)
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx as u32 + 1))
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>()
}

pub fn part_two(input_path: &Path) -> u32 {
    load_hands(input_path, true, true)
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx as u32 + 1))
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>()
}

enum HandRank {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(Debug, Ord, Eq, PartialEq)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
    card_values: Vec<u32>,
    strength: u32,
}

impl fmt::Display for Hand {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} {:>3} {}", self.cards.clone().into_iter().collect::<String>(), self.bid, self.strength)
    }
}

impl Hand {
    pub fn new(cards: Vec<char>, bid: u32, use_jokers: bool) -> Hand {
        Hand {
            cards: cards.clone(),
            bid,
            card_values: Hand::converts_cards_to_numeric_values(cards.clone(), use_jokers),
            strength: Hand::strength_of_hand(cards.clone(), use_jokers),
        }
    }

    fn strength_of_hand(cards: Vec<char>, use_jokers: bool) -> u32 {
        let mut mapping: HashMap<char, u32> = HashMap::new();

        let mut number_of_jokers_in_hand: u32 = 0;
        for card in cards {
            *mapping.entry(card).or_insert(0) += 1;
            if use_jokers {
                if card == 'J' {
                    number_of_jokers_in_hand += 1;
                }
            }
        }

        let values: Vec<u32> = mapping.clone().into_values().collect();

        let mut hand_rank: HandRank = match mapping.len() {
            1 => HandRank::FiveOfAKind,
            2 => {
                match values.contains(&4) {
                    true => HandRank::FourOfAKind,
                    false => HandRank::FullHouse,
                }
            }
            3 => {
                match values.contains(&3) {
                    true => HandRank::ThreeOfAKind,
                    false => HandRank::TwoPair,
                }
            }
            4 => HandRank::OnePair,
            _ => HandRank::HighCard,
        };

        hand_rank = match hand_rank {
            HandRank::FiveOfAKind => HandRank::FiveOfAKind,
            HandRank::FourOfAKind => match number_of_jokers_in_hand {
                1 => HandRank::FiveOfAKind,
                4 => HandRank::FiveOfAKind,
                _ => HandRank::FourOfAKind,
            }
            HandRank::FullHouse => match number_of_jokers_in_hand {
                2 => HandRank::FiveOfAKind,
                3 => HandRank::FiveOfAKind,
                _ => HandRank::FullHouse,
            }
            HandRank::ThreeOfAKind => match number_of_jokers_in_hand {
                1 => HandRank::FourOfAKind,
                2 => HandRank::FiveOfAKind,
                3 => HandRank::FourOfAKind,
                _ => HandRank::ThreeOfAKind,
            }
            HandRank::TwoPair => match number_of_jokers_in_hand {
                1 => HandRank::FullHouse,
                2 => HandRank::FourOfAKind,
                _ => HandRank::TwoPair,
            }
            HandRank::OnePair => match number_of_jokers_in_hand {
                1 => HandRank::ThreeOfAKind,
                2 => HandRank::ThreeOfAKind,
                3 => HandRank::FiveOfAKind,
                _ => HandRank::OnePair,
            }
            HandRank::HighCard => match number_of_jokers_in_hand {
                1 => HandRank::OnePair,
                _ => HandRank::HighCard
            }
            _ => HandRank::HighCard,
        };

        hand_rank as u32
    }

    fn converts_cards_to_numeric_values(cards: Vec<char>, use_jokers: bool) -> Vec<u32> {
        let mut card_values: Vec<u32> = vec![];
        for card in cards {
            let card_value = match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => match use_jokers {
                    true => 0,
                    false => 11,
                },
                'T' => 10,
                _ => card.to_digit(10).unwrap(),
            };
            card_values.push(card_value);
        }

        card_values
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.strength == other.strength {
            for (i, value) in self.card_values.iter().enumerate() {
                let other_value: u32 = *other.card_values.get(i).unwrap();
                if *value == other_value {
                    continue;
                }

                return Some(value.cmp(&other_value))
            }
        }

        Some(self.strength.cmp(&other.strength))
    }
}

fn load_hands(input_path: &Path, use_jokers: bool, sort_hands: bool) -> Vec<Hand> {
    let mut hands: Vec<Hand> = vec![];

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let split_result: Vec<&str> = result.split_whitespace().collect::<Vec<_>>();
                let cards: Vec<char> = split_result.get(0).unwrap().chars().collect();
                let bid: u32 = split_result.get(1).unwrap().parse::<u32>().unwrap();
                hands.push(Hand::new(cards, bid, use_jokers));
            }
        }
    }

    if sort_hands {
        hands.sort();
    }

    hands
}
