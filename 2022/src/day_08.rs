use advent_of_code::read_lines;
use std::collections::{HashMap, VecDeque};
use std::path::Path;

pub fn part_one(input_path: &Path) -> i32 {
    let input: Input = Input::new(input_path);

    input
        .trees
        .iter()
        .filter(|t| t.is_visible(input.rows - 1, input.columns - 1))
        .count()
        .try_into()
        .unwrap()
}

pub fn part_two(input_path: &Path) -> i32 {
    let input: Input = Input::new(input_path);

    input.trees.iter().map(|t| t.scenic_score()).max().unwrap()
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct TreeCoordinate {
    row: isize,
    column: isize,
}

impl TreeCoordinate {
    pub fn new(row: isize, column: isize) -> TreeCoordinate {
        TreeCoordinate { row, column }
    }
}

#[derive(Debug, Clone, Copy)]
struct Tree {
    height: u32,
    coordinates: TreeCoordinate,
}

impl Tree {
    pub fn new(height: u32, coordinates: TreeCoordinate) -> Tree {
        Tree {
            height,
            coordinates,
        }
    }

    pub fn is_edge_tree(&self, edge_row: isize, edge_column: isize) -> bool {
        self.coordinates.row == 0
            || self.coordinates.column == 0
            || self.coordinates.row == edge_row
            || self.coordinates.column == edge_column
    }

    pub fn is_visible(
        &self,
        edge_row: isize,
        edge_column: isize,
        neighbors: &TreeNeighbors,
    ) -> bool {
        self.is_edge_tree(edge_row, edge_column) || neighbors.is_tree_visible(&self)
    }
}

#[derive(Debug)]
struct TreeNeighbors {
    upper_neighbors: VecDeque<Tree>,
    lower_neighbors: VecDeque<Tree>,
    right_neighbors: VecDeque<Tree>,
    left_neighbors: VecDeque<Tree>,
}

impl TreeNeighbors {
    pub fn new() -> TreeNeighbors {
        TreeNeighbors {
            upper_neighbors: VecDeque::new(),
            lower_neighbors: VecDeque::new(),
            right_neighbors: VecDeque::new(),
            left_neighbors: VecDeque::new(),
        }
    }

    pub fn add_upper_neighbor(&mut self, neighbor: Tree) {
        let _ = &self.upper_neighbors.push_front(neighbor);
    }

    pub fn add_lower_neighbor(&mut self, neighbor: Tree) {
        let _ = &self.lower_neighbors.push_back(neighbor);
    }

    pub fn add_right_neighbor(&mut self, neighbor: Tree) {
        let _ = &self.right_neighbors.push_back(neighbor);
    }

    pub fn add_left_neighbor(&mut self, neighbor: Tree) {
        let _ = &self.left_neighbors.push_front(neighbor);
    }

    pub fn is_tree_visible(&self, tree: &Tree) -> bool {
        self.visible_based_on_neighbors(tree, &self.upper_neighbors)
            || self.visible_based_on_neighbors(tree, &self.lower_neighbors)
            || self.visible_based_on_neighbors(tree, &self.left_neighbors)
            || self.visible_based_on_neighbors(tree, &self.right_neighbors)
    }

    fn visible_based_on_neighbors(&self, tree: &Tree, neighbors: &VecDeque<Tree>) -> bool {
        neighbors.iter().filter(|t| t.height >= tree.height).count() == 0
    }

    pub fn scenic_score(&self, tree: &Tree) -> i32 {
        self.viewing_distance_based_on_neighbors(tree, &self.upper_neighbors)
            * self.viewing_distance_based_on_neighbors(tree, &self.lower_neighbors)
            * self.viewing_distance_based_on_neighbors(tree, &self.right_neighbors)
            * self.viewing_distance_based_on_neighbors(tree, &self.left_neighbors)
    }

    fn viewing_distance_based_on_neighbors(&self, tree: &Tree, neighbors: &VecDeque<Tree>) -> i32 {
        let mut viewing_distance: i32 = 0;

        for neighbor in neighbors {
            viewing_distance = viewing_distance + 1;
            if neighbor.height >= tree.height {
                break;
            }
        }

        viewing_distance
    }
}

#[derive(Debug)]
struct TreeWithNeighbors {
    tree: Tree,
    neighbors: TreeNeighbors,
}

impl TreeWithNeighbors {
    pub fn new(tree: Tree, neighbors: TreeNeighbors) -> TreeWithNeighbors {
        TreeWithNeighbors { tree, neighbors }
    }

    pub fn is_visible(&self, edge_row: isize, edge_column: isize) -> bool {
        self.tree.is_visible(edge_row, edge_column, &self.neighbors)
    }

    pub fn scenic_score(&self) -> i32 {
        self.neighbors.scenic_score(&self.tree)
    }
}

struct Input {
    trees: Vec<TreeWithNeighbors>,
    rows: isize,
    columns: isize,
}

impl Input {
    pub fn new(input_path: &Path) -> Input {
        let mut row_index: isize = 0;
        let mut column_index: isize = 0;
        let mut temporary_map: HashMap<TreeCoordinate, Tree> = HashMap::new();
        let mut trees: Vec<TreeWithNeighbors> = Vec::new();

        if let Ok(lines) = read_lines(input_path) {
            for line in lines {
                column_index = 0;

                if let Ok(result) = line {
                    for char in result.chars() {
                        let height: u32 = char.to_digit(10).unwrap();
                        let coordinates: TreeCoordinate =
                            TreeCoordinate::new(row_index.into(), column_index.into());
                        temporary_map
                            .insert(coordinates.clone(), Tree::new(height, coordinates.clone()));
                        column_index = column_index + 1;
                    }
                }

                row_index = row_index + 1;
            }
        }

        let rows: isize = row_index;
        let mut columns: isize = column_index;

        for row in 0..rows {
            for column in 0..columns {
                let current_coordinate: TreeCoordinate = TreeCoordinate::new(row, column);
                let current_tree: &Tree = temporary_map.get(&current_coordinate).unwrap();
                let mut current_neighbors: TreeNeighbors = TreeNeighbors::new();

                // I need every neighbor in my column
                for inner_row in 0..rows {
                    if inner_row == row {
                        continue;
                    }

                    let neighbor_coordinate: TreeCoordinate =
                        TreeCoordinate::new(inner_row, column);
                    if let Some(neighbor) = temporary_map.get(&neighbor_coordinate) {
                        if inner_row < row {
                            current_neighbors.add_upper_neighbor(neighbor.clone());
                        } else if inner_row > row {
                            current_neighbors.add_lower_neighbor(neighbor.clone());
                        }
                    }
                }
                // And every neighbor in my row
                for inner_column in 0..columns {
                    if inner_column == column {
                        continue;
                    }

                    let neighbor_coordinate: TreeCoordinate =
                        TreeCoordinate::new(row, inner_column);

                    if let Some(neighbor) = temporary_map.get(&neighbor_coordinate) {
                        if inner_column < column {
                            current_neighbors.add_left_neighbor(neighbor.clone());
                        } else if inner_column > column {
                            current_neighbors.add_right_neighbor(neighbor.clone());
                        }
                    }
                }

                trees.push(TreeWithNeighbors::new(
                    current_tree.clone(),
                    current_neighbors,
                ))
            }

            columns = column_index;
        }

        Input {
            rows: row_index,
            columns: column_index,
            trees,
        }
    }
}
