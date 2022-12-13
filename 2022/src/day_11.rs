use advent_of_code::read_file_to_string;
use log::debug;
use regex::Regex;
use std::collections::{BTreeMap, VecDeque};
use std::fmt;
use std::path::Path;

pub fn part_one(input_path: &Path) -> i64 {
    monkey_business(load_monkeys(input_path), false, 20)
}

pub fn part_two(input_path: &Path) -> i64 {
    monkey_business(load_monkeys(input_path), true, 10000)
}

fn monkey_business(mut monkeys: Monkeys, worried: bool, rounds: usize) -> i64 {
    let overall_worry_level: i64 = monkeys
        .values()
        .map(|m| m.test_operand as i64)
        .collect::<Vec<i64>>()
        .iter()
        .product();
    let monkey_ids = monkeys.clone();

    for round in 1..=rounds {
        for monkey_id in monkey_ids.keys() {
            debug!("Monkey {}:", monkey_id);
            monkeys
                .get_mut(&monkey_id)
                .map(|m| m.pass_items(worried, Some(overall_worry_level)))
                .unwrap()
                .iter()
                .for_each(|i| {
                    monkeys
                        .entry(i.to)
                        .and_modify(|m| m.add_item(i.item.clone()));
                });
        }

        debug!(
            "\nAfter round {}, the monkeys are holding items with these worry levels:",
            round
        );
        monkeys.iter().for_each(|(_, m)| debug!("{}", m));
        debug!("");
    }

    monkeys
        .iter()
        .for_each(|(_, m)| println!("Monkey {} inspected items {} times.", m.id, m.inspections));

    let mut inspections: Vec<i64> = monkeys
        .iter()
        .map(|(_, m)| m.inspections as i64)
        .collect::<Vec<i64>>();
    inspections.sort();
    inspections.reverse();
    let (top_two_inspections, _) = inspections.split_at(2);

    let mut monkey_business_level: i64 = 1;
    for inspection in top_two_inspections {
        monkey_business_level = monkey_business_level * inspection;
    }

    monkey_business_level
}

fn load_monkeys(input_path: &Path) -> Monkeys {
    let monkey_id_regex = Regex::new(r"(?m)^Monkey\s+(?P<id>\d+)").unwrap();
    let item_ids_regex = Regex::new(r"(?m)^\s+Starting\sitems:\s+(?P<item_ids>.+)").unwrap();
    let operation_regex =
        Regex::new(r"(?m)^\s+Operation:\s+new\s+=\s+old\s+(?P<operator>.)\s+(?P<operand>.+)")
            .unwrap();
    let test_regex = Regex::new(r"(?m)^\s+Test:\s+divisible\s+by\s+(?P<operand>.+)").unwrap();
    let true_test_regex = Regex::new(r"(?m)^\s+If\s+true:\s+throw to monkey\s+(?P<id>.+)").unwrap();
    let false_test_regex =
        Regex::new(r"(?m)^\s+If\s+false:\s+throw to monkey\s+(?P<id>.+)").unwrap();

    let mut monkeys: Monkeys = Monkeys::new();

    if let Ok(lines) = read_file_to_string(input_path) {
        for (index, _) in monkey_id_regex.captures_iter(&lines).enumerate() {
            monkeys
                .entry(index.try_into().unwrap())
                .or_insert(Monkey::new(index.try_into().unwrap()));
        }

        for (index, cap) in item_ids_regex.captures_iter(&lines).enumerate() {
            let items: VecDeque<Item> = cap
                .name("item_ids")
                .unwrap()
                .as_str()
                .split(", ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|i| Item::new(WorryLevel::new(i.parse::<i64>().unwrap())))
                .collect::<VecDeque<Item>>();
            monkeys
                .entry(index.try_into().unwrap())
                .and_modify(|m| m.items = items);
        }

        for (index, cap) in operation_regex.captures_iter(&lines).enumerate() {
            let operator = cap.name("operator").unwrap().as_str().to_string();
            let operand: String = cap.name("operand").unwrap().as_str().to_string();

            monkeys
                .entry(index.try_into().unwrap())
                .and_modify(|m| m.operation_operator = operator);
            monkeys
                .entry(index.try_into().unwrap())
                .and_modify(|m| m.operation_operand = operand);
        }

        for (index, cap) in test_regex.captures_iter(&lines).enumerate() {
            let operand: u32 = cap.name("operand").unwrap().as_str().parse().unwrap();

            monkeys
                .entry(index.try_into().unwrap())
                .and_modify(|m| m.test_operand = operand);
        }

        for (index, cap) in true_test_regex.captures_iter(&lines).enumerate() {
            let id: i32 = cap.name("id").unwrap().as_str().parse().unwrap();

            monkeys
                .entry(index.try_into().unwrap())
                .and_modify(|m| m.test_true_pass_id = id);
        }

        for (index, cap) in false_test_regex.captures_iter(&lines).enumerate() {
            let id: i32 = cap.name("id").unwrap().as_str().parse().unwrap();

            monkeys
                .entry(index.try_into().unwrap())
                .and_modify(|m| m.test_false_pass_id = id);
        }
    }

    monkeys
}

type Monkeys = BTreeMap<i32, Monkey>;

#[derive(Debug, Clone)]
struct WorryLevel {
    value: i64,
}

impl WorryLevel {
    pub fn new(value: i64) -> WorryLevel {
        WorryLevel { value }
    }

    pub fn is_divisible_by(self, operand: i64) -> bool {
        self.value % operand == 0
    }

    pub fn divide(&mut self, operand: Option<i64>) -> i64 {
        let _operand = match operand {
            Some(value) => value,
            None => self.value,
        };

        self.value = self.value / _operand;

        self.value
    }

    pub fn modulo(&mut self, operand: Option<i64>) -> i64 {
        let _operand = match operand {
            Some(value) => value,
            None => self.value,
        };

        self.value = self.value % _operand;

        self.value
    }

    pub fn add(&mut self, operand: Option<i64>) -> i64 {
        let _operand = match operand {
            Some(value) => value,
            None => self.value,
        };

        self.value = self.value + _operand;

        self.value
    }

    pub fn multiply(&mut self, operand: Option<i64>) -> i64 {
        let _operand = match operand {
            Some(value) => value,
            None => self.value,
        };

        self.value = self.value * _operand;

        self.value
    }
}

impl fmt::Display for WorryLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value,)
    }
}

#[derive(Debug, Clone)]
struct PassedItem {
    to: i32,
    item: Item,
}

impl PassedItem {
    pub fn new(to: i32, item: Item) -> PassedItem {
        PassedItem { to, item }
    }
}

#[derive(Debug, Clone)]
struct Item {
    worry_level: WorryLevel,
}

impl Item {
    pub fn new(worry_level: WorryLevel) -> Item {
        Item { worry_level }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    id: i32,
    inspections: i32,
    items: VecDeque<Item>,
    operation_operator: String,
    operation_operand: String,
    test_operand: u32,
    test_true_pass_id: i32,
    test_false_pass_id: i32,
}

impl Monkey {
    pub fn new(id: i32) -> Monkey {
        Monkey {
            id,
            inspections: 0,
            items: VecDeque::new(),
            operation_operator: "+".to_string(),
            operation_operand: "0".to_string(),
            test_operand: 1,
            test_true_pass_id: -1,
            test_false_pass_id: -1,
        }
    }

    pub fn operation(&self, item: &mut Item) -> i64 {
        let right_operand = match self.operation_operand.parse::<i64>() {
            Ok(operand) => Some(operand),
            Err(_) => None,
        };

        match self.operation_operator.as_str() {
            "+" => item.worry_level.add(right_operand),
            "*" => item.worry_level.multiply(right_operand),
            "/" => item.worry_level.divide(right_operand),
            "%" => item.worry_level.modulo(right_operand),
            &_ => item.worry_level.add(right_operand),
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push_back(item);
    }

    pub fn pass_items(
        &mut self,
        worried: bool,
        overall_worry_level: Option<i64>,
    ) -> Vec<PassedItem> {
        let mut passed_items: Vec<PassedItem> = Vec::new();

        while let Some(mut item) = self.items.pop_front() {
            self.inspections = self.inspections + 1;
            let mut new_worry_level: i64 = self.operation(&mut item);
            debug!(
                "  Monkey inspects an item with worry level of {}",
                new_worry_level,
            );
            debug!(
                "    Worry level is {} by {} to {}",
                self.operation_operator, self.operation_operand, new_worry_level,
            );

            if !worried {
                new_worry_level = item.worry_level.divide(Some(3));
                debug!(
                    "    Monkey gets bored with item. Worry level is divided by 3 to {}",
                    new_worry_level // new_worry_level
                );
            }

            match overall_worry_level {
                _ => item.worry_level.modulo(overall_worry_level),
            };

            let next_monkey_id: i32 = match item
                .clone()
                .worry_level
                .is_divisible_by(self.test_operand.into())
            {
                true => {
                    debug!(
                        "    Current worry level is divisible by {}",
                        self.test_operand
                    );
                    self.test_true_pass_id
                }
                false => {
                    debug!(
                        "    Current worry level is not divisible by {}",
                        self.test_operand
                    );
                    self.test_false_pass_id
                }
            };

            debug!(
                "    Item with worry level {} is thrown to monkey {}",
                item.worry_level, next_monkey_id
            );
            passed_items.push(PassedItem::new(next_monkey_id, item.clone()));
        }

        passed_items
    }
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Monkey {}: {}",
            self.id,
            self.items
                .iter()
                .map(|i| i.worry_level.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
