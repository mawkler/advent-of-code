use indoc::indoc;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    error::{make_error, ErrorKind},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::{collections::HashMap, ops::Range};

// 0..50
// 50..52
// 52..100

struct MapRange {
    source: u32,
    destination: u32,
    count: u32,
}

impl From<Triple> for MapRange {
    fn from(value: Triple) -> Self {
        MapRange {
            source: value.0,
            destination: value.1,
            count: value.2,
        }
    }
}

struct Map<'a>(HashMap<&'a str, Vec<MapRange>>);

fn test(input: Vec<(u16, u32)>) -> HashMap<u16, u32> {
    input.into_iter().collect()
}

impl<'a> From<Vec<(&'a str, Vec<Triple>)>> for Map<'a> {
    fn from(lines: Vec<(&'a str, Vec<Triple>)>) -> Self {
        let map = lines
            .into_iter()
            .map(|(name, map_lines)| {
                let map_ranges = map_lines.into_iter().map(|triple| triple.into()).collect();
                (name, map_ranges)
            })
            .collect();

        Map(map)
    }
}

fn to_range((source, destination, count): (u32, u32, u32)) -> (Range<u32>, Range<u32>) {
    (source..source + count, destination..destination + count)
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

type Almanac<'a> = (Vec<u32>, Vec<(&'a str, Vec<Triple>)>);

fn parse_almanac(i: &str) -> IResult<&str, Almanac> {
    separated_pair(
        parse_seeds,
        tag("\n\n"),
        separated_list1(tag("\n\n"), parse_map),
    )(i)
}

fn main() {
    let result = parse_almanac(DATA);
    println!("result = {:#?}", result);
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
