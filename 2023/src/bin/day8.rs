use indoc::indoc;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, newline},
    multi::{count, separated_list1},
    sequence::{delimited, separated_pair},
    Finish, IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
struct Network<'a>(HashMap<&'a str, (&'a str, &'a str)>);

impl Network<'_> {
    #![deny(clippy::infinite_iter)]
    fn follow(&self, instructions: &str) -> usize {
        let mut node = "AAA";

        instructions
            .chars()
            .cycle()
            .map_while(|instruction| {
                if node == "ZZZ" {
                    return None::<()>;
                }

                match instruction {
                    'L' => {
                        node = self.0.get(node).unwrap().0;
                        Some(())
                    }
                    'R' => {
                        node = self.0.get(node).unwrap().1;
                        Some(())
                    }
                    other => panic!("Unexpected direction '{}' found", other),
                }
            })
            .count()
    }
}

type Line<'a> = (&'a str, (&'a str, &'a str));

impl<'a> From<Vec<Line<'a>>> for Network<'a> {
    fn from(lines: Vec<Line<'a>>) -> Self {
        Network(lines.into_iter().collect())
    }
}

fn parse_pair(i: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        char('('),
        separated_pair(alpha1, tag(", "), alpha1),
        char(')'),
    )(i)
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    separated_pair(alpha1, tag(" = "), parse_pair)(i)
}

fn parse_network(i: &str) -> IResult<&str, Vec<Line>> {
    separated_list1(newline, parse_line)(i)
}

fn parse(i: &str) -> (&str, Network) {
    let (instructions, network) = separated_pair(alpha1, count(newline, 2), parse_network)(i)
        .finish()
        .unwrap()
        .1;
    (instructions, network.into())
}

fn main() {
    let data = include_str!("../../data/day8");

    let (instructions, network) = parse(data);
    let step_count = network.follow(instructions);

    println!("Part 1: {:?}", step_count);
}
