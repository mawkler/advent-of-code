use std::collections::HashMap;

const DATA: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

fn get_first_digit(string: &str) -> Option<(char, usize)> {
    let position = string.find(|c: char| c.is_ascii_digit())?;
    let char = string.chars().nth(position)?;
    Some((char, position))
}

fn get_literal_to_digit_mapping() -> HashMap<&'static str, char> {
    HashMap::from([
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

/// Returns the leftmost literal digit and its index
fn get_first_literal_digit(string: &str) -> Option<(&str, usize)> {
    // TODO: has to be reversed when we're searching backwards
    get_literal_to_digit_mapping()
        .keys()
        .filter_map(|&literal| string.find(literal).map(|i| (literal, i)))
        .min_by_key(|&(_, i)| i)
}

fn digit_from_literal(string: &str) -> Option<char> {
    get_literal_to_digit_mapping().get(string).cloned()
}

fn get_leftmost_pos<T>(d1: (T, usize), d2: (T, usize)) -> (T, usize) {
    std::cmp::min_by_key(d1, d2, |&(_, i)| i)
}

fn get_first_digit_or_literal(string: &str) -> Option<char> {
    let digit = get_first_digit(string);
    let literal =
        get_first_literal_digit(string).and_then(|l| Some((digit_from_literal(l.0)?, l.1)));

    digit
        .or(literal)
        .and_then(|d| literal.map(|l| get_leftmost_pos(l, d)))
        .or(digit)
        .map(|(leftmost, _)| leftmost)
}

fn get_first_and_last_digit(string: &str) -> (char, char) {
    let first = get_first_digit_or_literal(string).expect("Must exist");
    let reversed_string: String = string.chars().rev().collect();
    let last = get_first_digit_or_literal(&reversed_string).expect("Must exist");

    (first, last)
}

fn extract_number_from_line(string: &str) -> u32 {
    let (first, last) = get_first_and_last_digit(string);
    format!("{}{}", first, last).parse().unwrap()
}

fn main() {
    let data = include_str!("../../data/day1");
    let sum: u32 = data.lines().map(extract_number_from_line).sum();
    println!("Day 1.1: {:#?}", sum);

    let res: Vec<_> = DATA
        .lines()
        .map(|l| {
            println!("l = {:#?}", l);
            extract_number_from_line(l)
        })
        .collect();
    println!("res = {:#?}", res);

    // let result = get_first_literal_digit(DATA.lines().next().unwrap());
    // let result = get_first_and_last_digit("two4");
    // println!("result = {:#?}", result);
}

#[test]
fn gets_first_and_last() {
    let string = "twoooo4";
    assert_eq!(get_first_and_last_digit(string), ('2', '4'))
}

#[test]
fn gets_first_digit() {
    let string = "f4wheew";
    assert_eq!(get_first_digit_or_literal(string), Some('4'));
}

#[test]
fn gets_first_literal() {
    let string = "eightwothree";
    assert_eq!(get_first_digit_or_literal(string), Some('8'));
}
