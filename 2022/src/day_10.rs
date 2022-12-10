use advent_of_code::read_lines;
use std::path::Path;
use std::collections::{BTreeMap};
use regex::Regex;
use std::fmt;
use log::{debug};


fn should_calculate_signal_strength(current_cycle: usize) -> bool {
    if current_cycle < 20 {
        return false
    }
    current_cycle == 20
        || (current_cycle - 20) % 40 == 0
}

pub fn part_one(input_path: &Path) -> i32 {
    let mut register_value: i32 = 1;

    let noop_regex = Regex::new(r"^noop").unwrap();
    let addx_regex = Regex::new(r"^addx\s+(?P<increment_by>-?\d+)").unwrap();
    let mut next_execution_cycle: usize = 1;
    let total_cycles: usize = 220;

    let mut cycle_executions: BTreeMap<usize, Vec<i32>> = BTreeMap::new();

    if let Ok(lines) = read_lines(input_path) {
        for (_, line) in lines.enumerate() {
            if let Ok(result) = line {
                match addx_regex.captures(&result) {
                    Some(capture) => {
                        match capture.name("increment_by") {
                            Some(increment_by) => {
                                let _increment_by: i32 = increment_by.as_str().parse().unwrap();
                                next_execution_cycle = next_execution_cycle + 2;

                                cycle_executions
                                    .entry(next_execution_cycle)
                                    .and_modify(|v| {
                                        v.push(_increment_by);
                                    })
                                    .or_insert(Vec::from([_increment_by]));
                            }
                            None => {}
                        }
                    },
                    None => {},
                };

                match noop_regex.captures(&result) {
                    Some(_) => {
                        cycle_executions
                            .entry(next_execution_cycle)
                            .and_modify(|v| v.push(0))
                            .or_insert(Vec::from([0]));
                        next_execution_cycle = next_execution_cycle + 1;
                    },
                    None => {},
                };
            }
        }
    }

    let mut signal_strengths: Vec<i32> = Vec::new();
    for current_cycle in 1..=total_cycles {
        debug!("Register value at start of cycle {}: {}", current_cycle, register_value);

        if let Some(_cycle_executions) = cycle_executions.remove(&current_cycle) {
            for cycle_execution in _cycle_executions.iter() {
                debug!("Executing {} in cycle {}", cycle_execution, current_cycle);
                register_value = register_value + cycle_execution;
            }
        }

        if should_calculate_signal_strength(current_cycle) {
            let signal_strength = (current_cycle as i32) * register_value;
            debug!("Signal strength at cycle {}: {}", current_cycle, signal_strength);
            signal_strengths.push(signal_strength);
        }
        debug!("Register value at end of cycle {}: {}", current_cycle, register_value);
    }

    signal_strengths.iter().sum()
}

pub fn part_two(input_path: &Path) -> String {
    let mut register_value: i32 = 1;

    let noop_regex = Regex::new(r"^noop").unwrap();
    let addx_regex = Regex::new(r"^addx\s+(?P<increment_by>-?\d+)").unwrap();
    let mut next_execution_cycle: usize = 0;

    let mut cycle_executions: BTreeMap<usize, Vec<i32>> = BTreeMap::new();

    if let Ok(lines) = read_lines(input_path) {
        for (_, line) in lines.enumerate() {
            if let Ok(result) = line {
                match addx_regex.captures(&result) {
                    Some(capture) => {
                        match capture.name("increment_by") {
                            Some(increment_by) => {
                                let _increment_by: i32 = increment_by.as_str().parse().unwrap();
                                next_execution_cycle = next_execution_cycle + 2;

                                cycle_executions
                                    .entry(next_execution_cycle)
                                    .and_modify(|v| {
                                        v.push(_increment_by);
                                    })
                                    .or_insert(Vec::from([_increment_by]));
                            }
                            None => {}
                        }
                    },
                    None => {},
                };

                match noop_regex.captures(&result) {
                    Some(_) => {
                        next_execution_cycle = next_execution_cycle + 1;
                    },
                    None => {},
                };
            }
        }
    }

    let total_cycles: usize = 240;
    let mut crt_drawing_position: usize = 0;
    let mut crt_rows: Vec<Vec<&str>> = Vec::with_capacity(6);
    let mut crt_row: Vec<&str> = Vec::with_capacity(40);

    let mut sprite_position: SpritePosition = SpritePosition::new();
    debug!("Sprite position: {}\n", sprite_position);

    for current_cycle in 1..=total_cycles {
        if let Some(_cycle_executions) = cycle_executions.remove(&(current_cycle - 1)) {
            let currently_executing_cycle = current_cycle - 1;

            for cycle_execution in _cycle_executions.iter() {
                register_value = register_value + cycle_execution;
                debug!("End of cycle  {}: finish executing addx {} (Register X is now {})", currently_executing_cycle, cycle_execution, register_value);

                match usize::try_from(register_value) {
                    Ok(value) => {
                        sprite_position.move_sprite(value);
                    },
                    Err(_) => {},
                }
                debug!("Sprite position: {}", sprite_position);
            }
        }

        if sprite_position.is_visible(crt_drawing_position) {
            crt_row.push("#");
        } else {
            crt_row.push(".");
        }

        debug!("During cycle  {}: CRT draws pixel in position {}", current_cycle, crt_drawing_position);
        debug!("Current CRT row: {}", crt_row.join(""));
        crt_drawing_position = (crt_drawing_position + 1) % 40;

        debug!("");

        if (current_cycle >= 40) && (current_cycle % 40 == 0) {
            crt_rows.push(crt_row);
            crt_row = Vec::with_capacity(40);
        }
    }

    format!("\n{}", crt_rows.iter().map(|r| r.join("")).collect::<Vec<String>>().join("\n"))
}


struct SpritePosition {
    sprites: BTreeMap<usize, String>,
}

impl SpritePosition {
    pub fn new() -> SpritePosition {
        let mut sprites: BTreeMap<usize, String> = BTreeMap::new();

        for i in 0..40 {
            sprites.insert(i, ".".to_string());
        }

        sprites.insert(0, "#".to_string());
        sprites.insert(1, "#".to_string());
        sprites.insert(2, "#".to_string());

        SpritePosition { sprites }
    }

    pub fn is_visible(&self, position: usize) -> bool {
        if let Some(sprite) = self.sprites.get(&position) {
            return sprite == &"#".to_string();
        }

        false
    }

    pub fn clear(&mut self) {
        for i in 0..40 {
            self.sprites.entry(i).and_modify(|s| *s = ".".to_string());
        }
    }

    pub fn move_sprite(&mut self, middle_position: usize) {
        self.clear();
        self.sprites.entry(middle_position-1).and_modify(|s| *s = "#".to_string());
        self.sprites.entry(middle_position).and_modify(|s| *s = "#".to_string());
        self.sprites.entry(middle_position+1).and_modify(|s| *s = "#".to_string());
    }
}

impl fmt::Display for SpritePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.sprites.values().cloned().collect::<Vec<String>>().join("")
        )
    }
}
