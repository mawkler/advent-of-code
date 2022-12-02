use std::fs::File;
use std::io::{self, BufRead};
use self::Outcome::{Win, Loss, Draw};
use self::Action::{Rock, Paper, Scissors};

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

fn game_win(opponent: &Action, me: &Action) -> Outcome {
    match (opponent, me) {
        (Rock, Paper) => Win,
        (Rock, Scissors) => Loss,
        (Paper, Rock) => Loss,
        (Paper, Scissors) => Win,
        (Scissors, Rock) => Win,
        (Scissors, Paper) => Loss,
        (_, _) => Draw
    }
}

fn action_from_letter(letter: char) -> Action {
    match letter.to_ascii_lowercase() {
        'a' | 'x' => Rock,
        'b' | 'y' => Paper,
        'c' | 'z' => Scissors,
        other => panic!("Got unexpected character: {}", other),
    }
}

fn outcome_from_letter(letter: char) -> Outcome {
    match letter.to_ascii_lowercase() {
        'x' => Loss,
        'y' => Draw,
        'z' => Win,
        other => panic!("Got unexpected character: {}", other),
    }
}

fn points_from_action(action: Action) -> u32 {
    match action {
        Rock => 1,
        Paper => 2,
        Scissors => 3
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
    let strategy = io::BufReader::new(file).lines();

    let score: u32 = strategy.map(|game|{
        let strategy_string = game.unwrap().to_ascii_lowercase();
        let (opponent_action, my_action) = (
            action_from_letter(strategy_string.chars().nth(0).unwrap()),
            action_from_letter(strategy_string.chars().nth(2).unwrap())
        );
        let outcome = game_win(&opponent_action, &my_action);
        points_from_action(my_action) + points_from_outcome(&outcome)
    }).sum();

    println!("Part 1: {:?}", score);
}

fn part_two() {
    let file_path = "data.txt";
    let file = File::open(file_path).expect("File not found");
    let strategy = io::BufReader::new(file).lines();

    let score: u32 = strategy.map(|game|{
        let strategy_string = game.unwrap().to_ascii_lowercase();
        let opponent_action = action_from_letter(strategy_string.chars().nth(0).unwrap());
        let outcome = outcome_from_letter(strategy_string.chars().nth(2).unwrap());
        let action = action_from_outcome(opponent_action, &outcome);

        points_from_action(action) + points_from_outcome(&outcome)
    }).sum();

    println!("Part 2: {:?}", score);
}

fn main() {
    part_one();
    part_two();
}
