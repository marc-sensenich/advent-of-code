use std::path::PathBuf;
mod day_01;
mod day_03;
mod day_04;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

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
}

fn main() {
    let cli = Cli::parse();

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
        None => {}
    }
}
