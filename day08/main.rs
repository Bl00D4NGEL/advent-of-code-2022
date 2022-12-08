use std::usize;

#[derive(Debug)]
struct Forest {
    trees: Vec<u32>,
    length: usize,
}

#[derive(Debug)]
struct TreePosition {
    column: usize,
    row: usize,
}

impl TreePosition {
    pub fn new(column: usize, row: usize) -> TreePosition {
        TreePosition { column, row }
    }
}

// The implementation assumes that the forest is a square which means that height = width
impl Forest {
    pub fn new(trees: Vec<u32>, length: usize) -> Forest {
        Forest { trees, length }
    }

    pub fn get_trees_above(&self, index: usize) -> Vec<&u32> {
        let tree_position = self.get_tree_position(index);
        let mut trees = vec![];
        for i in 0..tree_position.row {
            trees.push(self.get_tree_at_position(TreePosition::new(tree_position.column, i)));
        }

        // We need to reverse the list of trees because we want to keep the "looking" order of the trees
        trees.reverse();

        trees
    }

    pub fn get_trees_below(&self, index: usize) -> Vec<&u32> {
        let tree_position = self.get_tree_position(index);
        let mut trees = vec![];
        // Current row + 1 because we want the rows below the current tree and not include the row of the current tree
        for i in (tree_position.row + 1)..self.length {
            trees.push(self.get_tree_at_position(TreePosition::new(tree_position.column, i)));
        }

        trees
    }

    pub fn get_trees_to_the_right_of(&self, index: usize) -> Vec<&u32> {
        let tree_position = self.get_tree_position(index);
        let mut trees = vec![];
        // Current column + 1 because we want the columns to the right of the current tree and not include the column of the current tree
        for i in (tree_position.column + 1)..self.length {
            trees.push(self.get_tree_at_position(TreePosition::new(i, tree_position.row)));
        }

        trees
    }

    pub fn get_trees_to_the_left_of(&self, index: usize) -> Vec<&u32> {
        let tree_position = self.get_tree_position(index);
        let mut trees = vec![];
        for i in 0..tree_position.column {
            trees.push(self.get_tree_at_position(TreePosition::new(i, tree_position.row)));
        }

        // We need to reverse the list of trees because we want to keep the "looking" order of the trees
        trees.reverse();

        trees
    }

    fn get_tree_at_position(&self, position: TreePosition) -> &u32 {
        let tree = self.trees.get(position.row * self.length + position.column);
        match tree {
            None => panic!("Could not find tree at position {:?}", position),
            Some(v) => v,
        }
    }

    fn get_tree_at_index(&self, index: usize) -> &u32 {
        let tree = self.trees.get(index);
        match tree {
            None => panic!("Could not find tree at index {:?}", index),
            Some(v) => v,
        }
    }

    fn get_tree_position(&self, tree_index: usize) -> TreePosition {
        // Given the grid is a square we can calculate the tree position based on the index and the length of the forest
        // If the forest is of length 5 we can assume the following (0 based indexes):
        // If index = 3 then the tree is in row 0 column 3
        // If index = 5 then the tree is in row 1 column 0
        // If index = 15 then the tree is in row 3 column 0
        // If index = 24 then the tree is in row 4 column 4

        let column = tree_index % self.length;
        let row = tree_index / self.length;

        TreePosition::new(column, row)
    }
}

fn main() {
    let contents = include_str!("./input.txt");

    let trees = contents
        .split("\n")
        .map(|line| line.chars().into_iter().map(|c| c.to_digit(10).unwrap()))
        .flatten()
        .collect::<Vec<u32>>();
    let forest = Forest::new(trees, contents.split("\n").count());

    println!(
        "Day 8 part 1: {}",
        forest
            .trees
            .iter()
            .enumerate()
            .filter(|(index, _)| is_tree_visible(&forest, *index))
            .count()
    );

    let mut scenic_secores = forest
        .trees
        .iter()
        .enumerate()
        .map(|(index, _)| get_scenic_score_for_tree(&forest, index))
        .collect::<Vec<u32>>();

    scenic_secores.sort();
    println!("Day 8 part 2: {:?}", scenic_secores.pop());
}

fn get_scenic_score_for_tree(forest: &Forest, tree_index: usize) -> u32 {
    let tree_heigth = forest.get_tree_at_index(tree_index);

    let scenic_scores = vec![
        get_scenic_score_for_allotment(tree_heigth, &forest.get_trees_above(tree_index)),
        get_scenic_score_for_allotment(tree_heigth, &forest.get_trees_to_the_left_of(tree_index)),
        get_scenic_score_for_allotment(tree_heigth, &forest.get_trees_below(tree_index)),
        get_scenic_score_for_allotment(tree_heigth, &forest.get_trees_to_the_right_of(tree_index)),
    ];

    match scenic_scores.into_iter().reduce(|acc, score| acc * score) {
        None => 0,
        Some(v) => v,
    }
}

fn get_scenic_score_for_allotment(tree_heigth: &u32, other_trees: &Vec<&u32>) -> u32 {
    let mut scenic_score = 0;
    for tree in other_trees {
        scenic_score = scenic_score + 1;

        if *tree >= tree_heigth {
            break;
        }
    }

    scenic_score
}

fn is_tree_visible(forest: &Forest, tree_index: usize) -> bool {
    let tree_heigth = forest.get_tree_at_index(tree_index);
    if is_tree_visible_in_allotment(tree_heigth, &mut forest.get_trees_above(tree_index)) {
        return true;
    }

    if is_tree_visible_in_allotment(tree_heigth, &mut forest.get_trees_below(tree_index)) {
        return true;
    }

    if is_tree_visible_in_allotment(
        tree_heigth,
        &mut forest.get_trees_to_the_left_of(tree_index),
    ) {
        return true;
    }

    is_tree_visible_in_allotment(
        tree_heigth,
        &mut &mut forest.get_trees_to_the_right_of(tree_index),
    )
}

fn is_tree_visible_in_allotment(tree_heigth: &u32, other_tree_heigths: &mut Vec<&u32>) -> bool {
    other_tree_heigths.sort();

    match other_tree_heigths.pop() {
        None => true,
        Some(heighest_tree) => heighest_tree < tree_heigth,
    }
}
