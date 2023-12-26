use indoc::indoc;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
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
    fn follow(&self, instructions: &str) -> usize {
        let steps = instructions
            .chars()
            .cycle()
            .fold_while(vec!["AAA"], |acc, instruction| {
                let node_name = *acc.last().unwrap();

                if node_name == "ZZZ" {
                    return Done(acc);
                }

                let next_node = match instruction {
                    'L' => self.0.get(node_name).unwrap().0,
                    'R' => self.0.get(node_name).unwrap().1,
                    other => panic!("Unexpected direction '{}' found", other),
                };

                Continue(acc.into_iter().chain(Some(next_node)).collect())
            })
            .into_inner()
            .len();

        // Count steps, not nodes
        steps - 1
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

    // let data = indoc! {"
    //     RL

    //     AAA = (BBB, CCC)
    //     BBB = (DDD, EEE)
    //     CCC = (ZZZ, GGG)
    //     DDD = (DDD, DDD)
    //     EEE = (EEE, EEE)
    //     GGG = (GGG, GGG)
    //     ZZZ = (ZZZ, ZZZ)
    // "};

    // let data = indoc! {"
    //     LLR

    //     AAA = (BBB, BBB)
    //     BBB = (AAA, ZZZ)
    //     ZZZ = (ZZZ, ZZZ)
    // "};

    let (instructions, network) = parse(data);
    let step_count = network.follow(instructions);

    println!("Part 1: {:?}", step_count);
}
