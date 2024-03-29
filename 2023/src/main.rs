use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_19;
mod day_20;
mod day_21;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    input_string: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", global = true)]
    input: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    DayOne {},
    DayTwo {},
    DayThree {},
    DayFour {},
    DayFive {},
    DaySix {},
    DaySeven {},
    DayEight {},
    DayNine {},
    DayTen {},
    DayEleven {},
    DayThirteen {},
    DayFourteen {},
    DayFifteen {},
    DaySixteen {},
    DayNineteen {},
    DayTwenty {},
    DayTwentyOne {
        #[arg(short, long)]
        goal_distance: u64,
    },
}

fn main() {
    let cli = Cli::parse();
    env_logger::init();

    match &cli.command {
        Some(Commands::DayOne {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_01::part_one(input_path));
                println!("Part two answer: {}", day_01::part_two(input_path));
            }
        }
        Some(Commands::DayTwo {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_02::part_one(input_path));
                println!("Part two answer: {}", day_02::part_two(input_path));
            }
        }
        Some(Commands::DayThree {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_03::part_one(input_path));
                println!("Part two answer: {}", day_03::part_two(input_path));
            }
        }
        Some(Commands::DayFour {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_04::part_one(input_path));
                println!("Part two answer: {}", day_04::part_two(input_path));
            }
        }
        Some(Commands::DayFive {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_05::part_one(input_path));
                // println!("Part two answer: {}", day_05::part_two(input_path));
            }
        }
        Some(Commands::DaySix {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_06::part_one(input_path));
                println!("Part two answer: {}", day_06::part_two(input_path));
            }
        }
        Some(Commands::DaySeven {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_07::part_one(input_path));
                println!("Part two answer: {}", day_07::part_two(input_path));
            }
        }
        Some(Commands::DayEight {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_08::part_one(input_path));
                println!("Part two answer: {}", day_08::part_two(input_path));
            }
        }
        Some(Commands::DayNine {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_09::part_one(input_path));
                println!("Part two answer: {}", day_09::part_two(input_path));
            }
        }
        Some(Commands::DayTen {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_10::part_one(input_path));
                // println!("Part two answer: {}", day_10::part_two(input_path));
            }
        }
        Some(Commands::DayEleven {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_11::part_one(input_path));
                println!("Part two answer: {}", day_11::part_two(input_path));
            }
        }
        Some(Commands::DayThirteen {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_13::part_one(input_path));
                // println!("Part two answer: {}", day_13::part_two(input_path));
            }
        }
        Some(Commands::DayFourteen {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_14::part_one(input_path));
                // println!("Part two answer: {}", day_14::part_two(input_path));
            }
        }
        Some(Commands::DayFifteen {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_15::part_one(input_path));
                println!("Part two answer: {}", day_15::part_two(input_path));
            }
        }
        Some(Commands::DaySixteen {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_16::part_one(input_path));
                println!("Part two answer: {}", day_16::part_two(input_path));
            }
        }
        Some(Commands::DayNineteen {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_19::part_one(input_path));
                // println!("Part two answer: {}", day_19::part_two(input_path));
            }
        }
        Some(Commands::DayTwenty {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_20::part_one(input_path));
                println!("Part two answer: {}", day_20::part_two(input_path));
            }
        }
        Some(Commands::DayTwentyOne { goal_distance }) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!(
                    "Part one answer: {}",
                    day_21::part_one(input_path, *goal_distance)
                );
                // println!("Part two answer: {}", day_21::part_two(input_path));
            }
        }
        None => {}
    }
}
