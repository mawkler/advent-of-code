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

type Number = u64;

#[derive(Debug, Clone)]
struct Range {
    source_start: Number,
    destination_start: Number,
    count: Number,
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
    _from: &'a str,
    to: &'a str,
    mappings: Vec<Range>,
}

impl Map<'_> {
    fn map_number(&self, value: Number) -> Number {
        let range = self
            .mappings
            .iter()
            .find(|&range| (range.source_start..range.source_start + range.count).contains(&value));

        match range {
            Some(range) => {
                let offset = value - range.source_start;
                range.destination_start + offset
            }
            None => value,
        }
    }
}

impl<'a> From<(&'a str, &'a str, Vec<Triple>)> for Map<'a> {
    fn from((from, to, triples): (&'a str, &'a str, Vec<Triple>)) -> Self {
        let mappings: Vec<Range> = triples.into_iter().map(|triple| triple.into()).collect();

        Map {
            _from: from,
            to,
            mappings,
        }
    }
}

#[derive(Debug)]
struct Almanac<'a>(HashMap<&'a str, Map<'a>>);

impl<'a> Almanac<'a> {
    fn seed_to_location(&self, seed: Number) -> Number {
        let mut current_map = self.get("seed");
        let mut current_number = seed;

        while let Some(map) = current_map {
            current_number = map.map_number(current_number);
            current_map = self.get(map.to);
        }

        current_number
    }

    fn seeds_to_locations(&self, range_start: Number, count: Number) -> Vec<Number> {
        (range_start..range_start + count)
            .map(|seed| self.seed_to_location(seed))
            .collect()
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

fn parse_to_number(i: &str) -> IResult<&str, Number> {
    map_res(digit1, str::parse)(i)
}

fn parse_seeds(i: &str) -> IResult<&str, Vec<Number>> {
    preceded(tag("seeds: "), separated_list1(tag(" "), parse_to_number))(i)
}

fn parse_map_line(i: &str) -> IResult<&str, (Number, Number, Number)> {
    let (remaining, numbers) = separated_list1(space1, parse_to_number)(i)?;

    match numbers.as_slice() {
        [first, second, third] => Ok((remaining, (*first, *second, *third))),
        _ => Err(nom::Err::Failure(make_error(i, ErrorKind::SeparatedList))),
    }
}

type Triple = (Number, Number, Number);

fn parse_map(i: &str) -> IResult<&str, (&str, Vec<Triple>)> {
    separated_pair(
        is_not(" "),
        tag(" map:\n"),
        separated_list1(newline, parse_map_line),
    )(i)
}

type RawAlmanac<'a> = (Vec<Number>, Vec<(&'a str, Vec<Triple>)>);

fn parse_almanac(i: &str) -> IResult<&str, RawAlmanac> {
    separated_pair(
        parse_seeds,
        tag("\n\n"),
        separated_list1(tag("\n\n"), parse_map),
    )(i)
}

fn main() {
    let input = include_str!("../../data/day5");
    let (seeds, almanac) = parse_almanac(input).finish().unwrap().1;
    let almanac: Almanac = almanac.into();

    let result = seeds
        .iter()
        .map(|&seed| almanac.seed_to_location(seed))
        .min()
        .expect("Min should exist");

    println!("Part 1: {}", result);

    let lowest_location = seeds
        .iter()
        .tuples()
        .flat_map(|(&seeds, &count)| almanac.seeds_to_locations(seeds, count))
        .min()
        .expect("Should exist");

    println!("Part 2: {}", lowest_location);
}

#[cfg(test)]
use indoc::indoc;

#[cfg(test)]
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
    let (_, almanac) = parse_almanac(DATA).finish().unwrap();
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
    let map = almanac.get("seed").unwrap();

    assert_eq!(map.map_number(0), 0);
    assert_eq!(map.map_number(49), 49);
    assert_eq!(map.map_number(50), 52);
    assert_eq!(map.map_number(51), 53);
    assert_eq!(map.map_number(98), 50);
    assert_eq!(map.map_number(99), 51);
    assert_eq!(map.map_number(100), 100);
}

#[test]
fn maps_seeds() {
    let almanac: Almanac = parse_almanac(DATA).finish().unwrap().1 .1.into();
    assert_eq!(almanac.seed_to_location(79), 82);
    assert_eq!(almanac.seed_to_location(14), 43);
    assert_eq!(almanac.seed_to_location(55), 86);
    assert_eq!(almanac.seed_to_location(13), 35);
}
