use advent_of_code::read_lines;
use log::{debug, log_enabled, Level};
use std::path::Path;
use std::collections::HashMap;
use std::cmp::Ordering;

pub fn part_one(input_path: &Path) -> u32 {
    load_hands(input_path).iter().enumerate().map(|(idx, hand)| hand.bid * (idx as u32 + 1)).collect::<Vec<u32>>().iter().sum::<u32>()
}

pub fn part_two(input_path: &Path) -> i32 {
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                debug!("{}", result);
            }
        }
    }

    0
}

#[derive(Debug, Ord, Eq, PartialEq)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
    card_values: Vec<u32>,
    strength: u32,
}

impl Hand {
    pub fn new(cards: Vec<char>, bid: u32) -> Hand {
        Hand {
            cards: cards.clone(),
            bid,
            card_values: Hand::converts_cards_to_numeric_values(cards.clone()),
            strength: Hand::strength_of_hand(cards.clone()),
        }
    }

    fn strength_of_hand(cards: Vec<char>) -> u32 {
        // 6 == Five of a kind
        // 5 == Four of a kind
        // 4 == Full house
        // 3 == Three of a kind
        // 2 == Two pair
        // 1 == One pair
        // 0 == High card
        let mut mapping: HashMap<char, u32> = HashMap::new();

        for card in cards {
            *mapping.entry(card).or_insert(0) += 1;
        }
        let values: Vec<u32> = mapping.clone().into_values().collect();

        match mapping.len() {
            1 => 6,
            2 => {
                match values.contains(&4) {
                    true => 5,
                    false => 4,
                }
            }
            3 => {
                match values.contains(&3) {
                    true => 3,
                    false => 2,
                }
            }
            4 => 1,
            5 => 0,
            _ => 0,
        }
    }

    fn converts_cards_to_numeric_values(cards: Vec<char>) -> Vec<u32> {
        let mut card_values: Vec<u32> = vec![];
        for card in cards {
            let card_value = match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
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

fn load_hands(input_path: &Path) -> Vec<Hand> {
    let mut hands: Vec<Hand> = vec![];

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let split_result: Vec<&str> = result.split_whitespace().collect::<Vec<_>>();
                let cards: Vec<char> = split_result.get(0).unwrap().chars().collect();
                let bid: u32 = split_result.get(1).unwrap().parse::<u32>().unwrap();
                hands.push(Hand::new(cards, bid));
            }
        }
    }

    hands.sort();

    hands
}
