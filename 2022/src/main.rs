use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod day_01;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_14;
mod day_17;
mod day_18;

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
    DayThree {},
    DayFour {},
    DayFive {},
    DaySix { input_string: String },
    DaySeven {},
    DayEight {},
    DayNine {},
    DayTen {},
    DayEleven {},
    DayTwelve {},
    DayFourteen {},
    DaySeventeen {},
    DayEighteen {},
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
                println!("Part two answer: {}", day_05::part_two(input_path));
            }
        }
        Some(Commands::DaySix { input_string }) => {
            match day_06::part_one(input_string.to_string()) {
                Ok(index) => println!("Part one answer: {}", index),
                Err(err) => println!("Part one error: {}", err),
            }

            match day_06::part_two(input_string.to_string()) {
                Ok(index) => println!("Part two answer: {}", index),
                Err(err) => println!("Part two error: {}", err),
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
                println!("Part two answer: {}", day_10::part_two(input_path));
            }
        }
        Some(Commands::DayEleven {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_11::part_one(input_path));
                println!("Part two answer: {}", day_11::part_two(input_path));
            }
        }
        Some(Commands::DayTwelve {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_12::part_one(input_path));
                println!("Part two answer: {}", day_12::part_two(input_path));
            }
        }
        Some(Commands::DayFourteen {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_14::part_one(input_path));
                println!("Part two answer: {}", day_14::part_two(input_path));
            }
        }
        Some(Commands::DaySeventeen {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_17::part_one(input_path));
                println!("Part two answer: {}", day_17::part_two(input_path));
            }
        }
        Some(Commands::DayEighteen {}) => {
            if let Some(input_path) = cli.input.as_deref() {
                println!("Part one answer: {}", day_18::part_one(input_path));
                println!("Part two answer: {}", day_18::part_two(input_path));
            }
        }
        None => {}
    }
}
