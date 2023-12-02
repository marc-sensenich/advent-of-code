use advent_of_code::read_lines;
use log::{debug, log_enabled, Level};
use std::path::Path;
use regex::Regex;


pub fn part_one(input_path: &Path) -> i32 {
    let mut sum: i32 = 0;

    let games: Vec<Game> = load_games(input_path);

    let mut game_result: u32 = 0;
    games.iter().for_each(|g| {
        game_result = g.id;

        g.rounds.iter().for_each(|r| {
            if r.red_cubes > 12 || r.green_cubes > 13 || r.blue_cubes > 14 {
                game_result = 0;
            }
        });

        sum += game_result as i32;
    });

    sum
}

pub fn part_two(input_path: &Path) -> i32 {
    let mut sum: i32 = 0;

    let mut red_cubes: Vec<u32> = vec![];
    let mut green_cubes: Vec<u32> = vec![];
    let mut blue_cubes: Vec<u32> = vec![];

    let games: Vec<Game> = load_games(input_path);

    games.iter().for_each(|g| {
        g.rounds.iter().for_each(|r| {
            red_cubes.push(r.red_cubes);
            green_cubes.push(r.green_cubes);
            blue_cubes.push(r.blue_cubes);
        });

        let game_power = red_cubes.iter().max().unwrap() * green_cubes.iter().max().unwrap() * blue_cubes.iter().max().unwrap();
        red_cubes.clear();
        green_cubes.clear();
        blue_cubes.clear();

        sum += game_power as i32;
    });

    sum
}

#[derive(Debug, Clone)]
struct Round {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn load_games(input_path: &Path) -> Vec<Game> {
    let mut games: Vec<Game> = vec![];

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(mut result) = line {
                let mut game = Game{id: 0, rounds: vec![] };

                result = result.replace(" ", "");
                let game_id = result.split_once(":").unwrap().0.to_string().replace("Game", "").parse::<u32>().unwrap();
                game.id = game_id;
                let rounds: Vec<_> = result.split_once(":").unwrap().1.split(";").collect::<Vec<_>>();
                for round in &rounds {
                    let mut new_round = Round{red_cubes: 0, green_cubes: 0, blue_cubes: 0};

                    let regex = Regex::new(r"(?m)((?P<red>\d+)red)?((?P<green>\d+)green)?((?P<blue>\d+)blue)?").unwrap();
                    let result = regex.captures_iter(&round);
                    for mat in result {
                        match mat.name("red") {
                            Some(cubes) => {
                                new_round.red_cubes = cubes.as_str().to_string().parse().unwrap();
                            }
                            None => {}
                        }
                        match mat.name("green") {
                            Some(cubes) => {
                                new_round.green_cubes = cubes.as_str().to_string().parse().unwrap();
                            }
                            None => {}
                        }
                        match mat.name("blue") {
                            Some(cubes) => {
                                new_round.blue_cubes = cubes.as_str().to_string().parse().unwrap();
                            }
                            None => {}
                        }
                    }

                    game.rounds.push(new_round.clone());
                }

                if game.id != 0 {
                    games.push(game.clone());
                }
            }
        }
    }

    games
}
