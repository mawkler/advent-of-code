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
    fn cycle_length(&self, instructions: &str, start_node_name: &str) -> usize {
        let steps = instructions
            .chars()
            .cycle()
            .fold_while(vec![start_node_name], |visited_nodes, instruction| {
                let node_name = *visited_nodes.last().unwrap();

                let next_node = match instruction {
                    'L' => self.0.get(node_name).unwrap().0,
                    'R' => self.0.get(node_name).unwrap().1,
                    other => panic!("Unexpected direction '{}' found", other),
                };

                if visited_nodes.len() > instructions.len() && visited_nodes.contains(&next_node) {
                    let pos = visited_nodes
                        .iter()
                        .position(|node| node == &next_node)
                        .unwrap();
                    println!("acc.len() = {:#?}", visited_nodes.len());
                    println!("pos = {:#?}", pos);
                    println!("next_node = {:#?}", next_node);
                    Done(visited_nodes)
                } else {
                    Continue(visited_nodes.into_iter().chain(Some(next_node)).collect())
                }
            })
            .into_inner()
            .len();

        steps
    }

    fn path_length(&self, instructions: &str, start_node_name: &str) -> usize {
        let steps = instructions
            .chars()
            .cycle()
            .fold_while(vec![start_node_name], |acc, instruction| {
                let node_name = *acc.last().unwrap();

                let next_node = match instruction {
                    'L' => self.0.get(node_name).unwrap().0,
                    'R' => self.0.get(node_name).unwrap().1,
                    other => panic!("Unexpected direction '{}' found", other),
                };

                if next_node.ends_with('Z') {
                    println!("(path_length) next_node = {:#?}", next_node);
                    // println!("acc = {:#?}", acc);
                    Done(acc)
                } else {
                    Continue(acc.into_iter().chain(Some(next_node)).collect())
                }
            })
            .into_inner()
            .len();

        steps
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

    let node = "PBA";
    // let node = "22A";
    let cycle = network.cycle_length(instructions, node);
    println!("cycle = {:#?}", cycle);

    let path_length = network.path_length(instructions, node);
    println!("path_length = {:#?}", path_length);

    // let step_count = network.count_steps(instructions);

    // println!(
    //     "network.get_starting_nodes() = {:#?}",
    //     network.get_starting_nodes()
    // );

    // println!("Part 2: {:?}", step_count);
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
