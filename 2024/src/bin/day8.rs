use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    ops::{Add, Mul, Sub},
};

type Antennas = HashMap<char, HashSet<Coordinate>>;

#[derive(Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord, Debug)]
struct Coordinate(i32, i32);

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<i32> for Coordinate {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Coordinate(self.0 * rhs, self.1 * rhs)
    }
}

#[derive(Debug)]
struct Map {
    antennas: Antennas,
    height: usize,
    width: usize,
}

impl Map {
    fn get_first_antinodes(&self) -> impl Iterator<Item = Coordinate> + use<'_> {
        self.antennas
            .values()
            .flat_map(|antennas| {
                antennas.iter().permutations(2).flat_map(|coordinates| {
                    let (a1, a2) = get_first_antinodes(*coordinates[0], *coordinates[1]);
                    [a1, a2]
                })
            })
            .filter(|coordinate| !self.is_out_of_bounds(*coordinate))
            .unique()
    }

    fn get_antinodes(&self) -> impl Iterator<Item = Coordinate> + use<'_> {
        self.antennas
            .values()
            .flat_map(|antennas| {
                antennas.iter().permutations(2).flat_map(|coordinates| {
                    self.get_antinodes_for_coordinate(*coordinates[0], *coordinates[1])
                })
            })
            .filter(|coordinate| !self.is_out_of_bounds(*coordinate))
            .unique()
    }

    fn get_antinodes_for_coordinate(
        &self,
        c1: Coordinate,
        c2: Coordinate,
    ) -> impl Iterator<Item = Coordinate> {
        // n == height should be wide enough to cover the entire map
        (0..self.height).flat_map(move |n| {
            let antinode1 = c1 + (c1 - c2) * n as i32;
            let antinode2 = c2 + (c2 - c1) * n as i32;
            [antinode1, antinode2]
        })
    }

    fn is_out_of_bounds(&self, coordinate: Coordinate) -> bool {
        coordinate.0.is_negative()
            || coordinate.1.is_negative()
            || coordinate.0 as usize >= self.width
            || coordinate.1 as usize >= self.height
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let antinodes: Vec<_> = self.get_antinodes().collect();

        let map = (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| {
                        let (x, y) = (x as i32, y as i32);

                        if antinodes.contains(&Coordinate(x, y)) {
                            return '#';
                        }

                        let antenna = self
                            .antennas
                            .iter()
                            .find(|(_, positions)| positions.contains(&Coordinate(x, y)));

                        if let Some(antenna) = antenna {
                            *antenna.0
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .join("\n");
        write!(f, "{map}")
    }
}

fn parse_map(map: &str) -> Map {
    let antennas = map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (c, Coordinate(x as i32, y as i32)))
        })
        .fold(HashMap::new(), |mut map: Antennas, (c, coordinate)| {
            if c != '.' {
                map.entry(c).or_default().insert(coordinate);
            }
            map
        });

    Map {
        antennas,
        height: map.lines().count(),
        width: map.lines().next().expect("Has lines").len(),
    }
}

fn get_first_antinodes(c1: Coordinate, c2: Coordinate) -> (Coordinate, Coordinate) {
    let antinode1 = c1 + (c1 - c2);
    let antinode2 = c2 + (c2 - c1);
    (antinode1, antinode2)
}

fn main() {
    let data = include_str!("../../data/day8");

    println!("Part 1: {}", parse_map(data).get_first_antinodes().count());
    println!("Part 2: {}", parse_map(data).get_antinodes().count());
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use itertools::assert_equal;
    use itertools::Itertools;

    fn expected_sorted_hash_set(expected: &[(i32, i32)]) -> Vec<Coordinate> {
        expected
            .iter()
            .collect::<HashSet<_>>()
            .iter()
            .map(|(x, y)| Coordinate(*x, *y))
            .sorted()
            .collect()
    }

    #[test]
    fn parses_map() {
        let map = indoc! {"
            ............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............
        "};

        let map = parse_map(map);

        let zero_antennas = map.antennas.get(&'0').unwrap();
        let a_antennas = map.antennas.get(&'A').unwrap();

        let zero_expected = expected_sorted_hash_set(&[(8, 1), (5, 2), (7, 3), (4, 4)]);
        assert_equal(zero_expected, zero_antennas.iter().sorted().cloned());

        let a_expected = expected_sorted_hash_set(&[(6, 5), (8, 8), (9, 9)]);
        assert_equal(a_expected, a_antennas.iter().cloned().sorted());
    }

    #[test]
    fn gets_first_antinodes() {
        let antenna1 = Coordinate(4, 3);
        let antenna2 = Coordinate(5, 5);

        let expected_antinodes = (Coordinate(3, 1), Coordinate(6, 7));
        assert_eq!(expected_antinodes, get_first_antinodes(antenna1, antenna2))
    }

    #[test]
    fn gets_first_antinodes_from_map() {
        let map = indoc! {"
            ..........
            ..........
            ..........
            ....a.....
            ..........
            .....a....
            ..........
            ..........
            ..........
            ..........
        "};
        let map = parse_map(map);

        let expected_antinodes: Vec<_> = [Coordinate(3, 1), Coordinate(6, 7)]
            .into_iter()
            .sorted()
            .collect();
        let antinodes = map.get_first_antinodes().sorted().collect::<Vec<_>>();

        assert_eq!(expected_antinodes, antinodes)
    }

    #[test]
    fn counts_first_antinodes() {
        let map = indoc! {"
            ............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............
        "};
        assert_eq!(14, parse_map(map).get_first_antinodes().count());
    }

    #[test]
    fn gets_all_antinodes() {
        let map = indoc! {"
            T...........
            ...T........
            .T..........
            ............
            ............
            ............
            ............
            ............
            ............
            ............
        "};
        let map = parse_map(map);

        assert_eq!(9, map.get_antinodes().count());
    }
}
