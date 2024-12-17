use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Coordinate = (i32, i32);
type Plot = char;

#[derive(Clone)]
struct Garden(String);

#[derive(Debug, PartialEq, Clone)]
struct Region {
    name: char,
    plots: HashSet<Coordinate>,
    perimeters: Vec<Coordinate>,
}

// Part 1
fn sum_region_costs(garden: &str) -> usize {
    let garden = Garden(garden.to_string());
    garden.get_regions().map(Region::get_cost).sum()
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
            .map(|neighbour| {
                if visited.contains(&neighbour) {
                    return ([].into(), [].into());
                }

                let Some(neighbour_plot) = self.get_plot(neighbour) else {
                    return ([].into(), [neighbour].into());
                };

                if neighbour_plot != plot_type {
                    return ([].into(), [neighbour].into());
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

        let perimeters = perimeters.into_iter().flatten().collect();
        let mut plots: HashSet<_> = plots.into_iter().flatten().collect();
        plots.insert(coordinate);

        let region = Region {
            name: plot_type,
            plots,
            perimeters,
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

    fn get_plot_neighbours(&self, (x, y): Coordinate) -> Vec<Coordinate> {
        [(x + 1, y), (x, y - 1), (x - 1, y), (x, y + 1)].to_vec()
    }
}

impl Region {
    fn get_cost(self) -> usize {
        self.perimeters.len() * self.plots.len()
    }

    fn merge_regions_by_name(regions: HashMap<char, Vec<Region>>) -> impl Iterator<Item = Region> {
        regions.into_iter().map(|(name, regions)| {
            let plots = regions.iter().flat_map(|region| region.plots.clone());
            let perimeters = regions.iter().flat_map(|region| region.perimeters.clone());

            Region {
                name,
                plots: plots.collect(),
                perimeters: perimeters.collect(),
            }
        })
    }
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
                (-1, 3),
                (0, 2),
                (1, 2),
                (2, 2),
                (3, 3),
                (2, 4),
                (1, 4),
                (0, 4),
            ],
            region.perimeters,
        );

        // A
        let expected = HashSet::from([(0, 0), (1, 0), (2, 0), (3, 0)]);
        let result = garden.clone().get_region((0, 0));

        // C
        let expected = vec![
            (2, 0),
            (3, 1),
            (3, 1),
            (4, 2),
            (4, 3),
            (3, 4),
            (2, 3),
            (2, 3),
            (1, 2),
            (1, 1),
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

        let x_regions: Vec<_> = regions
            .clone()
            .filter(|r| r.name == 'X')
            .flat_map(|r| r.perimeters)
            .collect();

        let expected = vec![
            (1, 0),
            (2, 1),
            (2, 1),
            (1, 2),
            (1, 2),
            (0, 1),
            (3, 0),
            (4, 1),
            (3, 2),
            (3, 2),
            (0, 3),
            (1, 4),
            (2, 3),
            (2, 3),
            (3, 4),
            (4, 3),
        ];
        assert_equal(expected, x_regions);

        let y_region = regions.clone().find(|region| region.name == 'O').unwrap();
        let expected = vec![
            (0, -1),
            (1, -1),
            (2, -1),
            (3, -1),
            (4, -1),
            //
            (5, 0),
            (5, 1),
            (5, 2),
            (5, 3),
            (5, 4),
            //
            (4, 5),
            (3, 5),
            (2, 5),
            (1, 5),
            (0, 5),
            //
            (-1, 4),
            (-1, 3),
            (-1, 2),
            (-1, 1),
            (-1, 0),
            //
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            //
            (3, 1),
            (3, 1),
            (3, 1),
            (3, 1),
            //
            (1, 3),
            (1, 3),
            (1, 3),
            (1, 3),
            //
            (3, 3),
            (3, 3),
            (3, 3),
            (3, 3),
        ];
        assert_equal(expected, y_region.perimeters);
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
}
