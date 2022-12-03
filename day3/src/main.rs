use std::fs::File;
use std::io::{self, BufRead, Error};

fn priority_from_char(char: char) -> u32 {
    if char.is_lowercase() {
        char as u32 - 96
    } else {
        (char as u32 - 64) + 26
    }
}

type Rucksack = Result<String, Error>;

fn common_item(r1: &Rucksack, r2: &Rucksack, r3: &Rucksack) -> char {
    let (r1, r2, r3) = (r1.as_ref().unwrap(), r2.as_ref().unwrap(), r3.as_ref().unwrap());
    r1.chars().find(|item| r2.contains(*item) && r3.contains(*item)).unwrap()
}

fn part_one() {
    let file_path = "data.txt";
    let file = File::open(file_path).expect("File not found");
    let rucksacks = io::BufReader::new(file).lines();

    let sum: u32 = rucksacks.map(|rucksack| {
        let half = rucksack.as_ref().unwrap().len() / 2;
        let (left, right) = rucksack.as_ref().unwrap().split_at(half);

        let item = left.chars().find(|item| right.contains(*item)).unwrap();
        priority_from_char(item)
    }).sum();

    println!("Part 1: {:?}", sum);
}

fn part_two() {
    let file_path = "data.txt";
    let file = File::open(file_path).expect("File not found");
    let rucksacks: Vec<Result<String, io::Error>> = io::BufReader::new(file).lines().collect();

    let sum: u32 = rucksacks.chunks(3).map(|rucksack_group| {
        let item = common_item(&rucksack_group[0], &rucksack_group[1], &rucksack_group[2]);
        priority_from_char(item)
    }).sum();

    println!("Part 2: {:?}", sum);
}

fn main() {
    part_one();
    part_two();
}
