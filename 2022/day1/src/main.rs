use std::fs;

fn part_one() {
    let file_path = "data.txt";
    let all_calories = fs::read_to_string(file_path).expect("File not found");
    let inventories = all_calories.split("\n\n");

    let max_calories = inventories.map(|inventory| {
        inventory.split("\n").fold(0, |acc, calory| {
            acc + calory.parse::<u32>().unwrap_or(0)
        })
    }).max().unwrap();

    println!("Part 1: {:?}", max_calories);
}

fn part_two() {
    let file_path = "data.txt";
    let all_calories = fs::read_to_string(file_path).expect("File not found");
    let inventories = all_calories.split("\n\n");

    let mut max_calories: Vec<u32> = inventories.map(|inventory| {
        inventory.split("\n").fold(0, |acc, calory| {
            acc + calory.parse::<u32>().unwrap_or(0)
        })
    }).collect();

    max_calories.sort();
    max_calories.reverse();
    max_calories.truncate(3);

    println!("Part 2: {:?}", max_calories.iter().sum::<u32>());
}


fn main() {
    part_one();
    part_two();
}
