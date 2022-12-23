use advent_of_code::{read_lines, Coordinate};
use log::debug;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    let mut patterns: JetPatterns = load_jet_patterns(input_path);
    let mut top_of_pile: u32 = 0;
    let mut chamber: HashSet<Coordinate> = HashSet::new();
    let mut rocks_at_rest: u32 = 0;
    let rock_goal: u32 = 2022;
    let mut rock_factory = RockFactory::new();

    for y_coordinate in 1..=7 {
        chamber.insert(Coordinate::new(0, y_coordinate));
    }

    while rocks_at_rest != rock_goal {
        debug!("A new rock begins falling");
        let mut test_rock = rock_factory.next(top_of_pile + 4);

        loop {
            if let Some(pattern) = next_jet_pattern(&mut patterns) {
                debug!("Jet of gas pushes rock {:#?}", pattern);
                test_rock.as_mut().push(&pattern, &chamber);
            }

            if !test_rock.has_come_to_rest(&chamber) {
                test_rock.as_mut().descend();
            } else {
                break;
            }
        }

        rocks_at_rest = rocks_at_rest + 1;
        chamber.extend(test_rock.as_ref().all_coordinates());
        top_of_pile = chamber.iter().map(|c| c.x).max().unwrap() as u32
    }

    top_of_pile as i32
}

pub fn part_two(input_path: &Path) -> i32 {
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                println!("{}", result);
            }
        }
    }

    0
}

type JetPatterns = VecDeque<JetPattern>;

#[derive(Debug, Clone)]
enum JetPattern {
    Left,
    Right,
}

fn load_jet_patterns(input_path: &Path) -> JetPatterns {
    let mut patterns: JetPatterns = JetPatterns::new();

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                for char in result.chars() {
                    match char {
                        '<' => patterns.push_back(JetPattern::Left),
                        '>' => patterns.push_back(JetPattern::Right),
                        _ => debug!("Unknown jet pattern '{}'; ignoring", char),
                    }
                }
            }
        }
    }

    patterns
}

trait Rock {
    fn descend(&mut self);
    fn has_come_to_rest(&self, chamber: &HashSet<Coordinate>) -> bool;
    fn push(&mut self, jet_pattern: &JetPattern, chamber: &HashSet<Coordinate>);
    fn all_coordinates(&self) -> HashSet<Coordinate>;
}

#[derive(Clone)]
enum RockType {
    FlatRock,
    CrossRock,
    JRock,
    SkinnyRock,
    SquareRock,
}

struct RockFactory {
    rock_order: VecDeque<Box<RockType>>,
}

impl RockFactory {
    pub fn new() -> RockFactory {
        let rock_order = VecDeque::from([
            Box::new(RockType::FlatRock),
            Box::new(RockType::CrossRock),
            Box::new(RockType::JRock),
            Box::new(RockType::SkinnyRock),
            Box::new(RockType::SquareRock),
        ]);

        RockFactory { rock_order }
    }

    pub fn next(&mut self, starting_level: u32) -> Box<dyn Rock> {
        let next_rock = self.rock_order.pop_front().unwrap();
        self.rock_order
            .push_back(Box::new(next_rock.as_ref().clone()));

        match next_rock.as_ref() {
            RockType::FlatRock => Box::new(FlatRock::new(starting_level)),
            RockType::CrossRock => Box::new(CrossRock::new(starting_level)),
            RockType::JRock => Box::new(JRock::new(starting_level)),
            RockType::SkinnyRock => Box::new(SkinnyRock::new(starting_level)),
            RockType::SquareRock => Box::new(SquareRock::new(starting_level)),
        }
    }
}

struct FlatRock {
    coordinates: HashMap<u32, Coordinate>,
}

impl FlatRock {
    pub fn new(starting_level: u32) -> FlatRock {
        let mut coordinates: HashMap<u32, Coordinate> = HashMap::with_capacity(4);
        coordinates.insert(0, Coordinate::new(starting_level as i32, 3));
        coordinates.insert(1, Coordinate::new(starting_level as i32, 4));
        coordinates.insert(2, Coordinate::new(starting_level as i32, 5));
        coordinates.insert(3, Coordinate::new(starting_level as i32, 6));

        FlatRock { coordinates }
    }
}

impl Rock for FlatRock {
    fn descend(&mut self) {
        for (_, c) in self.coordinates.iter_mut() {
            c.x = c.x - 1;
        }
    }

    fn has_come_to_rest(&self, chamber: &HashSet<Coordinate>) -> bool {
        for (_, coordinate) in &self.coordinates {
            if chamber.contains(&Coordinate::new(coordinate.x - 1, coordinate.y)) {
                return true;
            }
        }

        false
    }

    fn push(&mut self, jet_pattern: &JetPattern, chamber: &HashSet<Coordinate>) {
        let mut can_push: bool = false;
        match jet_pattern {
            JetPattern::Left => {
                let leftmost_coordinate = self.coordinates.get(&0).unwrap();
                can_push = leftmost_coordinate.y != 1
                    && chamber
                        .get(&Coordinate::new(
                            leftmost_coordinate.x,
                            leftmost_coordinate.y - 1,
                        ))
                        .is_none();

                if can_push {
                    for (_, c) in self.coordinates.iter_mut() {
                        c.y = c.y - 1;
                    }
                }
            }
            JetPattern::Right => {
                let rightmost_coordinate = self.coordinates.get(&3).unwrap();
                can_push = rightmost_coordinate.y != 7
                    && chamber
                        .get(&Coordinate::new(
                            rightmost_coordinate.x,
                            rightmost_coordinate.y + 1,
                        ))
                        .is_none();

                if can_push {
                    for (_, c) in self.coordinates.iter_mut() {
                        c.y = c.y + 1;
                    }
                }
            }
        }
    }

    fn all_coordinates(&self) -> HashSet<Coordinate> {
        let mut _all_coordinates: HashSet<Coordinate> = HashSet::with_capacity(4);
        let base_coordinate: &Coordinate = self.coordinates.get(&0).unwrap();

        for y_coordinate in base_coordinate.y..=(base_coordinate.y + 3) {
            _all_coordinates.insert(Coordinate::new(base_coordinate.x, y_coordinate));
        }

        _all_coordinates
    }
}

struct CrossRock {
    coordinates: HashMap<u32, Coordinate>,
}

impl CrossRock {
    pub fn new(starting_level: u32) -> CrossRock {
        let mut coordinates: HashMap<u32, Coordinate> = HashMap::with_capacity(3);
        // Middle left of the cross
        coordinates.insert(0, Coordinate::new((starting_level + 1) as i32, 3));
        // Center bottom of the cross
        coordinates.insert(1, Coordinate::new(starting_level as i32, 4));
        // Middle Right of the cross
        coordinates.insert(2, Coordinate::new((starting_level + 1) as i32, 5));

        CrossRock { coordinates }
    }
}

impl Rock for CrossRock {
    fn all_coordinates(&self) -> HashSet<Coordinate> {
        let mut _all_coordinates: HashSet<Coordinate> = HashSet::with_capacity(5);
        let middle_left_coordinate: &Coordinate = self.coordinates.get(&0).unwrap();
        let center_bottom_coordinate: &Coordinate = self.coordinates.get(&1).unwrap();

        for y_coordinate in middle_left_coordinate.y..=(middle_left_coordinate.y + 2) {
            _all_coordinates.insert(Coordinate::new(middle_left_coordinate.x, y_coordinate));
        }

        _all_coordinates.insert(Coordinate::new(
            center_bottom_coordinate.x,
            center_bottom_coordinate.y,
        ));
        _all_coordinates.insert(Coordinate::new(
            center_bottom_coordinate.x + 2,
            center_bottom_coordinate.y,
        ));

        _all_coordinates
    }

    fn has_come_to_rest(&self, chamber: &HashSet<Coordinate>) -> bool {
        for (_, coordinate) in &self.coordinates {
            if chamber.contains(&Coordinate::new(coordinate.x - 1, coordinate.y)) {
                return true;
            }
        }

        false
    }

    fn descend(&mut self) {
        for (_, c) in self.coordinates.iter_mut() {
            c.x = c.x - 1;
        }
    }

    fn push(&mut self, jet_pattern: &JetPattern, chamber: &HashSet<Coordinate>) {
        let mut can_push: bool = false;
        match jet_pattern {
            JetPattern::Left => {
                let leftmost_coordinate = self.coordinates.get(&0).unwrap();
                let center_coordinate = self.coordinates.get(&1).unwrap();

                can_push = leftmost_coordinate.y != 1
                    && chamber
                        .get(&Coordinate::new(
                            leftmost_coordinate.x,
                            leftmost_coordinate.y - 1,
                        ))
                        .is_none()
                    && chamber
                        .get(&Coordinate::new(
                            center_coordinate.x,
                            center_coordinate.y - 1,
                        ))
                        .is_none();
                if can_push {
                    for (_, c) in self.coordinates.iter_mut() {
                        c.y = c.y - 1;
                    }
                }
            }
            JetPattern::Right => {
                let rightmost_coordinate = self.coordinates.get(&2).unwrap();
                let center_coordinate = self.coordinates.get(&1).unwrap();

                can_push = rightmost_coordinate.y != 7
                    && chamber
                        .get(&Coordinate::new(
                            rightmost_coordinate.x,
                            rightmost_coordinate.y + 1,
                        ))
                        .is_none()
                    && chamber
                        .get(&Coordinate::new(
                            center_coordinate.x,
                            center_coordinate.y + 1,
                        ))
                        .is_none();

                if can_push {
                    for (_, c) in self.coordinates.iter_mut() {
                        c.y = c.y + 1;
                    }
                }
            }
        }
    }
}

struct JRock {
    coordinates: HashMap<u32, Coordinate>,
}

impl JRock {
    pub fn new(starting_level: u32) -> JRock {
        let mut coordinates: HashMap<u32, Coordinate> = HashMap::with_capacity(3);
        coordinates.insert(0, Coordinate::new(starting_level as i32, 3));
        coordinates.insert(1, Coordinate::new(starting_level as i32, 4));
        coordinates.insert(2, Coordinate::new(starting_level as i32, 5));

        JRock { coordinates }
    }
}

impl Rock for JRock {
    fn descend(&mut self) {
        for (_, c) in self.coordinates.iter_mut() {
            c.x = c.x - 1;
        }
    }

    fn has_come_to_rest(&self, chamber: &HashSet<Coordinate>) -> bool {
        for (_, coordinate) in &self.coordinates {
            if chamber.contains(&Coordinate::new(coordinate.x - 1, coordinate.y)) {
                return true;
            }
        }

        false
    }

    fn push(&mut self, jet_pattern: &JetPattern, chamber: &HashSet<Coordinate>) {
        let mut can_push: bool = false;
        let leftmost_coordinate = self.coordinates.get(&0).unwrap();
        let rightmost_coordinate = self.coordinates.get(&2).unwrap();

        match jet_pattern {
            JetPattern::Left => {
                can_push = leftmost_coordinate.y != 1
                    && chamber
                        .get(&Coordinate::new(
                            leftmost_coordinate.x,
                            leftmost_coordinate.y - 1,
                        ))
                        .is_none()
                    && chamber
                        .get(&Coordinate::new(
                            rightmost_coordinate.x + 1,
                            rightmost_coordinate.y - 1,
                        ))
                        .is_none()
                    && chamber
                        .get(&Coordinate::new(
                            rightmost_coordinate.x + 2,
                            rightmost_coordinate.y - 1,
                        ))
                        .is_none();

                if can_push {
                    for (_, c) in self.coordinates.iter_mut() {
                        c.y = c.y - 1;
                    }
                }
            }
            JetPattern::Right => {
                can_push = rightmost_coordinate.y != 7
                    && chamber
                        .get(&Coordinate::new(
                            rightmost_coordinate.x,
                            rightmost_coordinate.y + 1,
                        ))
                        .is_none()
                    && chamber
                        .get(&Coordinate::new(
                            rightmost_coordinate.x + 1,
                            rightmost_coordinate.y + 1,
                        ))
                        .is_none()
                    && chamber
                        .get(&Coordinate::new(
                            rightmost_coordinate.x + 2,
                            rightmost_coordinate.y + 1,
                        ))
                        .is_none();

                if can_push {
                    for (_, c) in self.coordinates.iter_mut() {
                        c.y = c.y + 1;
                    }
                }
            }
        }
    }

    fn all_coordinates(&self) -> HashSet<Coordinate> {
        let mut _all_coordinates: HashSet<Coordinate> = HashSet::with_capacity(4);
        let base_coordinate: &Coordinate = self.coordinates.get(&0).unwrap();
        let right_coordinate: &Coordinate = self.coordinates.get(&2).unwrap();

        for y_coordinate in base_coordinate.y..=(base_coordinate.y + 2) {
            _all_coordinates.insert(Coordinate::new(base_coordinate.x, y_coordinate));
        }

        _all_coordinates.insert(Coordinate::new(right_coordinate.x + 1, right_coordinate.y));
        _all_coordinates.insert(Coordinate::new(right_coordinate.x + 2, right_coordinate.y));

        _all_coordinates
    }
}

struct SkinnyRock {
    coordinates: HashMap<u32, Coordinate>,
}

impl SkinnyRock {
    pub fn new(starting_level: u32) -> SkinnyRock {
        let mut coordinates: HashMap<u32, Coordinate> = HashMap::with_capacity(1);
        coordinates.insert(0, Coordinate::new(starting_level as i32, 3));

        SkinnyRock { coordinates }
    }
}

impl Rock for SkinnyRock {
    fn descend(&mut self) {
        for (_, c) in self.coordinates.iter_mut() {
            c.x = c.x - 1;
        }
    }

    fn has_come_to_rest(&self, chamber: &HashSet<Coordinate>) -> bool {
        for (_, coordinate) in &self.coordinates {
            if chamber.contains(&Coordinate::new(coordinate.x - 1, coordinate.y)) {
                return true;
            }
        }

        false
    }

    fn push(&mut self, jet_pattern: &JetPattern, chamber: &HashSet<Coordinate>) {
        let mut can_push: bool = false;
        let bottom_coordinate = self.coordinates.get(&0).unwrap();

        match jet_pattern {
            JetPattern::Left => {
                can_push = bottom_coordinate.y != 1
                    && chamber
                        .get(&Coordinate::new(
                            bottom_coordinate.x,
                            bottom_coordinate.y - 1,
                        ))
                        .is_none()
                    && chamber
                        .get(&Coordinate::new(
                            bottom_coordinate.x + 1,
                            bottom_coordinate.y - 1,
                        ))
                        .is_none()
                    && chamber
                        .get(&Coordinate::new(
                            bottom_coordinate.x + 2,
                            bottom_coordinate.y - 1,
                        ))
                        .is_none()
                    && chamber
                        .get(&Coordinate::new(
                            bottom_coordinate.x + 3,
                            bottom_coordinate.y - 1,
                        ))
                        .is_none();

                if can_push {
                    for (_, c) in self.coordinates.iter_mut() {
                        c.y = c.y - 1;
                    }
                }
            }
            JetPattern::Right => {
                can_push = bottom_coordinate.y != 7
                    && chamber
                        .get(&Coordinate::new(
                            bottom_coordinate.x,
                            bottom_coordinate.y + 1,
                        ))
                        .is_none()
                    && chamber
                        .get(&Coordinate::new(
                            bottom_coordinate.x + 1,
                            bottom_coordinate.y + 1,
                        ))
                        .is_none()
                    && chamber
                        .get(&Coordinate::new(
                            bottom_coordinate.x + 2,
                            bottom_coordinate.y + 1,
                        ))
                        .is_none()
                    && chamber
                        .get(&Coordinate::new(
                            bottom_coordinate.x + 3,
                            bottom_coordinate.y + 1,
                        ))
                        .is_none();

                if can_push {
                    for (_, c) in self.coordinates.iter_mut() {
                        c.y = c.y + 1;
                    }
                }
            }
        }
    }

    fn all_coordinates(&self) -> HashSet<Coordinate> {
        let mut _all_coordinates: HashSet<Coordinate> = HashSet::with_capacity(4);
        let base_coordinate: &Coordinate = self.coordinates.get(&0).unwrap();

        for x_coordinate in base_coordinate.x..=(base_coordinate.x + 3) {
            _all_coordinates.insert(Coordinate::new(x_coordinate, base_coordinate.y));
        }

        _all_coordinates
    }
}

struct SquareRock {
    coordinates: HashMap<u32, Coordinate>,
}

impl SquareRock {
    pub fn new(starting_level: u32) -> SquareRock {
        let mut coordinates: HashMap<u32, Coordinate> = HashMap::with_capacity(4);
        coordinates.insert(0, Coordinate::new(starting_level as i32, 3));
        coordinates.insert(1, Coordinate::new(starting_level as i32, 4));

        SquareRock { coordinates }
    }
}

impl Rock for SquareRock {
    fn descend(&mut self) {
        for (_, c) in self.coordinates.iter_mut() {
            c.x = c.x - 1;
        }
    }

    fn has_come_to_rest(&self, chamber: &HashSet<Coordinate>) -> bool {
        for (_, coordinate) in &self.coordinates {
            if chamber.contains(&Coordinate::new(coordinate.x - 1, coordinate.y)) {
                return true;
            }
        }

        false
    }

    fn push(&mut self, jet_pattern: &JetPattern, chamber: &HashSet<Coordinate>) {
        let mut can_push: bool = false;
        match jet_pattern {
            JetPattern::Left => {
                let leftmost_coordinate = self.coordinates.get(&0).unwrap();
                can_push = leftmost_coordinate.y != 1
                    && chamber
                        .get(&Coordinate::new(
                            leftmost_coordinate.x,
                            leftmost_coordinate.y - 1,
                        ))
                        .is_none()
                    && chamber
                        .get(&Coordinate::new(
                            leftmost_coordinate.x + 1,
                            leftmost_coordinate.y - 1,
                        ))
                        .is_none();

                if can_push {
                    for (_, c) in self.coordinates.iter_mut() {
                        c.y = c.y - 1;
                    }
                }
            }
            JetPattern::Right => {
                let rightmost_coordinate = self.coordinates.get(&1).unwrap();
                can_push = rightmost_coordinate.y != 7
                    && chamber
                        .get(&Coordinate::new(
                            rightmost_coordinate.x,
                            rightmost_coordinate.y + 1,
                        ))
                        .is_none()
                    && chamber
                        .get(&Coordinate::new(
                            rightmost_coordinate.x + 1,
                            rightmost_coordinate.y + 1,
                        ))
                        .is_none();

                if can_push {
                    for (_, c) in self.coordinates.iter_mut() {
                        c.y = c.y + 1;
                    }
                }
            }
        }
    }

    fn all_coordinates(&self) -> HashSet<Coordinate> {
        let mut _all_coordinates: HashSet<Coordinate> = HashSet::with_capacity(4);
        let base_coordinate: &Coordinate = self.coordinates.get(&0).unwrap();

        for y_coordinate in base_coordinate.y..=(base_coordinate.y + 1) {
            _all_coordinates.insert(Coordinate::new(base_coordinate.x, y_coordinate));
            _all_coordinates.insert(Coordinate::new(base_coordinate.x + 1, y_coordinate));
        }

        _all_coordinates
    }
}

fn next_jet_pattern(patterns: &mut JetPatterns) -> Option<JetPattern> {
    if let Some(pattern) = patterns.pop_front() {
        patterns.push_back(pattern.clone());
        return Some(pattern);
    }

    None
}
