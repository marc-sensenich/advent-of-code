use advent_of_code::read_lines;
use log::{debug, log_enabled, Level};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::path::Path;

pub fn part_one(input_path: &Path) -> u64 {
    solve(input_path, 1000)
}

pub fn part_two(input_path: &Path) -> u64 {
    0
}

#[derive(Clone, Debug)]
enum FlipFlopState {
    On,
    Off,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Pulse {
    High,
    Low,
}

impl fmt::Display for Pulse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pulse::High => "high",
                Pulse::Low => "low",
            }
        )
    }
}

#[derive(Clone, Debug)]
struct ConjunctionModule {
    id: String,
    memory: HashMap<String, Pulse>,
    destination_ids: Vec<String>,
}

impl ConjunctionModule {
    pub fn new(id: String, destination_ids: Vec<String>) -> ConjunctionModule {
        let mut memory: HashMap<String, Pulse> = HashMap::new();

        ConjunctionModule {
            id,
            destination_ids,
            memory: memory.clone(),
        }
    }
}

impl Module for ConjunctionModule {
    fn broadcast_outputs(&self) -> Vec<WorkItem> {
        let mut items_to_publish: Vec<WorkItem> = vec![];
        for destination_id in &self.destination_ids {
            // println!("{} -BroadcastOutput-> {}", self.id, destination_id);
            items_to_publish.push(WorkItem::new(
                self.id.to_string(),
                destination_id.to_string(),
                WorkAction::BroadcastOutput,
                None,
            ));
        }

        items_to_publish
    }

    fn receive(&mut self, work_item: &WorkItem) -> Vec<WorkItem> {
        match work_item.action {
            WorkAction::BroadcastOutput => {
                if work_item.consumer == self.id {
                    self.memory
                        .entry(work_item.publisher.clone())
                        .or_insert(Pulse::Low);
                }

                vec![]
            }
            WorkAction::Receive => {
                if let Some(pulse) = work_item.pulse {
                    if self.memory.contains_key(&work_item.publisher) {
                        self.memory
                            .entry(work_item.publisher.clone())
                            .and_modify(|e| *e = pulse);
                    }

                    return self.publish();
                }

                vec![]
            }
            _ => vec![],
        }
    }

    fn publish(&mut self) -> Vec<WorkItem> {
        let pulse = match self
            .memory
            .values()
            .copied()
            .filter(|f| *f == Pulse::Low)
            .collect::<Vec<_>>()
            .len()
        {
            0 => Pulse::Low,
            _ => Pulse::High,
        };

        let mut items_to_publish: Vec<WorkItem> = vec![];
        for destination_id in &self.destination_ids {
            debug!("{} -{}-> {}", self.id, pulse, destination_id);
            items_to_publish.push(WorkItem::new(
                self.id.to_string(),
                destination_id.to_string(),
                WorkAction::Receive,
                Some(pulse),
            ));
        }
        items_to_publish
    }
}

trait Module {
    fn receive(&mut self, work_item: &WorkItem) -> Vec<WorkItem>;
    fn publish(&mut self) -> Vec<WorkItem>;
    // Tell everybody I'm a publisher
    fn broadcast_outputs(&self) -> Vec<WorkItem>;
}

#[derive(Clone, Debug)]
struct NoOpModule {
    id: String,
}

impl NoOpModule {
    pub fn new(id: String) -> NoOpModule {
        NoOpModule { id }
    }
}

impl Module for NoOpModule {
    fn broadcast_outputs(&self) -> Vec<WorkItem> {
        vec![]
    }

    fn publish(&mut self) -> Vec<WorkItem> {
        vec![]
    }

    fn receive(&mut self, work_item: &WorkItem) -> Vec<WorkItem> {
        vec![]
    }
}
#[derive(Clone, Debug)]
struct BroadcastModule {
    id: String,
    pulse_to_broadcast: Pulse,
    destination_ids: Vec<String>,
}

impl Module for BroadcastModule {
    fn broadcast_outputs(&self) -> Vec<WorkItem> {
        let mut items_to_publish: Vec<WorkItem> = vec![];
        for destination_id in &self.destination_ids {
            items_to_publish.push(WorkItem::new(
                self.id.to_string(),
                destination_id.to_string(),
                WorkAction::BroadcastOutput,
                None,
            ));
        }

        items_to_publish
    }

    fn receive(&mut self, work_item: &WorkItem) -> Vec<WorkItem> {
        if let Some(pulse) = work_item.pulse {
            if work_item.action == WorkAction::Receive {
                debug!("{} -{}-> {}", work_item.publisher, pulse, self.id);
                self.pulse_to_broadcast = pulse;
                return self.publish();
            }
        }

        vec![]
    }

    fn publish(&mut self) -> Vec<WorkItem> {
        let mut items_to_publish: Vec<WorkItem> = vec![];
        for destination_id in &self.destination_ids {
            debug!(
                "{} -{}-> {}",
                self.id, self.pulse_to_broadcast, destination_id
            );
            items_to_publish.push(WorkItem::new(
                self.id.to_string(),
                destination_id.to_string(),
                WorkAction::Receive,
                Some(self.pulse_to_broadcast),
            ));
        }
        items_to_publish
    }
}

impl BroadcastModule {
    pub fn new(id: String, destination_ids: Vec<String>) -> BroadcastModule {
        BroadcastModule {
            id,
            destination_ids,
            pulse_to_broadcast: Pulse::Low,
        }
    }
}

#[derive(Clone, Debug)]
struct FlipFlopModule {
    id: String,
    state: FlipFlopState,
    destination_ids: Vec<String>,
}

impl Module for FlipFlopModule {
    fn receive(&mut self, work_item: &WorkItem) -> Vec<WorkItem> {
        if work_item.action == WorkAction::Receive {
            if let Some(pulse) = work_item.pulse {
                return match pulse {
                    Pulse::High => vec![],
                    Pulse::Low => {
                        match self.state {
                            FlipFlopState::Off => {
                                self.state = FlipFlopState::On;
                            }
                            FlipFlopState::On => {
                                self.state = FlipFlopState::Off;
                            }
                        }

                        self.publish()
                    }
                };
            }
        }

        vec![]
    }

    fn publish(&mut self) -> Vec<WorkItem> {
        let pulse_to_publish: Pulse = match self.state {
            FlipFlopState::On => Pulse::High,
            FlipFlopState::Off => Pulse::Low,
        };

        let mut items_to_publish: Vec<WorkItem> = vec![];
        for destination_id in &self.destination_ids {
            debug!("{} -{}-> {}", self.id, pulse_to_publish, destination_id);
            items_to_publish.push(WorkItem::new(
                self.id.to_string(),
                destination_id.to_string(),
                WorkAction::Receive,
                Some(pulse_to_publish),
            ));
        }

        items_to_publish
    }

    fn broadcast_outputs(&self) -> Vec<WorkItem> {
        let mut items_to_publish: Vec<WorkItem> = vec![];
        for destination_id in &self.destination_ids {
            // println!("{} -BroadcastOutput-> {}", self.id, destination_id);
            items_to_publish.push(WorkItem::new(
                self.id.to_string(),
                destination_id.to_string(),
                WorkAction::BroadcastOutput,
                None,
            ));
        }

        items_to_publish
    }
}

impl FlipFlopModule {
    pub fn new(id: String, destination_ids: Vec<String>) -> FlipFlopModule {
        FlipFlopModule {
            id,
            destination_ids,
            state: FlipFlopState::Off,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum WorkAction {
    Publish,
    Receive,
    BroadcastOutput,
    BroadcastRequest,
}

#[derive(Clone, Debug)]
struct WorkItem {
    publisher: String,
    consumer: String,
    action: WorkAction,
    pulse: Option<Pulse>,
}

impl WorkItem {
    pub fn new(
        publisher: String,
        consumer: String,
        action: WorkAction,
        pulse: Option<Pulse>,
    ) -> WorkItem {
        WorkItem {
            publisher,
            consumer,
            action,
            pulse,
        }
    }
}

fn solve(input_path: &Path, iterations: usize) -> u64 {
    let mut _iterations: usize = iterations;
    let mut work_queue: VecDeque<WorkItem> = VecDeque::new();
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                let split_result: Vec<&str> = result.split(" -> ").collect::<Vec<_>>();
                let destination_ids_str = split_result.get(1).unwrap().replace(" ", "");
                let destination_ids = destination_ids_str
                    .split(",")
                    .map(|m| m.to_string())
                    .collect::<Vec<String>>();
                let mut module_id: String = split_result.get(0).unwrap().to_string();

                if module_id.contains("&") {
                    module_id = module_id.clone().replace("&", "");

                    modules.insert(
                        module_id.clone(),
                        Box::new(ConjunctionModule::new(module_id.clone(), destination_ids)),
                    );
                } else if module_id.contains("%") {
                    module_id = module_id.clone().replace("%", "");

                    modules.insert(
                        module_id.clone(),
                        Box::new(FlipFlopModule::new(module_id.clone(), destination_ids)),
                    );
                } else {
                    if module_id == "broadcaster" {
                        modules.insert(
                            module_id.clone(),
                            Box::new(BroadcastModule::new(module_id.clone(), destination_ids)),
                        );
                    } else {
                        modules.insert(
                            module_id.clone(),
                            Box::new(NoOpModule::new(module_id.clone())),
                        );
                    }
                }
            }
        }
    }

    if !modules.contains_key("rx") {
        modules.insert(
            "rx".to_string(),
            Box::new(NoOpModule::new("rx".to_string())),
        );
    }

    let mut low_pulses_sent: u64 = 0;
    let mut high_pulses_sent: u64 = 0;

    for idx in 1..=_iterations {
        if idx == 1 {
            work_queue.push_back(WorkItem::new(
                String::from("*"),
                String::from("*"),
                WorkAction::BroadcastRequest,
                None,
            ));
        } else {
            work_queue.push_back(WorkItem::new(
                String::from("button"),
                String::from("broadcaster"),
                WorkAction::Publish,
                Some(Pulse::Low),
            ));
            low_pulses_sent += 1;
        }
        debug!("button -low-> broadcaster");
        while !work_queue.is_empty() {
            let work_item: WorkItem = work_queue.pop_front().unwrap();

            if work_item.action == WorkAction::BroadcastOutput
            {
                for module in modules.values_mut() {
                    module.receive(&work_item);
                }
            } else if work_item.action == WorkAction::BroadcastRequest {
                for module in modules.values() {
                    work_queue.extend(module.broadcast_outputs());
                }
                work_queue.push_back(WorkItem::new(
                    String::from("button"),
                    String::from("broadcaster"),
                    WorkAction::Publish,
                    Some(Pulse::Low),
                ));
                low_pulses_sent += 1;
            } else {
                let module: &mut Box<dyn Module> =
                    modules.get_mut(&work_item.consumer).unwrap();
                match work_item.action {
                    WorkAction::Publish => {
                        work_queue.extend(module.publish());
                    }
                    WorkAction::Receive => {
                        work_queue.extend(module.receive(&work_item));
                    }
                    _ => {}
                }

                if work_item.action == WorkAction::Receive {
                    if let Some(pulse) = work_item.pulse {
                        match pulse {
                            Pulse::High => high_pulses_sent += 1,
                            Pulse::Low => low_pulses_sent += 1,
                        }
                    }
                }
            }
        }
    }

    low_pulses_sent * high_pulses_sent
}
