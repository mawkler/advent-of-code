use std::fs;

fn has_duplicate(signal: &Vec<char>) -> bool {
    let mut signal = signal.to_vec();
    signal.sort();
    signal.windows(2).any(|pair| pair[0] == pair[1])
}

fn part_one() {
    let  file_path = "data.txt";
    let signal = fs::read_to_string(file_path).expect("File not found");

    let marker_pos = signal
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .enumerate()
        .find_map(|(i, window)| {
            if !has_duplicate(&window.to_vec()) {
                println!("window: {:?}", window);
                Some(i + window.len())
            } else { None }
        })
        .unwrap();

    println!("Part 1: {:?}", marker_pos);
}

fn main() {
    part_one()
}
