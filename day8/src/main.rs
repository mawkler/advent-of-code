use std::fs::File;
use std::io::{BufRead, BufReader};

struct Coordinate { y: usize, x: usize, }

type Tree = u32;
type Forest = Vec<Vec<Tree>>;

fn is_tallest(height: u32, treeline: Vec<Tree>) -> bool {
    treeline.iter().all(|tree| tree < &height )
}

/// Splits `treeline` at tree at `position`, and excludes the tree at the split
/// `position`
fn split_treeline(treeline: &Vec<Tree>, position: usize) -> (Vec<Tree>, Vec<Tree>) {
    let (left, right_with_tree) = treeline.split_at(position);
    let (_, right) = right_with_tree.split_first().unwrap();
    return (left.to_vec(), right.to_vec());
}

fn is_horizontally_visible(tree: &Coordinate, forest: &Forest) -> bool {
    let (left, right) = split_treeline(&forest[tree.y], tree.x);
    let height = forest[tree.y][tree.x];

    return is_tallest(height, right.to_vec())
        || is_tallest(height, left.to_vec());
}

fn is_vertically_visible(tree: &Coordinate, forest: &Forest) -> bool {
    let treeline: Vec<u32> = forest
        .iter()
        .map(|treeline| treeline[tree.x])
        .collect();
    let (top, bottom) = split_treeline(&treeline, tree.y);
    let height = forest[tree.y][tree.x];

    return is_tallest(height, top.to_vec())
        || is_tallest(height, bottom.to_vec());
}

fn is_visible(tree: &Coordinate, forest: &Forest) -> bool {
    return is_horizontally_visible(&tree, forest)
        || is_vertically_visible(&tree, forest)
}

fn part_one() {
    let file_path = "data.txt";
    let file = File::open(file_path).expect("File not found");
    let forest = BufReader::new(file).lines();

    let forest: Forest = forest
        .map(|treeline| treeline
            .unwrap()
            .chars()
            .map(|tree| tree.to_digit(10).unwrap())
            .collect()
        )
        .collect();

    let visible_trees: Vec<bool> = forest.to_owned().iter().enumerate().flat_map(|(y, treeline)| {
        treeline.iter().enumerate().map(|(x, _)| {
            is_visible(&Coordinate {x, y}, forest.as_ref())
        }).collect::<Vec<bool>>()
    }).collect();

    let sum = visible_trees
        .into_iter()
        .filter(|tree| *tree)
        .into_iter().count();

    println!("sum: {:?}", sum);
}

fn main() {
    part_one();
}
