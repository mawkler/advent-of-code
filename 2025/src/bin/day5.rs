use std::ops;

type Id = u64;

fn parse_available_ids(available_ids: &str) -> impl Iterator<Item = Id> {
    available_ids
        .lines()
        .map(|id| id.parse().expect("is numeric"))
}

fn parse_fresh_id_ranges(fresh_id_ranges: &str) -> impl Iterator<Item = ops::RangeInclusive<u64>> {
    fresh_id_ranges.lines().map(|line| {
        let (lower, upper) = line.split_once('-').expect("range separator exists");
        let lower: Id = lower.parse().expect("is numeric");
        let upper: Id = upper.parse().expect("is numeric");

        lower..=upper
    })
}

fn parse_input(
    str: &str,
) -> (
    impl Iterator<Item = ops::RangeInclusive<Id>>,
    impl Iterator<Item = Id>,
) {
    let (ranges, ids) = str.split_once("\n\n").expect("separator dexists");
    (parse_fresh_id_ranges(ranges), parse_available_ids(ids))
}

fn count_available_ids(str: &str) -> usize {
    let (fresh_id_ranges, available_ids) = parse_input(str);
    // We collect here because we need to iterate multiple times over `fresh_id_ranges`
    let fresh_id_ranges: Vec<_> = fresh_id_ranges.collect();
    available_ids
        .filter(|id| fresh_id_ranges.iter().any(|range| range.contains(id)))
        .count()
}

fn main() {
    let input = include_str!("../../input/day5");
    println!("Part 1: {}", count_available_ids(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn counts_available_ids() {
        assert_eq!(count_available_ids(INPUT), 3);
    }
}
