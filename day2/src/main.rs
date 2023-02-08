use self::Action::{Paper, Rock, Scissors};
use self::Outcome::{Draw, Loss, Win};
use std::fs::File;
use std::io::{self, BufRead};

enum Action {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

fn simulate_game(opponent: &Action, me: &Action) -> Outcome {
    match (opponent, me) {
        (Rock, Paper) => Win,
        (Rock, Scissors) => Loss,
        (Paper, Rock) => Loss,
        (Paper, Scissors) => Win,
        (Scissors, Rock) => Win,
        (Scissors, Paper) => Loss,
        (_, _) => Draw,
    }
}

fn action_from_string(string: &str) -> Action {
    match string.to_ascii_lowercase().as_ref() {
        "a" | "x" => Rock,
        "b" | "y" => Paper,
        "c" | "z" => Scissors,
        other => panic!("Got unexpected character: {}", other),
    }
}

fn outcome_from_string(letter: &str) -> Outcome {
    match letter.to_ascii_lowercase().as_ref() {
        "x" => Loss,
        "y" => Draw,
        "z" => Win,
        other => panic!("Got unexpected character: {}", other),
    }
}

fn points_from_action(action: Action) -> u32 {
    match action {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

fn points_from_outcome(outcome: &Outcome) -> u32 {
    match outcome {
        Win => 6,
        Draw => 3,
        Loss => 0,
    }
}

fn action_from_outcome(opponent_action: Action, outcome: &Outcome) -> Action {
    match (opponent_action, outcome) {
        (Rock, Win) => Paper,
        (Rock, Loss) => Scissors,
        (Paper, Win) => Scissors,
        (Paper, Loss) => Rock,
        (Scissors, Win) => Rock,
        (Scissors, Loss) => Paper,
        (action, Draw) => action,
    }
}

fn part_one() {
    let file_path = "data.txt";
    let file = File::open(file_path).expect("File not found");
    let strategies = io::BufReader::new(file).lines();

    let score: u32 = strategies
        .map(|game| {
            let strategy_string = game.unwrap().to_ascii_lowercase();
            let (left, right) = strategy_string.split_once(' ').unwrap();
            let opponent_action = action_from_string(left);
            let my_action = action_from_string(right);
            let outcome = simulate_game(&opponent_action, &my_action);

            points_from_action(my_action) + points_from_outcome(&outcome)
        })
        .sum();

    assert_eq!(score, 10624);
    println!("Part 1: {:?}", score);
}

fn part_two() {
    let file_path = "data.txt";
    let file = File::open(file_path).expect("File not found");
    let strategies = io::BufReader::new(file).lines();

    let score: u32 = strategies
        .map(|game| {
            let strategy_string = game.unwrap().to_ascii_lowercase();
            let (left, right) = strategy_string.split_once(' ').unwrap();
            let opponent_action = action_from_string(left);
            let outcome = outcome_from_string(right);
            let action = action_from_outcome(opponent_action, &outcome);

            points_from_action(action) + points_from_outcome(&outcome)
        })
        .sum();

    assert_eq!(score, 14060);
    println!("Part 2: {:?}", score);
}

fn main() {
    part_one();
    part_two();
}
