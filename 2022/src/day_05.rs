use advent_of_code::read_lines;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::path::Path;

pub fn part_one(input_path: &Path) -> String {
    let (stacks, manifests) = stacks_and_manifests_from_input(input_path);
    let mut crate_mover: CrateMover9000 = CrateMover9000::new(stacks, manifests);
    crate_mover.execute_manifests();

    crate_mover.top_of_stacks()
}

pub fn part_two(input_path: &Path) -> String {
    let (stacks, manifests) = stacks_and_manifests_from_input(input_path);
    let mut crate_mover: CrateMover9001 = CrateMover9001::new(stacks, manifests);
    crate_mover.execute_manifests();

    crate_mover.top_of_stacks()
}

struct CrateMover9000 {
    crates: BTreeMap<i32, VecDeque<String>>,
    manifests: Vec<CrateManifest>,
}

impl CrateMover9000 {
    fn new(crates: BTreeMap<i32, VecDeque<String>>, manifests: Vec<CrateManifest>) -> CrateMover9000 {
        CrateMover9000 { crates, manifests }
    }

    fn execute_manifests(&mut self) {
        let mut _manifests = self.manifests.iter().peekable();
        while _manifests.peek().is_some() {
            if let Some(manifest) = _manifests.next() {
                let mut index = 0;
                let mut crane_arm: VecDeque<String> = VecDeque::new();

                if let Some(from_stack) = self.crates.get_mut(&manifest.from_stack) {
                    while index < manifest.number_of_crates {
                        if let Some(item) = from_stack.pop_front() {
                            crane_arm.push_front(item);
                        }
                        index = index + 1;
                    }
                }

                if let Some(to_stack) = self.crates.get_mut(&manifest.to_stack) {
                    while let Some(item) = crane_arm.pop_back() {
                        to_stack.push_front(item);
                    }
                }
            }
        }
    }

    fn top_of_stacks(&self) -> String {
        let mut result: String = "".to_string();
        for (_, value) in &self.crates {
            result.push(value.get(0).unwrap().chars().next().unwrap());
        }

        result
    }
}

struct CrateMover9001 {
    crates: BTreeMap<i32, VecDeque<String>>,
    manifests: Vec<CrateManifest>,
}

impl CrateMover9001 {
    fn new(crates: BTreeMap<i32, VecDeque<String>>, manifests: Vec<CrateManifest>) -> CrateMover9001 {
        CrateMover9001 { crates, manifests }
    }

    fn execute_manifests(&mut self) {
        let mut _manifests = self.manifests.iter().peekable();
        while _manifests.peek().is_some() {
            if let Some(manifest) = _manifests.next() {
                let mut index = 0;
                let mut crane_arm: VecDeque<String> = VecDeque::new();

                if let Some(from_stack) = self.crates.get_mut(&manifest.from_stack) {
                    while index < manifest.number_of_crates {
                        if let Some(item) = from_stack.pop_front() {
                            crane_arm.push_back(item);
                        }
                        index = index + 1;
                    }
                }

                if let Some(to_stack) = self.crates.get_mut(&manifest.to_stack) {
                    while let Some(item) = crane_arm.pop_back() {
                        to_stack.push_front(item);
                    }
                }
            }
        }
    }

    fn top_of_stacks(&self) -> String {
        let mut result: String = "".to_string();
        for (_, value) in &self.crates {
            result.push(value.get(0).unwrap().chars().next().unwrap());
        }

        result
    }
}

#[derive(Debug)]
struct CrateManifest {
    number_of_crates: i32,
    from_stack: i32,
    to_stack: i32,
}

impl CrateManifest {
    fn new(number_of_crates: i32, from_stack: i32, to_stack: i32) -> CrateManifest {
        CrateManifest { number_of_crates, from_stack, to_stack }
    }
}

fn stacks_and_manifests_from_input(input_path: &Path) -> (BTreeMap<i32, VecDeque<String>>, Vec<CrateManifest>) {
    let mut stack_map: BTreeMap<i32, VecDeque<String>> = BTreeMap::new();
    let mut manifests: Vec<CrateManifest> = Vec::new();

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                if result.is_empty() {
                    continue;
                } else if result.contains("move") {
                    let moves = result.split(' ').collect::<Vec<&str>>();

                    let manifest = CrateManifest::new(
                        moves.get(1).unwrap().parse::<i32>().unwrap(),
                        moves.get(3).unwrap().parse::<i32>().unwrap(),
                        moves.get(5).unwrap().parse::<i32>().unwrap(),
                    );

                    manifests.push(manifest);
                } else {
                    let mut i = 1;
                    let mut j = 1;

                    while i < result.trim_end().len() {
                        let index = j.try_into().unwrap();
                        stack_map.entry(index).or_insert(VecDeque::new());
                        let working_queue = stack_map.get_mut(&index).unwrap();

                        let item: String = result
                            .trim_end()
                            .to_string()
                            .get(i..=i)
                            .unwrap()
                            .to_string()
                            .trim_end()
                            .to_string();
                        if !item.is_empty() {
                            let item_char = item.chars().next().unwrap();
                            if !item_char.is_numeric() {
                                working_queue.push_back(item);
                            }
                        }
                        i = i + 4;
                        j = j + 1;
                    }
                }
            }
        }
    }

    (stack_map, manifests)
}
