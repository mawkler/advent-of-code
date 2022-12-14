use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_visible_trees_from_left(treeline: &Vec<char>) -> u32 {
    let treeline = treeline.iter().map(|tree| tree.to_digit(10));

    let (_, total) = treeline.fold((-1 as i32, 0), |(tallest_height, visible_count), tree| {
        if tree.unwrap() as i32 > tallest_height {
            return (tree.unwrap() as i32, visible_count + 1);
        }
        else {
            return (tallest_height, visible_count);
        }
    });

    return total as u32;
}

fn count_visible_trees(treeline: &Vec<char>) -> u32 {
    let left_to_right = count_visible_trees_from_left(treeline);
    let reversed_treeline: Vec<char> = treeline
        .iter()
        .rev()
        .collect::<String>()
        .chars()
        .collect();
    let right_to_left = count_visible_trees_from_left(&reversed_treeline);
    println!("left_to_right: {:?}", left_to_right);
    println!("right_to_left: {:?}", right_to_left);
    return left_to_right + right_to_left;
}

// Borrowed from: https://users.rust-lang.org/t/rayon-transpose-of-vec-vec-t/62864
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn part_one() {
    let file_path = "data.txt";
    let file = File::open(file_path).expect("File not found");
    let forest = BufReader::new(file).lines();

    let forest: Vec<Vec<char>> = forest
        .map(|treeline| treeline.unwrap().chars().collect())
        .collect();

    let horizontal_count: u32 = forest.iter().map(count_visible_trees).sum();
    println!("horizontal_count: {:?}", horizontal_count);
    let vertical_count: u32 = transpose(forest)
        .iter()
        .map(count_visible_trees)
        .sum();
    println!("vertical_count: {:?}", vertical_count);

    println!("Part 1: {:?}", horizontal_count + vertical_count);
}

fn main() {
    part_one();
}
