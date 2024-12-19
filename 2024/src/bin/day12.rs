use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

type Coordinate = (i32, i32);
type Plot = char;

#[derive(Clone)]
struct Garden(String);

#[derive(Debug, PartialEq, Clone)]
struct Region {
    name: char,
    plots: HashSet<Coordinate>,
    perimeters: Vec<Perimeter>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd)]
enum Perimeter {
    Top(Coordinate),
    Right(Coordinate),
    Bottom(Coordinate),
    Left(Coordinate),
}

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug)]
struct Corner<'a>(&'a Perimeter, &'a Perimeter);

impl PartialEq for Corner<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 || self.0 == other.1 && self.1 == other.0
    }
}

impl Eq for Corner<'_> {}

impl Hash for Corner<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.0 <= self.1 {
            self.0.hash(state);
            self.1.hash(state);
        } else {
            self.1.hash(state);
            self.0.hash(state);
        }
    }
}

impl Perimeter {
    fn from(coordinate: Coordinate, direction: Direction) -> Self {
        match direction {
            Direction::Up => Perimeter::Top(coordinate),
            Direction::Right => Perimeter::Right(coordinate),
            Direction::Down => Perimeter::Bottom(coordinate),
            Direction::Left => Perimeter::Left(coordinate),
        }
    }

    fn create_outer_corner(&self, other: &Perimeter) -> bool {
        let (x, y) = self.coordinate();

        match self {
            Perimeter::Top(_) => [(x - 1, y + 1), (x + 1, y + 1)],
            Perimeter::Right(_) => [(x - 1, y + 1), (x - 1, y - 1)],
            Perimeter::Bottom(_) => [(x - 1, y - 1), (x + 1, y - 1)],
            Perimeter::Left(_) => [(x + 1, y + 1), (x + 1, y - 1)],
        }
        .iter()
        .any(|coordinate| *coordinate == other.coordinate())
    }

    fn creates_corner(&self, other: &Perimeter) -> bool {
        if self.coordinate() != other.coordinate() {
            return self.create_outer_corner(other);
        }

        #[allow(clippy::match_like_matches_macro)]
        match (self, other) {
            (Perimeter::Top(_), Perimeter::Right(_) | Perimeter::Left(_)) => true,
            (Perimeter::Right(_), Perimeter::Bottom(_) | Perimeter::Top(_)) => true,
            (Perimeter::Bottom(_), Perimeter::Right(_) | Perimeter::Left(_)) => true,
            (Perimeter::Left(_), Perimeter::Top(_) | Perimeter::Bottom(_)) => true,
            _ => false,
        }
    }

    fn coordinate(&self) -> Coordinate {
        match self {
            Perimeter::Top(c) => *c,
            Perimeter::Right(c) => *c,
            Perimeter::Bottom(c) => *c,
            Perimeter::Left(c) => *c,
        }
    }
}

// Part 1
fn sum_region_costs(garden: &str) -> usize {
    let garden = Garden(garden.to_string());
    garden.get_regions().map(Region::get_cost).sum()
}

// Part 2
fn sum_discounted_region_costs(garden: &str) -> usize {
    let garden = Garden(garden.to_string());

    let regions = garden.get_regions();
    // for region in regions {
    //     println!("{:?}", region);
    // }

    regions.map(|region| region.count_sides()).sum()
}

impl Garden {
    fn get_regions(self) -> impl Iterator<Item = Region> {
        let height = self.0.len();
        let width = self.0.lines().nth(0).unwrap().len();

        let mut visited_regions: Vec<Region> = vec![];

        let coordinates = (0..height).flat_map(move |y| (0..width).map(move |x| (x, y)));
        let regions: HashMap<_, Vec<Region>> = coordinates
            .flat_map(move |(x, y)| {
                let (x, y) = (x as i32, y as i32);

                if visited_regions.iter().any(|r| r.plots.contains(&(x, y))) {
                    return None;
                }

                self.get_plot((x, y))?;

                let region = self.clone().get_region((x, y));
                visited_regions.push(region.clone());

                Some(region)
            })
            .into_grouping_map_by(|region| region.name)
            .collect();

        regions.into_values().flatten()
    }

    pub fn get_region(self, coordinate: Coordinate) -> Region {
        let visited = HashSet::from([coordinate]);
        let perimeters = Vec::new();
        let plot = self.get_plot(coordinate).expect("Plot should exist");

        let (region, _) = self.flood(coordinate, plot, visited, perimeters);
        region
    }

    fn flood(
        self,
        coordinate: Coordinate,
        plot_type: Plot,
        mut visited: HashSet<Coordinate>,
        perimeters: Vec<Coordinate>,
    ) -> (Region, HashSet<Coordinate>) {
        let neighbours = self.get_plot_neighbours(coordinate);

        let (plots, perimeters): (Vec<_>, Vec<_>) = neighbours
            .into_iter()
            .map(|(neighbour, direction)| {
                if visited.contains(&neighbour) {
                    return (HashSet::new(), vec![]);
                }

                let Some(neighbour_plot) = self.get_plot(neighbour) else {
                    let perimeter = Perimeter::from(neighbour, direction);
                    return (HashSet::new(), vec![perimeter]);
                };

                if neighbour_plot != plot_type {
                    let perimeter = Perimeter::from(neighbour, direction);
                    return (HashSet::new(), vec![perimeter]);
                }

                visited.insert(neighbour);

                let (neighbour_region, visited_neighbours) =
                    self.clone()
                        .flood(neighbour, plot_type, visited.clone(), perimeters.clone());

                for visited_neighbour in visited_neighbours.iter() {
                    visited.insert(*visited_neighbour);
                }

                (neighbour_region.plots, neighbour_region.perimeters)
            })
            .unzip();

        let mut plots: HashSet<_> = plots.into_iter().flatten().collect();
        plots.insert(coordinate);

        let region = Region {
            name: plot_type,
            plots,
            perimeters: perimeters.into_iter().flatten().collect(),
        };
        (region, visited)
    }

    fn get_plot(&self, (x, y): Coordinate) -> Option<Plot> {
        if x.is_negative() || y.is_negative() {
            return None;
        }

        let (x, y) = (x as usize, y as usize);
        self.0.lines().nth(y).and_then(|line| line.chars().nth(x))
    }

    fn get_plot_neighbours(&self, (x, y): Coordinate) -> Vec<(Coordinate, Direction)> {
        [
            ((x + 1, y), Direction::Right),
            ((x, y - 1), Direction::Up),
            ((x - 1, y), Direction::Left),
            ((x, y + 1), Direction::Down),
        ]
        .to_vec()
    }
}

impl Region {
    // TODO: adapt to new implementation
    fn get_cost(self) -> usize {
        self.perimeters.len() * self.plots.len()
    }

    fn are_part_of_same_side(coordinate1: (i32, i32), coordinate2: (i32, i32)) -> bool {
        coordinate1.1 == coordinate2.1 && coordinate1.0 + 1 == coordinate2.0
    }

    fn get_sides(&self) -> (Vec<Vec<Perimeter>>, Vec<Vec<Perimeter>>) {
        let horizontal_perimeters = self.sort_sides_horizontally();
        let horizontals =
            self.group_sides(horizontal_perimeters, |previous_perimeter, perimeter| {
                let coordinate = perimeter.coordinate();
                let previous_coordinate = previous_perimeter.coordinate();

                (previous_coordinate.1 == coordinate.1 && previous_coordinate.0 + 1 == coordinate.0)
                    && !matches!(perimeter, Perimeter::Left(_))
            });

        let vertical_perimeters = self.sort_sides_vertically();
        let verticals = self.group_sides(vertical_perimeters, |previous_perimeter, perimeter| {
            let coordinate = perimeter.coordinate();
            let previous_coordinate = previous_perimeter.coordinate();

            (previous_coordinate.0 == coordinate.0 && previous_coordinate.1 + 1 == coordinate.1)
                && !matches!(perimeter, Perimeter::Bottom(_))
        });

        (horizontals, verticals)
    }

    fn get_corners(&self) -> impl Iterator<Item = Corner<'_>> {
        self.perimeters
            .iter()
            .flat_map(|perimeter| self.makes_up_corners(perimeter))
            .unique()
    }

    fn makes_up_corners<'a>(
        &'a self,
        perimeter: &'a Perimeter,
    ) -> impl Iterator<Item = Corner<'a>> {
        self.perimeters.iter().filter_map(move |p| {
            if perimeter.creates_corner(p) {
                Some(Corner(perimeter, p).clone())
            } else {
                None
            }
        })
    }

    fn count_sides(&self) -> usize {
        let (horizontals, verticals) = self.get_sides();
        dbg!(&verticals);
        dbg!(&horizontals);
        horizontals.len() + verticals.len()
    }

    fn group_sides<'a>(
        &self,
        mut perimeters: impl Iterator<Item = &'a Perimeter>,
        are_part_of_same_side: fn(&Perimeter, &Perimeter) -> bool,
    ) -> Vec<Vec<Perimeter>> {
        let (head, tail) = (perimeters.next().unwrap(), perimeters);

        tail.fold(vec![vec![head.clone()]], |mut acc, perimeter| {
            let last_side = acc.last_mut().expect("acc always has a last value");
            let previous_perimeter = last_side.last().expect("Always has a value");

            let coordinate = perimeter.coordinate();
            let previous_coordinate = previous_perimeter.coordinate();

            let is_part_of_same_side = are_part_of_same_side(previous_perimeter, perimeter)
                /* && !self.is_part_of_corner(perimeter) */;

            if is_part_of_same_side {
                // Same side
                last_side.push(perimeter.clone());
            } else {
                // New side
                acc.push(vec![perimeter.clone()]);
            }

            acc
        })
    }

    fn sort_sides_horizontally(&self) -> std::vec::IntoIter<&Perimeter> {
        self.perimeters
            .iter()
            .filter(|p| matches!(p, Perimeter::Top(_)) || matches!(p, Perimeter::Bottom(_)))
            .sorted_by(|&p1, &p2| {
                let (c1, c2) = (p1.coordinate(), p2.coordinate());
                c1.1.cmp(&c2.1).then_with(|| c1.0.cmp(&c2.0))
            })
    }

    fn sort_sides_vertically(&self) -> std::vec::IntoIter<&Perimeter> {
        self.perimeters
            .iter()
            .filter(|p| matches!(p, Perimeter::Left(_)) || matches!(p, Perimeter::Right(_)))
            .sorted_by(|&p1, &p2| {
                let (c1, c2) = (p1.coordinate(), p2.coordinate());
                c1.0.cmp(&c2.0).then_with(|| c1.1.cmp(&c2.1))
            })
    }

    // fn is_part_of_corner(&self, perimeter: &Perimeter) -> bool {
    //     self.perimeters.iter().any(|p| perimeter.opposite() == *p)
    // }
}

fn main() {
    let data = include_str!("../../data/day12");

    println!("Part 1: {}", sum_region_costs(data));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::{fmt::Debug, hash::Hash};

    fn assert_equal<T>(v1: Vec<T>, v2: Vec<T>)
    where
        T: Eq + Hash + Debug,
    {
        println!("{:?}", &v1);
        println!("{:?}", &v2);
        assert_eq!(v1.into_iter().counts(), v2.into_iter().counts());
    }

    #[test]
    fn gets_region() {
        let garden = indoc! {"
            AAAA
            BBCD
            BBCC
            EEEC
        "};
        let garden = Garden(garden.to_string());

        // E
        let region = garden.clone().get_region((0, 3));
        assert_eq!(HashSet::from([(0, 3), (1, 3), (2, 3)]), region.plots);
        assert_equal(
            vec![
                Perimeter::Left((-1, 3)),
                Perimeter::Top((0, 2)),
                Perimeter::Top((1, 2)),
                Perimeter::Top((2, 2)),
                Perimeter::Right((3, 3)),
                Perimeter::Bottom((2, 4)),
                Perimeter::Bottom((1, 4)),
                Perimeter::Bottom((0, 4)),
            ],
            region.perimeters,
        );

        // A
        let expected = HashSet::from([(0, 0), (1, 0), (2, 0), (3, 0)]);
        let result = garden.clone().get_region((0, 0));
        assert_eq!(expected, result.plots);

        // C
        let expected = vec![
            Perimeter::Top((2, 0)),
            Perimeter::Right((3, 1)),
            Perimeter::Top((3, 1)),
            Perimeter::Right((4, 2)),
            Perimeter::Right((4, 3)),
            Perimeter::Bottom((3, 4)),
            Perimeter::Left((2, 3)),
            Perimeter::Bottom((2, 3)),
            Perimeter::Left((1, 2)),
            Perimeter::Left((1, 1)),
        ];
        let result = garden.get_region((2, 1));
        assert_equal(expected, result.perimeters);
    }

    #[test]
    fn gets_dislocated_region() {
        let garden = indoc! {"
            OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO
        "};
        let garden = Garden(garden.to_string());
        let regions = garden.get_regions().collect_vec().into_iter();

        let x_perimeters: Vec<_> = regions
            .clone()
            .filter(|r| r.name == 'X')
            .flat_map(|r| r.perimeters)
            .collect();

        let expected = vec![
            Perimeter::Top((1, 0)),
            Perimeter::Right((2, 1)),
            Perimeter::Left((2, 1)),
            Perimeter::Bottom((1, 2)),
            Perimeter::Top((1, 2)),
            Perimeter::Left((0, 1)),
            Perimeter::Top((3, 0)),
            Perimeter::Right((4, 1)),
            Perimeter::Bottom((3, 2)),
            Perimeter::Top((3, 2)),
            Perimeter::Left((0, 3)),
            Perimeter::Bottom((1, 4)),
            Perimeter::Left((2, 3)),
            Perimeter::Right((2, 3)),
            Perimeter::Bottom((3, 4)),
            Perimeter::Right((4, 3)),
        ];
        assert_equal(expected, x_perimeters);

        let y_perimeters = regions.clone().find(|region| region.name == 'O').unwrap();
        let expected = vec![
            Perimeter::Top((0, -1)),
            Perimeter::Top((1, -1)),
            Perimeter::Top((2, -1)),
            Perimeter::Top((3, -1)),
            Perimeter::Top((4, -1)),
            //
            Perimeter::Right((5, 0)),
            Perimeter::Right((5, 1)),
            Perimeter::Right((5, 2)),
            Perimeter::Right((5, 3)),
            Perimeter::Right((5, 4)),
            //
            Perimeter::Bottom((4, 5)),
            Perimeter::Bottom((3, 5)),
            Perimeter::Bottom((2, 5)),
            Perimeter::Bottom((1, 5)),
            Perimeter::Bottom((0, 5)),
            //
            Perimeter::Left((-1, 4)),
            Perimeter::Left((-1, 3)),
            Perimeter::Left((-1, 2)),
            Perimeter::Left((-1, 1)),
            Perimeter::Left((-1, 0)),
            //
            Perimeter::Top((1, 1)),
            Perimeter::Right((1, 1)),
            Perimeter::Bottom((1, 1)),
            Perimeter::Left((1, 1)),
            //
            Perimeter::Top((3, 1)),
            Perimeter::Right((3, 1)),
            Perimeter::Bottom((3, 1)),
            Perimeter::Left((3, 1)),
            //
            Perimeter::Top((1, 3)),
            Perimeter::Right((1, 3)),
            Perimeter::Bottom((1, 3)),
            Perimeter::Left((1, 3)),
            //
            Perimeter::Top((3, 3)),
            Perimeter::Right((3, 3)),
            Perimeter::Bottom((3, 3)),
            Perimeter::Left((3, 3)),
        ];
        assert_equal(expected, y_perimeters.perimeters);
    }

    #[test]
    fn gets_region_cost() {
        let garden = indoc! {"
            AAAA
            BBCD
            BBCC
            EEEC
        "};
        let garden = Garden(garden.to_string());

        let cost = garden.clone().get_region((0, 0)).get_cost();
        assert_eq!(40, cost);

        let cost = garden.clone().get_region((0, 1)).get_cost();
        assert_eq!(32, cost);

        let cost = garden.clone().get_region((2, 1)).get_cost();
        assert_eq!(40, cost);

        let cost = garden.clone().get_region((3, 1)).get_cost();
        assert_eq!(4, cost);

        let cost = garden.clone().get_region((0, 3)).get_cost();
        assert_eq!(24, cost);
    }

    #[test]
    fn gets_larger_region_cost() {
        let garden = indoc! {"
            OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO
        "};
        let garden = Garden(garden.to_string());

        let cost = garden.clone().get_region((0, 0)).get_cost();
        assert_eq!(756, cost);

        let cost = garden.clone().get_region((1, 1)).get_cost();
        assert_eq!(4, cost);
    }

    #[test]
    fn sums_region_costs() {
        let garden = indoc! {"
            AAAA
            BBCD
            BBCC
            EEEC
        "};

        let cost = sum_region_costs(garden);
        assert_eq!(140, cost);
    }

    #[test]
    fn sums_larger_region_costs() {
        let garden = indoc! {"
            OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO
        "};

        let cost = sum_region_costs(garden);
        assert_eq!(772, cost);
    }

    #[test]
    fn sums_largest_region_costs() {
        let garden = indoc! {"
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
        "};

        let cost = sum_region_costs(garden);
        assert_eq!(1930, cost);
    }

    #[test]
    fn gets_diagonally_touching_regions() {
        let garden = indoc! {"
            XXXX
            XX.X
            X.XX
            XXXX
        "};

        let garden = Garden(garden.to_string());
        let regions = garden.get_regions().collect_vec().into_iter();
        let x_region = regions.clone().find(|region| region.name == 'X').unwrap();

        let expected_horizontals = vec![
            vec![
                Perimeter::Top((0, -1)),
                Perimeter::Top((1, -1)),
                Perimeter::Top((2, -1)),
                Perimeter::Top((3, -1)),
            ],
            //
            vec![
                Perimeter::Bottom((0, 4)),
                Perimeter::Bottom((1, 4)),
                Perimeter::Bottom((2, 4)),
                Perimeter::Bottom((3, 4)),
            ],
            //
            //
            vec![Perimeter::Top((2, 1))],
            vec![Perimeter::Bottom((2, 1))],
            //
            vec![Perimeter::Top((1, 2))],
            vec![Perimeter::Bottom((1, 2))],
        ];
        let (horizontals, verticals) = x_region.get_sides();
        assert_equal(expected_horizontals, horizontals);

        let expected_verticals = vec![
            vec![
                Perimeter::Right((4, 0)),
                Perimeter::Right((4, 1)),
                Perimeter::Right((4, 2)),
                Perimeter::Right((4, 3)),
            ],
            //
            vec![
                Perimeter::Left((-1, 0)),
                Perimeter::Left((-1, 1)),
                Perimeter::Left((-1, 2)),
                Perimeter::Left((-1, 3)),
            ],
            //
            vec![Perimeter::Left((1, 2))],
            vec![Perimeter::Right((1, 2))],
            //
            vec![Perimeter::Left((2, 1))],
            vec![Perimeter::Right((2, 1))],
        ];
        assert_equal(expected_verticals, verticals);
    }

    #[test]
    fn sums_diagonally_touching_regions_smaller() {
        let garden = indoc! {"
            XXXX
            XX.X
            X.XX
            XXXX
        "};

        assert_eq!(12 * 14 + 4 + 4, sum_discounted_region_costs(garden));
    }

    #[test]
    fn counts_diagonally_touching_region_discounted_costs() {
        let garden = indoc! {"
            AAAAAA
            AAABBA
            AAABBA
            ABBAAA
            ABBAAA
            AAAAAA
        "};

        let garden = Garden(garden.to_string());
        // let a_region = garden.clone().get_region((0, 0));
        // assert_eq!(12, a_region.count_sides());

        let b_region = garden.clone().get_region((3, 1));
        assert_eq!(8, b_region.count_sides());
    }

    #[test]
    fn counts_corners() {
        // TODO: Each B is counted twice
        let garden = indoc! {"
            AAAAAA
            AAABBA
            AAABBA
            ABBAAA
            ABBAAA
            AAAAAA
        "};

        let garden = Garden(garden.to_string());
        let a_region = garden.clone().get_region((0, 0));
        println!("perimeters: {:?}", a_region.perimeters);
        let corners = a_region.get_corners().collect_vec();
        println!("{:?}", corners);
        assert_eq!(12, corners.len());

        // let b_region = garden.clone().get_region((3, 1));
        // assert_eq!(368, cost);
    }

    #[test]
    fn sums_diagonally_touching_region_discounted_costs() {
        let garden = indoc! {"
            AAAAAA
            AAABBA
            AAABBA
            ABBAAA
            ABBAAA
            AAAAAA
        "};

        let cost = sum_discounted_region_costs(garden);
        // (4+8) * (6 * 6 - 8) + (4*4*2)
        assert_eq!(368, cost);
    }

    #[test]
    fn groups_horizontal_sides() {
        let garden = indoc! {"
            AAAA
            BBCD
            BBCC
            EEEC
        "};
        let garden = Garden(garden.to_string());
        let a_region = garden.clone().get_region((0, 0));
        let sides = a_region.sort_sides_horizontally();

        let expected_horizontals = vec![
            vec![
                Perimeter::Top((0, -1)),
                Perimeter::Top((1, -1)),
                Perimeter::Top((2, -1)),
                Perimeter::Top((3, -1)),
            ],
            vec![
                Perimeter::Bottom((0, 1)),
                Perimeter::Bottom((1, 1)),
                Perimeter::Bottom((2, 1)),
                Perimeter::Bottom((3, 1)),
            ],
        ];

        let expected_verticals = vec![
            vec![Perimeter::Left((-1, 0))],
            vec![Perimeter::Right((4, 0))],
        ];
        let (horizontals, verticals) = a_region.get_sides();

        assert_equal(expected_horizontals, horizontals);
        assert_equal(expected_verticals, verticals);
    }

    // #[test]
    // fn groups_vertical_sides() {
    //     let garden = indoc! {"
    //         AAAA
    //         BBCD
    //         BBCC
    //         EEEC
    //     "};
    //     let garden = Garden(garden.to_string());
    //     let a_region = garden.clone().get_region((0, 0));
    //     let sides = a_region.sort_sides_vertically();

    //     let expected = vec![vec![(-1, 0)], vec![(4, 0)]];

    //     assert_equal(expected, Region::group_vertical_sides(sides));
    // }

    #[test]
    fn gets_perimeters() {
        let garden = indoc! {"
            AAAA
            BBCD
            BBCC
            EEEC
        "};
        let garden = Garden(garden.to_string());

        let perimeters = garden.clone().get_region((0, 0)).perimeters;
        assert_equal(
            vec![
                Perimeter::Left((-1, 0)),
                Perimeter::Top((0, -1)),
                Perimeter::Top((1, -1)),
                Perimeter::Top((2, -1)),
                Perimeter::Top((3, -1)),
                Perimeter::Right((4, 0)),
                Perimeter::Bottom((3, 1)),
                Perimeter::Bottom((2, 1)),
                Perimeter::Bottom((1, 1)),
                Perimeter::Bottom((0, 1)),
            ],
            perimeters,
        );
    }

    #[test]
    fn finds_outer_corners() {
        let p1 = Perimeter::Right((-1, 0));
        let p2 = Perimeter::Bottom((0, -1));

        assert!(p1.create_outer_corner(&p2));
    }

    #[test]
    fn gets_corners() {
        let garden = indoc! {"
            AAAA
            BBCD
            BBCC
            EEEC
        "};
        let garden = Garden(garden.to_string());

        let a_region = garden.clone().get_region((0, 0));

        assert_equal(
            vec![
                Corner(&Perimeter::Right((4, 0)), &Perimeter::Top((3, -1))),
                Corner(&Perimeter::Right((4, 0)), &Perimeter::Bottom((3, 1))),
                Corner(&Perimeter::Top((0, -1)), &Perimeter::Left((-1, 0))),
                Corner(&Perimeter::Left((-1, 0)), &Perimeter::Bottom((0, 1))),
            ],
            a_region.get_corners().collect_vec(),
        );
    }

    #[test]
    fn equates_corners() {
        let corner1 = Corner(&Perimeter::Top((1, 2)), &Perimeter::Right((1, 2)));
        let corner2 = Corner(&Perimeter::Right((1, 2)), &Perimeter::Top((1, 2)));

        assert_eq!(corner1, corner2);
    }

    #[test]
    fn creates_corner() {
        assert!(Perimeter::Top((1, 2)).creates_corner(&Perimeter::Right((1, 2))));

        assert!(!Perimeter::Top((1, 2)).creates_corner(&Perimeter::Bottom((1, 2))));
        assert!(!Perimeter::Top((1, 2)).creates_corner(&Perimeter::Right((2, 2))));
    }
}
