use itertools::Itertools;
use std::iter;

fn predict_next_value(triangle: Vec<Vec<i32>>) -> i32 {
    triangle
        .iter()
        .rev()
        .fold(0, |diff, numbers| numbers.last().unwrap() + diff)
}

fn predict_previous_value(triangle: Vec<Vec<i32>>) -> i32 {
    triangle
        .iter()
        .rev()
        .fold(0, |diff, numbers| numbers.first().unwrap() - diff)
}

fn build_triangle(numbers: Vec<i32>) -> Vec<Vec<i32>> {
    iter::successors(Some(numbers), |num| {
        if num.iter().all(|&num| num == 0) {
            None
        } else {
            Some(get_differences(num))
        }
    })
    .collect()
}

fn get_differences(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .tuple_windows()
        .map(|(prev, next)| next - prev)
        .collect()
}

fn parse_lines(lines: &str) -> Vec<Vec<i32>> {
    lines
        .lines()
        .map(|line| line.split(' ').map(|num| num.parse().unwrap()).collect())
        .collect()
}

fn main() {
    let data = include_str!("../../data/day9");
    let lines = parse_lines(data);

    let sum: i32 = lines
        .clone()
        .into_iter()
        .map(build_triangle)
        .map(predict_next_value)
        .sum();

    println!("Part 1: {}", sum);

    let sum: i32 = lines
        .into_iter()
        .map(build_triangle)
        .map(predict_previous_value)
        .sum();

    println!("Part 2: {}", sum);
}
