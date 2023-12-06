use self::Color::{Blue, Green, Red};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};
use std::{collections::HashMap, ops::Add};

type Count = u16;

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

type Id = u32;

#[derive(Debug, Default)]
struct Game {
    id: Id,
    colors: HashMap<Color, Count>,
}

impl Add<(Count, Color)> for Game {
    type Output = Game;

    fn add(self, (count, color): (Count, Color)) -> Game {
        let mut colors = self.colors.clone();
        *colors.entry(color).or_insert(0) += count;

        Game {
            id: self.id,
            colors,
        }
    }
}

impl From<(Id, Vec<(Count, Color)>)> for Game {
    fn from((id, colors): (Id, Vec<(Count, Color)>)) -> Self {
        colors
            .into_iter()
            .fold(Game::new(id), |game, color| game + color)
    }
}

impl Game {
    fn new(id: Id) -> Self {
        Game {
            id,
            colors: HashMap::new(),
        }
    }

    fn count(&self, color: Color) -> Count {
        *self.colors.get(&color).unwrap_or(&0)
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

fn parse_colors(i: &str) -> IResult<&str, Vec<(Count, Color)>> {
    separated_list1(alt((tag(", "), tag("; "))), parse_color)(i)
}

fn parse_line(i: &str) -> Game {
    let (_, game) = tuple((parse_game_id, parse_colors))(i).unwrap();

    game.into()
}

fn possible_games(lines: &str) -> Vec<Id> {
    lines
        .lines()
        .map(parse_line)
        .filter(|game| game.count(Red) <= 12 && game.count(Green) <= 13 && game.count(Blue) <= 14)
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
fn parses_colors() {
    let input = "1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
    assert_eq!(
        parse_colors(input),
        Ok((
            "",
            vec![
                (1, Blue,),
                (2, Green,),
                (3, Green,),
                (4, Blue,),
                (1, Red,),
                (1, Green,),
                (1, Blue,),
            ],
        ),)
    );
}

#[test]
fn parses_line() {
    let game = parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

    assert_eq!(game.count(Red), 5);
    assert_eq!(game.count(Green), 4);
    assert_eq!(game.count(Blue), 9);
}

#[test]
fn adds_color_to_game() {
    let game = Game::default();
    let game = game + (2, Red);

    assert_eq!(game.count(Red), 2);
    assert_eq!(game.count(Green), 0);
    assert_eq!(game.count(Blue), 0);

    let game = game + (9, Blue);
    assert_eq!(game.count(Red), 2);
    assert_eq!(game.count(Green), 0);
    assert_eq!(game.count(Blue), (9));
}
