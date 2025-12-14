#![feature(core_float_math)]

use core::f32::math::{powi, sqrt};
use itertools::Itertools;
use std::{collections::BTreeSet, hash::Hash, ops::Sub};

#[derive(Clone, PartialEq, Eq, Hash)]
// We use a `BTreeSet` here to get Hash and Eq for free. Apparently, `HashSet`
// doesn't implement `Hash`.
struct Pair(BTreeSet<Coordinate>);

impl Pair {
    fn connects_with(&self, pair: &Pair) -> bool {
        self.0.intersection(&pair.0).count() > 0
    }
}

impl From<(Coordinate, Coordinate)> for Pair {
    fn from(pair: (Coordinate, Coordinate)) -> Self {
        Pair(BTreeSet::from([pair.0, pair.1]))
    }
}

impl<'a> From<&'a Pair> for (&'a Coordinate, &'a Coordinate) {
    fn from(pair: &'a Pair) -> Self {
        pair.0.iter().collect_tuple().expect("has two elements")
    }
}

impl std::fmt::Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (c1, c2) = self.into();
        write!(f, "{c1:?} - {c2:?}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Ord)]
struct Coordinate(u32, u32, u32);

impl Coordinate {
    fn distance_to(&self, coordinate: Coordinate) -> f32 {
        fn square(n: f32) -> f32 {
            powi(n, 2)
        }

        sqrt(
            square(coordinate.0 as f32 - self.0 as f32)
                + square(coordinate.1 as f32 - self.1 as f32)
                + square(coordinate.2 as f32 - self.2 as f32),
        )
    }
}

impl std::fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, Coordinate(x, y, z): Self) -> Self::Output {
        Coordinate(self.0 - x, self.1 - y, self.2 - z)
    }
}

impl From<&str> for Coordinate {
    fn from(value: &str) -> Self {
        let (x, y, z) = value
            .split(',')
            .map(|n| n.parse().expect("is numeric"))
            .collect_tuple()
            .expect("has three numbers");
        Coordinate(x, y, z)
    }
}

fn flatten_circuits(circuits: Vec<Vec<Pair>>) -> impl Iterator<Item = BTreeSet<Coordinate>> {
    circuits.into_iter().map(|circuit| {
        circuit.iter().fold(BTreeSet::new(), |acc, pair| {
            let (&c1, &c2) = pair.into();
            acc.into_iter().chain([c1, c2]).collect()
        })
    })
}

fn parse(str: &str) -> impl Iterator<Item = Coordinate> + Clone {
    str.lines().map(Into::into)
}

mod part1 {
    use crate::{Coordinate, Pair};
    use itertools::Itertools;
    use std::collections::BTreeSet;

    /// Sorted closest to furthest coordinate pairs
    pub(crate) fn sorted_coordinate_pairs(
        coordinates: impl Iterator<Item = Coordinate>,
    ) -> impl Iterator<Item = Pair> {
        coordinates
            .permutations(2)
            .map(|permutation| {
                let (c1, c2) = permutation
                    .into_iter()
                    .collect_tuple()
                    .expect("contains two coordinates");
                let distance = c1.distance_to(c2);
                (c1, c2, distance)
            })
            .sorted_by(|(_, _, d1), (_, _, d2)| f32::total_cmp(d1, d2))
            .map(|(c1, c2, _)| Pair::from((c1, c2)))
            .unique() // Remove pair inversions
    }

    fn connect_n_closest_coordinates(
        coordinates: impl Iterator<Item = Coordinate>,
        n: usize,
    ) -> impl Iterator<Item = BTreeSet<Coordinate>> {
        let mut pairs = sorted_coordinate_pairs(coordinates);
        let mut circuits: Vec<Vec<Pair>> = vec![];

        for pair in pairs.by_ref().take(n) {
            let mut matching_circuits = circuits
                .clone()
                .into_iter()
                .enumerate()
                .filter(|(_, circuit)| circuit.iter().any(|p| pair.connects_with(p)));

            let Some(connecting_circuit1) = matching_circuits.next() else {
                // Doesn't connect with any of the circuits
                circuits.push(vec![pair]);
                continue;
            };

            match matching_circuits.next() {
                None => {
                    // Connects with one of the circuits
                    circuits[connecting_circuit1.0].push(pair);
                    continue;
                }
                Some(connecting_circuit2) => {
                    // Connects two different circuits
                    let connecting_circuit2 = circuits.swap_remove(connecting_circuit2.0);
                    let new_circuit = connecting_circuit1.1.into_iter().chain(connecting_circuit2);

                    circuits[connecting_circuit1.0] = new_circuit.collect();
                }
            };
        }

        crate::flatten_circuits(circuits)
    }

    pub fn multiply_three_largest_circuits(input: &str) -> usize {
        let coordinates = crate::parse(input);
        let circuits = connect_n_closest_coordinates(coordinates, 1000);

        circuits
            .map(|circuit| circuit.len())
            .sorted_by(|len1, len2| Ord::cmp(len2, len1)) // descending order
            .take(3)
            .product()
    }
}

mod part2 {
    use crate::{Coordinate, Pair};

    fn get_last_pair_to_connect_all_coordinates(
        coordinates: impl Iterator<Item = Coordinate> + Clone,
    ) -> Pair {
        let pairs = crate::part1::sorted_coordinate_pairs(coordinates.clone());
        let mut circuits: Vec<Vec<Pair>> = vec![];
        let mut latest_pair = None;

        for pair in pairs {
            // TODO: this might not actually be the last pair
            if has_connected_all_coordinates(coordinates.clone(), &circuits) {
                return latest_pair.expect("`pairs` is non-empty");
            }

            let mut matching_circuits = circuits
                .clone()
                .into_iter()
                .enumerate()
                .filter(|(_, circuit)| circuit.iter().any(|p| pair.connects_with(p)));

            let Some(connecting_circuit1) = matching_circuits.next() else {
                // Doesn't connect with any of the circuits
                circuits.push(vec![pair.clone()]);
                latest_pair = Some(pair);
                continue;
            };

            match matching_circuits.next() {
                None => {
                    // Connects with one of the circuits
                    circuits[connecting_circuit1.0].push(pair.clone());
                    latest_pair = Some(pair);
                    continue;
                }
                Some(connecting_circuit2) => {
                    // Connects two different circuits
                    let connecting_circuit2 = circuits.swap_remove(connecting_circuit2.0);
                    let new_circuit = connecting_circuit1.1.into_iter().chain(connecting_circuit2);

                    circuits[connecting_circuit1.0] = new_circuit.collect();
                    latest_pair = Some(pair);
                }
            };
        }

        unreachable!()
    }

    fn has_connected_all_coordinates(
        coordinates: impl Iterator<Item = Coordinate> + Clone,
        circuits: &[Vec<Pair>],
    ) -> bool {
        let Some(circuits) = crate::flatten_circuits(circuits.to_vec()).next() else {
            return false;
        };

        let length = circuits.len();
        println!("Connected circuits: {length}");

        coordinates.count() == length
    }

    pub fn last_pair_x_coordinate_product(input: &str) -> u128 {
        let coordinates = crate::parse(input);
        let pair = get_last_pair_to_connect_all_coordinates(coordinates);
        let (c1, c2) = (&pair).into();

        c1.0 as u128 * c2.0 as u128
    }
}

fn main() {
    let input = include_str!("../../input/day8");

    println!("Part 1: {}", part1::multiply_three_largest_circuits(input));
    println!("Part 2: {}", part2::last_pair_x_coordinate_product(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn gets_distance_between_coordinates() {
        let c1: Coordinate = "162,817,812".into();
        let c2: Coordinate = "425,690,689".into();

        assert_eq!(c1.distance_to(c2), 316.9022);
    }

    #[test]
    fn pair_equality() {
        let c1: Coordinate = "162,817,812".into();
        let c2: Coordinate = "425,690,689".into();
        let p1 = Pair::from((c1, c2));
        let p2 = Pair::from((c2, c1));

        assert_eq!(p1, p2);
        assert!([p1].contains(&p2));
    }

    #[test]
    fn pairs_with() {
        let expected_pairs = [
            ("162,817,812", "425,690,689"),
            ("162,817,812", "431,825,988"),
            ("906,360,560", "805,96,715"),
            ("431,825,988", "425,690,689"),
        ]
        .map(|(c1, c2)| Pair::from((c1.into(), c2.into())));

        let coordinates = crate::parse(INPUT);
        let pairs = part1::sorted_coordinate_pairs(coordinates).take(4);

        assert!(itertools::equal(pairs, expected_pairs));
    }

    #[test]
    fn connects_with() {
        let p1 = Pair::from(("162,817,812".into(), "425,690,689".into()));
        let p2 = Pair::from(("162,817,812".into(), "431,825,988".into()));
        let p3 = Pair::from(("906,360,560".into(), "805,96,715".into()));

        assert!(p1.connects_with(&p2));
        assert!(!p1.connects_with(&p3));
        assert!(!p2.connects_with(&p3));
    }
}
