use self::Color::{Blue, Green, Red};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};
use std::{collections::HashMap, ops::Add};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s {
            "red" => Red,
            "green" => Green,
            "blue" => Blue,
            other => panic!("Unexpected color '{}' found", other),
        }
    }
}

type Count = u16;
type Draw = HashMap<Color, Count>;
type Id = u32;

#[derive(Debug, Default)]
struct Game {
    id: Id,
    draws: Vec<Draw>,
}

impl Add<Draw> for Game {
    type Output = Game;

    fn add(self, draw: Draw) -> Game {
        let mut draws = self.draws.clone();
        draws.push(draw);

        Game { id: self.id, draws }
    }
}

impl From<(Id, Vec<Draw>)> for Game {
    fn from((id, draws): (Id, Vec<Draw>)) -> Self {
        draws
            .into_iter()
            .fold(Game::new(id), |game, draw| game + draw)
    }
}

impl Game {
    fn new(id: Id) -> Self {
        Game { id, draws: vec![] }
    }

    fn max_count(&self, color: Color) -> Count {
        *self
            .draws
            .iter()
            .map(|draw| draw.get(&color).unwrap_or(&0))
            .max()
            .expect("Game should contain draws")
    }
}

fn parse_game_id(i: &str) -> IResult<&str, Id> {
    delimited(tag("Game "), map_res(digit1, str::parse), tag(": "))(i)
}

fn parse_color(i: &str) -> IResult<&str, (Count, Color)> {
    separated_pair(
        map_res(digit1, str::parse),
        tag(" "),
        map(alpha1, Color::from),
    )(i)
}

fn into_draw(colors: Vec<(Count, Color)>) -> Draw {
    colors
        .into_iter()
        .map(|(count, color)| (color, count))
        .collect()
}

fn parse_draw(i: &str) -> IResult<&str, Draw> {
    map(separated_list1(tag(", "), parse_color), into_draw)(i)
}

fn parse_draws(i: &str) -> IResult<&str, Vec<Draw>> {
    separated_list1(tag("; "), parse_draw)(i)
}

fn parse_line(i: &str) -> Game {
    let (_, game) = tuple((parse_game_id, parse_draws))(i).unwrap();

    game.into()
}

fn possible_games(lines: &str) -> Vec<Id> {
    lines
        .lines()
        .map(parse_line)
        .filter(|game| {
            game.max_count(Red) <= 12 && game.max_count(Green) <= 13 && game.max_count(Blue) <= 14
        })
        .map(|game| game.id)
        .collect()
}

fn main() {
    let data = include_str!("../../data/day2");
    let game_ids = possible_games(data);
    println!("game_ids = {:#?}", game_ids);
    let id_sum: u32 = game_ids.iter().sum();
    println!("id_sum = {:#?}", id_sum);
}

#[test]
fn parses_color() {
    let input = "1 blue";
    assert_eq!(parse_color(input), Ok(("", (1, Blue))));
}

#[test]
fn parses_game_id() {
    assert_eq!(
        parse_game_id("Game 1: 3 blue, 4 red"),
        Ok(("3 blue, 4 red", 1))
    );
}

#[test]
fn parses_line() {
    let game = parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

    assert_eq!(game.max_count(Red), 4);
    assert_eq!(game.max_count(Green), 2);
    assert_eq!(game.max_count(Blue), 6);
}

#[test]
fn adds_color_to_game() {
    let game = Game::default();

    let draw: Draw = HashMap::from([(Red, 2)]);
    let game = game + draw;

    assert_eq!(game.max_count(Red), 2);
    assert_eq!(game.max_count(Green), 0);
    assert_eq!(game.max_count(Blue), 0);

    let draw: Draw = HashMap::from([(Blue, 9)]);
    let game = game + draw;

    assert_eq!(game.max_count(Red), 2);
    assert_eq!(game.max_count(Green), 0);
    assert_eq!(game.max_count(Blue), (9));
}
