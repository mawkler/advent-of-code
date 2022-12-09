use std::fs;

fn has_duplicate(signal: &Vec<char>) -> bool {
    let mut signal = signal.to_vec();
    signal.sort();
    signal.windows(2).any(|pair| pair[0] == pair[1])
}

fn get_first_n_uniques_end_pos(signal: &String, n: usize) -> usize {
    signal
        .chars()
        .collect::<Vec<char>>()
        .windows(n)
        .enumerate()
        .find_map(|(i, window)| {
            if !has_duplicate(&window.to_vec()) {
                Some(i + window.len())
            } else {
                None
            }
        })
        .unwrap()
}

fn part_one() {
    let file_path = "data.txt";
    let signal = fs::read_to_string(file_path).expect("File not found");

    let marker_pos = get_first_n_uniques_end_pos(&signal, 4);
    println!("Part 1: {:?}", marker_pos);
}

fn part_two() {
    let file_path = "data.txt";
    let signal = fs::read_to_string(file_path).expect("File not found");

    let marker_pos = get_first_n_uniques_end_pos(&signal, 14);
    println!("Part 2: {:?}", marker_pos);
}

fn main() {
    part_one();
    part_two();
}
