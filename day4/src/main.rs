use std::fs::File;
use std::io::{self, BufRead};

fn split_in_two<'a>(string: &'a str, separator: &str) -> (&'a str, &'a str) {
    let strings: Vec<&str> = string.split(separator).collect();
    (strings[0], strings[1])
}

struct Range {
    left: u32,
    right: u32,
}

impl Range {
    pub fn from_string(range: &str) -> Range {
        let (left, right) = split_in_two(range, "-");

        Range {
            left: left.parse().unwrap(),
            right: right.parse().unwrap()
        }
    }

    fn overlap(self, other: Range) -> bool {
        self.left <= other.left && self.right >= other.right ||
            self.left >= other.left && self.right <= other.right
    }
}

fn part_one() {
    let file_path = "data.txt";
    let file = File::open(file_path).expect("File not found");
    let pairs = io::BufReader::new(file).lines();

    let overlaps = pairs.map(|pair| {
        let pair = pair.unwrap();
        let (left_range, right_range) = split_in_two(&pair, ",");
        let left = Range::from_string(left_range);
        let right = Range::from_string(right_range);
        right.overlap(left)
    }).filter(|overlap| *overlap).count();

    println!("Part 1: {:?}", overlaps);
}

fn part_two() {
    let file_path = "data.txt";
    let file = File::open(file_path).expect("File not found");
    let pairs = io::BufReader::new(file).lines();
}

fn main() {
    part_one();
    part_two();
}
