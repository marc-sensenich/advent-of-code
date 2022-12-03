use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let priorities = generate_priorities();
    let mut total = 0;

    if let Ok(lines) = read_lines(config.file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(result) = line {
                let mut first_compartment: String = result.clone();
                let compartment_length = first_compartment.trim().len() / 2;

                let second_compartment: String = first_compartment.split_off(compartment_length);

                let first_compartment_set: HashSet<char> =
                    HashSet::from_iter(first_compartment.chars());
                let second_compartment_set: HashSet<char> =
                    HashSet::from_iter(second_compartment.chars());

                let intersection = first_compartment_set.intersection(&second_compartment_set);
                match intersection.last() {
                    Some(duplicate) => {
                        if let Some(value) = priorities.get(&duplicate) {
                            total += *value;
                        }
                    }
                    None => println!("No duplicate found in the compartments"),
                }
            }
        }
    }

    println!("{}", total);

    Ok(())
}

fn generate_priorities() -> HashMap<char, i32> {
    let mut priorities: HashMap<char, i32> = HashMap::new();
    // Character iteration from https://users.rust-lang.org/t/iteration-thought-alphabets/30078/4
    let letters = b'a'..=b'z';
    for (i, c) in letters.enumerate() {
        let char_c = c as char;
        priorities.insert(char_c, (i + 1).try_into().unwrap());
        priorities.insert(char_c.to_ascii_uppercase(), (i + 27).try_into().unwrap());
    }

    priorities
}

// read_lines from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Config {
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("not enough arguments");
        }
        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}
