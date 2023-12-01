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

fn get_vertical_treeline(forest: &Forest, x: usize) -> Vec<u32> {
    return forest.iter().map(|treeline| treeline[x]).collect();
}

fn is_horizontally_visible(tree: &Coordinate, forest: &Forest) -> bool {
    let (left, right) = split_treeline(&forest[tree.y], tree.x);
    let height = forest[tree.y][tree.x];

    return is_tallest(height, right.to_vec())
        || is_tallest(height, left.to_vec());
}

fn is_vertically_visible(tree: &Coordinate, forest: &Forest) -> bool {
    let treeline = get_vertical_treeline(&forest, tree.x);
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

    let visible_trees: Vec<bool> = forest.iter().enumerate().flat_map(|(y, treeline)|
        treeline.iter().enumerate().map(|(x, _)|
            is_visible(&Coordinate {x, y}, forest.as_ref())
        ).collect::<Vec<bool>>()
    ).collect();

    let sum = visible_trees
        .into_iter()
        .filter(|tree| *tree)
        .into_iter().count();

    println!("Part 1: {:?}", sum);
}

fn count_visible_trees_to_right(treeline: &Vec<Tree>, height: u32) -> usize {
    // This had to be imperative since I couldn't find a nice way to do this with iterators
    let mut visibles = treeline.iter().peekable();
    let mut count = 0;
    while let Some(tree) = visibles.next() {
        count += 1;
        if *tree >= height { break; }
    };
    return count;
}

fn scenic_score(forest: &Forest, tree: &Coordinate) -> usize {
    let height = forest[tree.y][tree.x];
    let (mut left, right) = split_treeline(&forest[tree.y], tree.x);
    let vertical_treeline = get_vertical_treeline(forest, tree.x);
    let (mut top, bottom) = split_treeline(&vertical_treeline, tree.y);
    left.reverse();
    top.reverse();

    [left, right, top, bottom].iter().map(|treeline| {
        count_visible_trees_to_right(treeline, height)
    }).product()
}

fn part_two() {
    let file_path = "data.txt";
    let file = File::open(file_path).expect("File not found");
    let forest = BufReader::new(file).lines();

    let forest: Forest = forest.map(|treeline| treeline
        .unwrap()
        .chars()
        .map(|tree| tree.to_digit(10).unwrap())
        .collect()
    ).collect();

    let highest_scenic_score = forest.iter().enumerate().flat_map(|(y, treeline)|
        treeline.iter().enumerate().map(|(x, _)|
            scenic_score(&forest, &Coordinate {x, y})
        ).collect::<Vec<usize>>()
    ).max().unwrap();

    println!("Part 2: {:?}", highest_scenic_score);
}

fn main() {
    part_one();
    part_two();
}
