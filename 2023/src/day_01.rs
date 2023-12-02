use advent_of_code::read_lines;
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    let potential_coordinates: Vec<PotentialCoordinate> = vec![
        PotentialCoordinate {
            potential_ids: vec!["1".to_string()],
            value: "1".to_string(),
        },
        PotentialCoordinate {
            potential_ids: vec!["2".to_string()],
            value: "2".to_string(),
        },
        PotentialCoordinate {
            potential_ids: vec!["3".to_string()],
            value: "3".to_string(),
        },
        PotentialCoordinate {
            potential_ids: vec!["4".to_string()],
            value: "4".to_string(),
        },
        PotentialCoordinate {
            potential_ids: vec!["5".to_string()],
            value: "5".to_string(),
        },
        PotentialCoordinate {
            potential_ids: vec!["6".to_string()],
            value: "6".to_string(),
        },
        PotentialCoordinate {
            potential_ids: vec!["7".to_string()],
            value: "7".to_string(),
        },
        PotentialCoordinate {
            potential_ids: vec!["8".to_string()],
            value: "8".to_string(),
        },
        PotentialCoordinate {
            potential_ids: vec!["9".to_string()],
            value: "9".to_string(),
        },
    ];

    solve(input_path, potential_coordinates)
}

pub fn part_two(input_path: &Path) -> i32 {
    let potential_coordinates: Vec<PotentialCoordinate> = vec![
        PotentialCoordinate {
            potential_ids: vec!["1".to_string(), "one".to_string()],
            value: "1".to_string(),
        },
        PotentialCoordinate {
            potential_ids: vec!["2".to_string(), "two".to_string()],
            value: "2".to_string(),
        },
        PotentialCoordinate {
            potential_ids: vec!["3".to_string(), "three".to_string()],
            value: "3".to_string(),
        },
        PotentialCoordinate {
            potential_ids: vec!["4".to_string(), "four".to_string()],
            value: "4".to_string(),
        },
        PotentialCoordinate {
            potential_ids: vec!["5".to_string(), "five".to_string()],
            value: "5".to_string(),
        },
        PotentialCoordinate {
            potential_ids: vec!["6".to_string(), "six".to_string()],
            value: "6".to_string(),
        },
        PotentialCoordinate {
            potential_ids: vec!["7".to_string(), "seven".to_string()],
            value: "7".to_string(),
        },
        PotentialCoordinate {
            potential_ids: vec!["8".to_string(), "eight".to_string()],
            value: "8".to_string(),
        },
        PotentialCoordinate {
            potential_ids: vec!["9".to_string(), "nine".to_string()],
            value: "9".to_string(),
        },
    ];

    solve(input_path, potential_coordinates)
}

#[derive(Debug)]
struct PotentialCoordinate {
    potential_ids: Vec<String>,
    value: String,
}

#[derive(Debug)]
struct LocatedCoordinate {
    rank: i32,
    value: String,
}

fn solve(input_path: &Path, potential_coordinates: Vec<PotentialCoordinate>) -> i32 {
    let mut sum: i32 = 0;

    let mut located_coordinates: Vec<LocatedCoordinate> = Vec::new();

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                for potential_coordinate in &potential_coordinates {
                    for potential_id in &potential_coordinate.potential_ids {
                        for located_coordinate in result.match_indices(potential_id.as_str()).collect::<Vec<_>>() {
                            located_coordinates.push(LocatedCoordinate{rank: located_coordinate.0 as i32, value: potential_coordinate.value.as_str().to_string()});
                        }
                    }
                }

                located_coordinates.sort_by_key(|lc| lc.rank);

                let mut located_coordinate_value = 0;
                if located_coordinates.len() > 1 {
                    located_coordinate_value = format!("{}{}", located_coordinates.get(0).unwrap().value, located_coordinates.get(located_coordinates.len() - 1).unwrap().value).parse::<i32>().unwrap();
                } else if located_coordinates.len() == 1 {
                    located_coordinate_value = format!("{}{}", located_coordinates.get(0).unwrap().value, located_coordinates.get(0).unwrap().value).parse::<i32>().unwrap();
                }

                sum += located_coordinate_value;
                located_coordinates.clear();
            }
        }
    }

    sum
}
