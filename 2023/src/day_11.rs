use std::collections::HashMap;
use advent_of_code::{read_file_to_string, Coordinate};
use log::{debug, log_enabled, Level};
use std::path::Path;

pub fn part_one(input_path: &Path) -> u64 {
    solve(input_path, 1)
}

pub fn part_two(input_path: &Path) -> u64 {
    solve(input_path, 1000000)
}

fn solve(input_path: &Path, expansion_modifier: usize) -> u64 {
    let _expansion_modifier: usize = match expansion_modifier {
        0 | 1 => 1,
        _ => expansion_modifier - 1,
    };

    let mut input_image: String = String::from("");
    if let Ok(result) = read_file_to_string(input_path) {
        input_image = result;
    }

    let (rows_to_expand, columns_to_expand) = calculate_rows_and_columns_to_expand(&input_image);
    let mut universe: HashMap<i32, Coordinate> = HashMap::new();
    let mut galaxy_count: i32 = 1;
    let mut galaxy_ids: Vec<i32> = vec![];

    for (x, row) in input_image.as_str().split_whitespace().enumerate() {
        for (y, space) in row.split("").enumerate() {
            if space == "#" {
                let updated_x = rows_to_expand.iter().copied().filter(|r| r < &x).collect::<Vec<_>>().len() * _expansion_modifier;
                let updated_y = columns_to_expand.iter().copied().filter(|c| c < &y).collect::<Vec<_>>().len() * _expansion_modifier;
                universe.insert(galaxy_count, Coordinate::new((x + updated_x) as i32, (y + updated_y) as i32));
                galaxy_ids.push(galaxy_count);
                galaxy_count += 1;
            }
        }
    }

    let mut sum: u64 = 0;
    for galaxy_id in &galaxy_ids {
        for other_galaxy_id in (*galaxy_id + 1) as usize..galaxy_count as usize {
            let left = universe.get(&galaxy_id).unwrap();
            let right = universe.get(&(other_galaxy_id as i32)).unwrap();
            let distance: u64 = ((left.x - right.x).abs() + (left.y - right.y).abs()) as u64;
            sum += distance;
        }
    }

    sum
}

fn calculate_rows_and_columns_to_expand(input_map: &String) -> (Vec<usize>, Vec<usize>) {
    let mut rows_to_expand: Vec<usize> = vec![];
    let mut columns_to_expand: Vec<usize> = vec![];

    let mut input_map_rows = input_map.as_str().split_whitespace().collect::<Vec<&str>>();
    let row_count = input_map_rows.len();
    let column_count = input_map.find('\n').unwrap_or(0);

    for (idx, row) in input_map_rows.iter().enumerate() {
        let galaxy_count= row.matches("#").collect::<Vec<&str>>().len() as u32;
        if galaxy_count == 0 {
            rows_to_expand.push(idx);
        }
    }

    let mut empty_spaces_in_column = 0;
    for i in 0..column_count {
        for j in 0..row_count {
            match input_map_rows.get(j).unwrap().get(i..=i).unwrap_or("") {
                "." => empty_spaces_in_column += 1,
                _ => {}
            }
        }

        if empty_spaces_in_column == row_count {
            columns_to_expand.push(i);
        }

        empty_spaces_in_column = 0;
    }

    (rows_to_expand, columns_to_expand)
}
