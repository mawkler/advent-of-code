fn get_first_digit(string: &str) -> Option<char> {
    string.chars().find(|c| c.is_ascii_digit())
}

fn get_first_and_last_digit(string: &str) -> (Option<char>, Option<char>) {
    let first = get_first_digit(string);
    let reversed_string: String = string.chars().rev().collect();
    let last = get_first_digit(&reversed_string);

    (first, last)
}

fn extract_number_from_line(string: &str) -> Option<u32> {
    let (first, last) = get_first_and_last_digit(string);
    format!("{}{}", first?, last?).parse().ok()
}

fn main() {
    let data = include_str!("../../data/day1");
    let sum: u32 = data.lines().filter_map(extract_number_from_line).sum();
    println!("Day 1.1: {:#?}", sum);
}
