use advent_of_code::{read_file_to_string, read_lines};
use log::{debug, log_enabled, Level};
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn part_one(input_path: &Path) -> u64 {
    match read_file_to_string(input_path) {
        Ok(input) => solve_part_one(input.as_str()),
        _ => 0,
    }
}

fn solve_part_one(input: &str) -> u64 {
    let mut wires: Wires = parse_input(input);
    let binding = wires.clone();
    let mut keys = binding.keys().collect::<Vec<_>>();
    keys.sort();

    let test = keys
        .into_iter()
        .filter(|&k| k.starts_with("z"))
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .map(
            |i| match determine_wire_value(String::from(*i), &mut wires) {
                true => String::from("1"),
                false => String::from("0"),
            },
        )
        .collect::<Vec<String>>()
        .join("");

    u64::from_str_radix(test.as_str(), 2).unwrap()
}

fn determine_wire_value(name: String, wires: &mut Wires) -> bool {
    let wire = wires.get_mut(&name).unwrap().clone();

    match wire.value {
        Some(v) => v,
        None => {
            let left_wire_value = determine_wire_value(wire.left_operand.unwrap(), wires);
            let right_wire_value = determine_wire_value(wire.right_operand.unwrap(), wires);

            match wire.operation.unwrap() {
                LogicOperation::AND => left_wire_value && right_wire_value,
                LogicOperation::OR => left_wire_value || right_wire_value,
                LogicOperation::XOR => left_wire_value ^ right_wire_value,
                _ => false,
            }
        }
    }
}

fn parse_input(input: &str) -> Wires {
    let mut wires: Wires = HashMap::new();

    let split_input = input.split("\n\n").collect::<Vec<_>>();

    split_input[0].lines().for_each(|l| {
        let split_line = l.split(": ").collect::<Vec<_>>();
        let name = split_line[0].to_string();
        let value = match split_line[1] {
            "1" => true,
            _ => false,
        };

        wires.insert(
            name.clone(),
            Wire::new(name.clone(), Some(value), None, None, None),
        );
    });

    split_input[1].lines().for_each(|l| {
        let split_for_output = l.split(" -> ").collect::<Vec<_>>();
        let name = split_for_output[1].to_string();
        let split_for_logic = split_for_output[0].split(" ").collect::<Vec<_>>();

        let left_operand = split_for_logic[0].to_string();
        let right_operand = split_for_logic[2].to_string();
        let operation = match split_for_logic[1] {
            "AND" => LogicOperation::AND,
            "OR" => LogicOperation::OR,
            "XOR" => LogicOperation::XOR,
            _ => LogicOperation::Unknown,
        };

        wires.insert(
            name.clone(),
            Wire::new(
                name.clone(),
                None,
                Some(left_operand),
                Some(right_operand),
                Some(operation),
            ),
        );
    });

    wires
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum LogicOperation {
    AND,
    OR,
    XOR,
    Unknown,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Wire {
    name: String,
    value: Option<bool>,
    left_operand: Option<String>,
    right_operand: Option<String>,
    operation: Option<LogicOperation>,
}

impl Wire {
    fn new(
        name: String,
        value: Option<bool>,
        left_operand: Option<String>,
        right_operand: Option<String>,
        operation: Option<LogicOperation>,
    ) -> Wire {
        Wire {
            name,
            value,
            left_operand,
            right_operand,
            operation,
        }
    }
}

type Wires = HashMap<String, Wire>;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_part_one_simple() {
        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        let expected: u64 = 4;
        let got: u64 = solve_part_one(input);

        assert_eq!(got, expected);
    }

    #[test]
    fn solve_part_one_larger() {
        let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

        let expected: u64 = 2024;
        let got: u64 = solve_part_one(input);

        assert_eq!(got, expected);
    }
}
