use indoc::indoc;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use nom::character::complete::alphanumeric1;
use nom::{
    bytes::complete::tag,
    character::complete::{char, newline},
    multi::{count, separated_list1},
    sequence::{delimited, separated_pair},
    Finish, IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
struct Network<'a>(HashMap<&'a str, (&'a str, &'a str)>);

impl Network<'_> {
    fn follow(&self, instructions: &str) -> usize {
        let starting_nodes = self.get_starting_nodes();
        let (count, _) = instructions
            .chars()
            .cycle()
            .fold_while((0, starting_nodes), |acc, instruction| {
                let (count, nodes) = acc;
                if nodes.iter().all(|node| node.ends_with('Z')) {
                    return Done((count, nodes));
                }

                let new_acc = nodes.iter().map(|&node_name| match instruction {
                    'L' => self.0.get(node_name).unwrap().0,
                    'R' => self.0.get(node_name).unwrap().1,
                    other => panic!("Unexpected direction '{}' found", other),
                });

                Continue((count + 1, new_acc.collect()))
            })
            .into_inner();

        count
    }

    fn get_starting_nodes(&self) -> Vec<&str> {
        self.0
            .iter()
            .filter(|&(node_name, _)| node_name.ends_with('A'))
            .map(|(&node_name, _)| node_name)
            .collect()
    }
}

type Line<'a> = (&'a str, (&'a str, &'a str));

impl<'a> From<Vec<Line<'a>>> for Network<'a> {
    fn from(lines: Vec<Line<'a>>) -> Self {
        Network(lines.into_iter().collect())
    }
}

fn node_ends_with_z(node: &str) -> bool {
    node.ends_with('z')
}

fn parse_pair(i: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        char('('),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
        char(')'),
    )(i)
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    separated_pair(alphanumeric1, tag(" = "), parse_pair)(i)
}

fn parse_network(i: &str) -> IResult<&str, Vec<Line>> {
    separated_list1(newline, parse_line)(i)
}

fn parse(i: &str) -> (&str, Network) {
    let rest = separated_pair(alphanumeric1, count(newline, 2), parse_network)(i)
        .finish()
        .unwrap()
        .0;

    let (instructions, network) =
        separated_pair(alphanumeric1, count(newline, 2), parse_network)(i)
            .finish()
            .unwrap()
            .1;
    (instructions, network.into())
}

fn main() {
    let data = include_str!("../../data/day8");

    // let data = indoc! {"
    //     LR

    //     11A = (11B, XXX)
    //     11B = (XXX, 11Z)
    //     11Z = (11B, XXX)
    //     22A = (22B, XXX)
    //     22B = (22C, 22C)
    //     22C = (22Z, 22Z)
    //     22Z = (22B, 22B)
    //     XXX = (XXX, XXX)
    // "};

    let (instructions, network) = parse(data);
    let step_count = network.follow(instructions);

    println!("Part 2: {:?}", step_count);
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::parse;

    #[test]
    fn gets_starting_nodes() {
        let data = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "};
        let (_, network) = parse(data);
        let result = network.get_starting_nodes();

        assert_eq!(result, vec!["22A", "11A",])
    }
}
