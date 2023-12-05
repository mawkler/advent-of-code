use std::collections::HashMap;

fn get_literal_to_digit_mapping() -> HashMap<&'static str, char> {
    HashMap::from([
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ])
}

fn get_digits_from_string(string: &str) -> Vec<char> {
    (0..string.len())
        .flat_map(|offset| {
            string.get(offset..).and_then(|substring| {
                get_literal_to_digit_mapping()
                    .into_iter()
                    .find(|(literal, _)| substring.starts_with(literal))
                    .map(|(_, digit)| digit)
            })
        })
        .collect()
}

fn get_first_digit(string: &str) -> Option<char> {
    get_digits_from_string(string).into_iter().next()
}

fn get_last_digit(string: &str) -> Option<char> {
    get_digits_from_string(string).into_iter().last()
}

fn get_first_and_last_digit(string: &str) -> (char, char) {
    let first = get_first_digit(string).expect("Must exist");
    let last = get_last_digit(string).expect("Must exist");

    (first, last)
}

fn extract_number_from_line(string: &str) -> u32 {
    let (first, last) = get_first_and_last_digit(string);
    format!("{}{}", first, last).parse().unwrap()
}

fn main() {
    let data = include_str!("../../data/day1");
    let sum: u32 = data.lines().map(extract_number_from_line).sum();

    println!("Day 1.2: {:#?}", sum);
}

#[test]
fn find_first_digit() {
    assert_eq!(get_first_digit("two1nine"), Some('2'));
    assert_eq!(get_first_digit("eightwothree"), Some('8'));
    assert_eq!(get_first_digit("abcone2threexyz"), Some('1'));
    assert_eq!(get_first_digit("xtwone3four"), Some('2'));
    assert_eq!(get_first_digit("4nineeightseven2"), Some('4'));
    assert_eq!(get_first_digit("zoneight234"), Some('1'));
    assert_eq!(get_first_digit("7pqrstsixteen"), Some('7'));
}

#[test]
fn find_last_digit() {
    assert_eq!(get_last_digit("two1nine"), Some('9'));
    assert_eq!(get_last_digit("eightwothree"), Some('3'));
    assert_eq!(get_last_digit("abcone2threexyz"), Some('3'));
    assert_eq!(get_last_digit("xtwone3four"), Some('4'));
    assert_eq!(get_last_digit("4nineeightseven2"), Some('2'));
    assert_eq!(get_last_digit("zoneight234"), Some('4'));
    assert_eq!(get_last_digit("7pqrstsixteen"), Some('6'));
}

#[test]
fn find_number() {
    assert_eq!(extract_number_from_line("two1nine"), 29);
    assert_eq!(extract_number_from_line("eightwothree"), 83);
    assert_eq!(extract_number_from_line("abcone2threexyz"), 13);
    assert_eq!(extract_number_from_line("xtwone3four"), 24);
    assert_eq!(extract_number_from_line("4nineeightseven2"), 42);
    assert_eq!(extract_number_from_line("zoneight234"), 14);
    assert_eq!(extract_number_from_line("7pqrstsixteen"), 76);
}
