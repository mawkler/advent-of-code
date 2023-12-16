use indoc::indoc;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    error::{make_error, ErrorKind},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    Finish, IResult,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Range {
    source_start: u32,
    destination_start: u32,
    count: u32,
}

impl From<Triple> for Range {
    fn from((destination_start, source_start, count): Triple) -> Self {
        Range {
            destination_start,
            source_start,
            count,
        }
    }
}

#[derive(Debug)]
struct Map<'a> {
    from: &'a str,
    to: &'a str,
    mappings: Vec<Range>,
}

impl<'a> From<(&'a str, &'a str, Vec<Triple>)> for Map<'a> {
    fn from((from, to, triples): (&'a str, &'a str, Vec<Triple>)) -> Self {
        let mappings: Vec<Range> = triples.into_iter().map(|triple| triple.into()).collect();

        Map { from, to, mappings }
    }
}

#[derive(Debug)]
struct Almanac<'a>(HashMap<&'a str, Map<'a>>);

impl<'a> Almanac<'a> {
    fn map_number(&self, value: u32, source: &str) -> u32 {
        let range = self
            .get(source)
            .expect("Source should exist")
            .mappings
            .iter()
            .find(|&range| (range.source_start..range.source_start + range.count).contains(&value));

        match range {
            Some(range) => {
                let offset = value - range.source_start;
                range.destination_start + value - range.source_start
            }
            None => value,
        }
    }

    fn map(&self, value: u32, source: &str) {
        let f = self.get(source).expect("Source should exist");
    }
}

impl<'a> From<Vec<(&'a str, Vec<Triple>)>> for Almanac<'a> {
    fn from(mappings: Vec<(&'a str, Vec<Triple>)>) -> Self {
        let almanac = mappings
            .into_iter()
            .map(|(name, triples)| {
                let (from, to) = name.split_once("-to-").expect("Should exist in name");
                (from, (from, to, triples).into())
            })
            .collect();

        Almanac(almanac)
    }
}

impl<'a> std::ops::Deref for Almanac<'a> {
    type Target = HashMap<&'a str, Map<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn parse_to_number(i: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(i)
}

fn parse_seeds(i: &str) -> IResult<&str, Vec<u32>> {
    preceded(tag("seeds: "), separated_list1(tag(" "), parse_to_number))(i)
}

fn parse_map_line(i: &str) -> IResult<&str, (u32, u32, u32)> {
    let (remaining, numbers) = separated_list1(space1, parse_to_number)(i)?;

    match numbers.as_slice() {
        [first, second, third] => Ok((remaining, (*first, *second, *third))),
        _ => Err(nom::Err::Failure(make_error(i, ErrorKind::SeparatedList))),
    }
}

type Triple = (u32, u32, u32);

fn parse_map(i: &str) -> IResult<&str, (&str, Vec<Triple>)> {
    separated_pair(
        is_not(" "),
        tag(" map:\n"),
        separated_list1(newline, parse_map_line),
    )(i)
}

type RawAlmanac<'a> = (Vec<u32>, Vec<(&'a str, Vec<Triple>)>);

fn parse_almanac(i: &str) -> IResult<&str, RawAlmanac> {
    separated_pair(
        parse_seeds,
        tag("\n\n"),
        separated_list1(tag("\n\n"), parse_map),
    )(i)
}

fn get_locations(string: &str) -> u32 {
    let (seeds, almanac) = parse_almanac(string).unwrap().1;
    let almanac: Almanac = almanac.into();
    todo!();
}

fn main() {
    let almanac: Almanac = parse_almanac(DATA).finish().unwrap().1 .1.into();

    // almanac.map_number(79, "seed")
}

const DATA: &str = indoc! {"
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4
"};

#[test]
fn parses_almanac() {
    use nom::Finish;

    let (_, almanac) = parse_almanac(DATA).finish().unwrap();
    println!("result = {:#?}", almanac);
    let expected = (
        vec![79, 14, 55, 13],
        vec![
            ("seed-to-soil", vec![(50, 98, 2), (52, 50, 48)]),
            (
                "soil-to-fertilizer",
                vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)],
            ),
            (
                "fertilizer-to-water",
                vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)],
            ),
            ("water-to-light", vec![(88, 18, 7), (18, 25, 70)]),
            (
                "light-to-temperature",
                vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)],
            ),
            ("temperature-to-humidity", vec![(0, 69, 1), (1, 0, 69)]),
            ("humidity-to-location", vec![(60, 56, 37), (56, 93, 4)]),
        ],
    );

    assert_eq!(almanac, expected);
}

#[test]
fn maps_range() {
    let almanac: Almanac = parse_almanac(DATA).finish().unwrap().1 .1.into();

    assert_eq!(almanac.map_number(0, "seed"), 0);
    assert_eq!(almanac.map_number(49, "seed"), 49);
    assert_eq!(almanac.map_number(50, "seed"), 52);
    assert_eq!(almanac.map_number(51, "seed"), 53);
    assert_eq!(almanac.map_number(98, "seed"), 50);
    assert_eq!(almanac.map_number(99, "seed"), 51);
    assert_eq!(almanac.map_number(100, "seed"), 100);
}
