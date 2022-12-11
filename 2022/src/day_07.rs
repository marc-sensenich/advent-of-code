use advent_of_code::read_lines;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    let directory_tree: HashMap<String, Directory> = build_directory_tree(input_path);

    directory_tree
        .iter()
        .map(|(_, d)| sum_directory(&d, &directory_tree))
        .collect::<Vec<i32>>()
        .into_iter()
        .filter(|s| s < &100000)
        .collect::<Vec<i32>>()
        .into_iter()
        .sum()
}

pub fn part_two(input_path: &Path) -> i32 {
    let directory_tree: HashMap<String, Directory> = build_directory_tree(input_path);

    let file_system_size: i32 = 70000000;
    let space_needed_for_update: i32 = 30000000;
    let file_system_usage: i32 =
        sum_directory(&directory_tree.get("/").unwrap().clone(), &directory_tree);
    let available_space = file_system_size - file_system_usage;
    let minimum_space_to_delete = space_needed_for_update - available_space;

    directory_tree
        .iter()
        .map(|(_, d)| sum_directory(&d, &directory_tree))
        .collect::<Vec<i32>>()
        .into_iter()
        .filter(|s| s > &minimum_space_to_delete)
        .collect::<Vec<i32>>()
        .into_iter()
        .min()
        .unwrap()
}

fn sum_directory(directory: &Directory, directory_tree: &HashMap<String, Directory>) -> i32 {
    let mut sum: i32 = directory
        .files
        .clone()
        .iter()
        .map(|f| f.size)
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    for child_directory_name in &directory.children {
        sum = sum
            + sum_directory(
                &directory_tree
                    .get(&child_directory_name.clone())
                    .unwrap()
                    .clone(),
                directory_tree,
            );
    }

    sum
}

fn build_directory_tree(input_path: &Path) -> HashMap<String, Directory> {
    let cd_regex = Regex::new(r"^\$\s+cd\s+(?P<dir>.+)").unwrap();
    let dir_regex = Regex::new(r"^dir\s+(?P<dir>.+)").unwrap();
    let file_regex = Regex::new(r"^(?P<file_size>\d+)\s+(?P<file_name>.+)").unwrap();

    let root_directory_name: String = String::from("/");
    let root_directory: Directory = Directory::new(root_directory_name.clone(), None);

    let mut present_working_directory_path: String = root_directory_name.clone();
    let mut present_working_directory: Directory = root_directory.clone();

    let mut directory_tree: HashMap<String, Directory> = HashMap::new();
    directory_tree.insert(root_directory.name.clone(), root_directory.clone());

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(result) = line {
                match cd_regex.captures(&result) {
                    Some(capture) => {
                        let dir = String::from(capture.name("dir").unwrap().as_str());
                        if dir == ".." {
                            if let Some(parent_directory_path) = present_working_directory.parent {
                                present_working_directory =
                                    directory_tree.get(&parent_directory_path).unwrap().clone();
                            }
                        } else {
                            let mut path = Vec::from([
                                present_working_directory.name.clone(),
                                "/".to_string(),
                                dir.clone(),
                            ]);
                            path.dedup();
                            present_working_directory_path = path.join("").clone();
                            present_working_directory =
                                directory_tree.get(&path.join("").clone()).unwrap().clone();
                        }
                    }
                    None => {}
                };
                match dir_regex.captures(&result) {
                    Some(capture) => {
                        let dir = String::from(capture.name("dir").unwrap().as_str());
                        let mut path = Vec::from([
                            present_working_directory.name.clone(),
                            "/".to_string(),
                            dir.clone(),
                        ]);
                        path.dedup();
                        let new_directory: Directory = Directory::new(
                            path.join("").clone(),
                            Some(present_working_directory.name.clone()),
                        );
                        directory_tree.insert(path.join("").clone(), new_directory.clone());
                        directory_tree
                            .entry(present_working_directory_path.clone())
                            .and_modify(|d| d.children.push(new_directory.name.clone()));
                    }
                    None => {}
                };
                match file_regex.captures(&result) {
                    Some(capture) => {
                        let file_name: String =
                            String::from(capture.name("file_name").unwrap().as_str());
                        let file_size: i32 =
                            capture.name("file_size").unwrap().as_str().parse().unwrap();
                        directory_tree
                            .entry(present_working_directory_path.clone())
                            .and_modify(|d| d.files.push(File::new(file_name, file_size)));
                    }
                    None => {}
                };
            }
        }
    }

    directory_tree
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    parent: Option<String>,
    files: Vec<File>,
    children: Vec<String>,
}

impl Directory {
    pub fn new(name: String, parent: Option<String>) -> Directory {
        Directory {
            name,
            parent,
            files: Vec::new(),
            children: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: i32,
}

impl File {
    pub fn new(name: String, size: i32) -> File {
        File { name, size }
    }
}
