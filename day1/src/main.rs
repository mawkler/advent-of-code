use std::fs;

fn main() {
    let file_path = "data.txt";
    let all_calories = fs::read_to_string(file_path).expect("File not found");
    let inventories = all_calories.split("\n\n");

    let max_calories = inventories.map(|inventory| {
        inventory.split("\n").fold(0, |acc, calory| {
            acc + calory.parse::<i32>().unwrap_or(0)
        })
    }).max().unwrap();

    println!("Max calories: {:?}", max_calories);
}
