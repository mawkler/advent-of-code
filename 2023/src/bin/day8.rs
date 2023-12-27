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
use num::integer::lcm;
use std::collections::HashMap;

#[derive(Debug)]
struct Network<'a>(HashMap<&'a str, (&'a str, &'a str)>);

impl Network<'_> {
    fn get_end_frequency(&self, instructions: &str, start_node_name: &str) -> usize {
        let init = (vec![start_node_name], None);
        let (visited_nodes, _) = instructions
            .chars()
            .cycle()
            .fold_while(init, |(visited_nodes, end_node), instruction| {
                let node_name = *visited_nodes.last().unwrap();

                let next_node = match instruction {
                    'L' => self.0.get(node_name).unwrap().0,
                    'R' => self.0.get(node_name).unwrap().1,
                    other => panic!("Unexpected direction '{}' found", other),
                };

                if end_node.is_some_and(|end_node| next_node == end_node) {
                    return Done((visited_nodes, end_node));
                }

                let visited_nodes = visited_nodes.into_iter().chain(Some(next_node)).collect();
                if next_node.ends_with('Z') {
                    Continue((vec![next_node], Some(next_node)))
                } else {
                    Continue((visited_nodes, end_node))
                }
            })
            .into_inner();

        visited_nodes.len()
    }

    fn get_starting_nodes(&self) -> Vec<&str> {
        self.0
            .iter()
            .filter(|&(node_name, _)| node_name.ends_with('A'))
            .map(|(&node_name, _)| node_name)
            .collect()
    }

    fn get_steps_to_end_nodes(&self, starting_nodes: Vec<&str>, instructions: &str) -> usize {
        let least_common_denominator = starting_nodes
            .iter()
            .map(|starting_node| self.get_end_frequency(instructions, starting_node))
            .reduce(lcm)
            .unwrap();

        least_common_denominator
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
    let (instructions, network) =
        separated_pair(alphanumeric1, count(newline, 2), parse_network)(i)
            .finish()
            .unwrap()
            .1;
    (instructions, network.into())
}

fn main() {
    let data = include_str!("../../data/day8");

    let (instructions, network) = parse(data);
    let start_nodes = network.get_starting_nodes();
    let steps = network.get_steps_to_end_nodes(start_nodes, instructions);

    println!("Part 2: {}", steps);
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
