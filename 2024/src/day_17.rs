use advent_of_code::{read_file_to_string, read_lines};
use log::{debug, log_enabled, Level};
use regex::Regex;
use std::path::Path;

pub fn part_one(input_path: &Path) -> String {
    match read_file_to_string(input_path) {
        Ok(input) => solve_part_one(input.as_str()),
        _ => String::from(""),
    }
}

fn solve_part_one(input: &str) -> String {
    let mut c = parse_input(input);

    c.execute_instructions();

    c.output
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

type Register = u64;
type Operand = u64;
type Instruction = u64;
type Instructions = Vec<Instruction>;

#[derive(Debug, PartialEq)]
struct Computer {
    register_a: Register,
    register_b: Register,
    register_c: Register,
    instructions: Instructions,
    instruction_pointer: usize,
    output: Vec<u64>,
}

impl Computer {
    pub fn new(
        register_a: Register,
        register_b: Register,
        register_c: Register,
        instructions: Instructions,
    ) -> Computer {
        Computer {
            register_a,
            register_b,
            register_c,
            instructions,
            instruction_pointer: 0,
            output: vec![],
        }
    }

    pub fn execute_instructions(&mut self) {
        let mut instruction = self.instructions.get(self.instruction_pointer);
        let mut operand = self.instructions.get(self.instruction_pointer + 1);

        while let Some(&i) = instruction {
            if let Some(&o) = operand {
                match i {
                    0 | 6 | 7 => {
                        if let Some(c) = self.get_combo_operand(o) {
                            let result = self.adv_instruction(c);

                            match i {
                                0 => self.register_a = result,
                                6 => self.register_b = result,
                                7 => self.register_c = result,
                                _ => {}
                            }
                        }
                    }
                    1 => self.register_b = self.register_b ^ o,
                    2 => {
                        if let Some(c) = self.get_combo_operand(o) {
                            self.register_b = c.rem_euclid(8);
                        }
                    }
                    3 => {
                        if self.register_a != 0 {
                            self.instruction_pointer = o as usize;
                            instruction = self.instructions.get(self.instruction_pointer);
                            operand = self.instructions.get(self.instruction_pointer + 1);
                            continue;
                        }
                    }
                    4 => self.register_b = self.register_b ^ self.register_c,
                    5 => {
                        if let Some(c) = self.get_combo_operand(o) {
                            self.output.push(c.rem_euclid(8));
                        }
                    }
                    _ => {}
                }
            }

            self.instruction_pointer += 2;
            instruction = self.instructions.get(self.instruction_pointer);
            operand = self.instructions.get(self.instruction_pointer + 1);
        }
    }

    fn get_combo_operand(&self, operand: Operand) -> Option<u64> {
        match operand {
            0 | 1 | 2 | 3 => Some(operand as u64),
            4 => Some(self.register_a),
            5 => Some(self.register_b),
            6 => Some(self.register_c),
            _ => None,
        }
    }

    fn adv_instruction(&self, operand: Operand) -> u64 {
        self.register_a / 2_u64.pow(operand as u32)
    }
}

fn parse_input(input: &str) -> Computer {
    let regex = Regex::new(r"(?m)Register\s+A:\s+(?<a>\d+)\nRegister\s+B:\s+(?<b>\d+)\nRegister\s+C:\s+(?<c>\d+)\n\nProgram:\s+(?<program>.+)").unwrap();

    let c = regex
        .captures_iter(input)
        .map(|capture| {
            Computer::new(
                capture["a"].parse().unwrap(),
                capture["b"].parse().unwrap(),
                capture["c"].parse().unwrap(),
                capture["program"]
                    .split(",")
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .nth(0)
        .unwrap();

    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_example() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

        let expected = Computer::new(729, 0, 0, vec![0, 1, 5, 4, 3, 0]);

        let got = parse_input(input);

        assert_eq!(got, expected);
    }

    #[test]
    fn solve_part_one_example() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let expected = "4,6,3,5,6,3,5,2,1,0";
        let got = solve_part_one(input);

        assert_eq!(got, expected);
    }

    #[test]
    fn execute_instructions_example_one() {
        let expected: u64 = 1;
        let mut computer: Computer = Computer::new(0, 0, 9, vec![2, 6]);

        computer.execute_instructions();

        assert_eq!(computer.register_b, expected)
    }

    #[test]
    fn execute_instructions_example_two() {
        let expected: Vec<u64> = vec![0, 1, 2];
        let mut computer: Computer = Computer::new(10, 0, 0, vec![5, 0, 5, 1, 5, 4]);

        computer.execute_instructions();

        assert_eq!(computer.output, expected)
    }

    #[test]
    fn execute_instructions_example_three() {
        let expected_register_a: u64 = 0;
        let expected_output: Vec<u64> = vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0];
        let mut computer: Computer = Computer::new(2024, 0, 0, vec![0, 1, 5, 4, 3, 0]);

        computer.execute_instructions();

        assert_eq!(computer.output, expected_output);
        assert_eq!(computer.register_a, expected_register_a);
    }

    #[test]
    fn execute_instructions_example_four() {
        let expected_register_b: u64 = 26;
        let mut computer: Computer = Computer::new(0, 29, 0, vec![1, 7]);

        computer.execute_instructions();

        assert_eq!(computer.register_b, expected_register_b);
    }

    #[test]
    fn execute_instructions_example_five() {
        let expected_register_b: u64 = 44354;
        let mut computer: Computer = Computer::new(0, 2024, 43690, vec![4, 0]);

        computer.execute_instructions();

        assert_eq!(computer.register_b, expected_register_b);
    }
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
