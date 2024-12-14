use itertools::Itertools;

struct Map(String);

type Coordinate = (i32, i32);
type Trailhead = Vec<Coordinate>;

// Part 1
pub fn sum_trailhead_scores(map: &str) -> usize {
    let map = Map(map.to_string());
    let height = map.0.len();
    let width = map.0.lines().next().expect("Has lines").len();

    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    let (x, y) = (x.try_into().unwrap(), y.try_into().unwrap());
                    let Some(current_tile) = map.get_tile((x, y)) else {
                        return 0;
                    };

                    if current_tile != 0 {
                        return 0;
                    }

                    map.get_unique_trailheads((x, y)).count()
                })
                .sum::<usize>()
        })
        .sum()
}

// Part 2
pub fn sum_trailhead_ratings(map: &str) -> usize {
    let map = Map(map.to_string());
    let height = map.0.len();
    let width = map.0.lines().next().expect("Has lines").len();

    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    let (x, y) = (x.try_into().unwrap(), y.try_into().unwrap());
                    let Some(current_tile) = map.get_tile((x, y)) else {
                        return 0;
                    };

                    if current_tile != 0 {
                        return 0;
                    }

                    // The only difference to Part 1 is that we don't filter here
                    map.get_trailheads((x, y)).len()
                })
                .sum::<usize>()
        })
        .sum()
}

impl Map {
    fn get_tile(&self, (x, y): Coordinate) -> Option<usize> {
        let tile = self.0.lines().nth(y as usize)?.chars().nth(x as usize);
        tile.and_then(|t| Some(t.to_digit(10)? as usize))
    }

    fn get_neighbours(&self, (x, y): Coordinate) -> impl Iterator<Item = Coordinate> {
        [(x + 1, y), (x, y - 1), (x - 1, y), (x, y + 1)].into_iter()
    }

    fn get_unique_trailheads(&self, tile: Coordinate) -> impl Iterator<Item = Trailhead> {
        self.get_trailheads(tile)
            .into_iter()
            .unique_by(|trailhead| trailhead.last().cloned())
    }

    fn get_trailheads(&self, tile: Coordinate) -> Vec<Trailhead> {
        let Some(current_tile) = self.get_tile(tile) else {
            return vec![];
        };

        if current_tile == 9 {
            return vec![vec![tile]];
        }

        self.get_neighbours(tile)
            .flat_map(|neighbour| match self.get_tile(neighbour) {
                Some(neighbour_tile) if neighbour_tile == current_tile + 1 => {
                    let trailheads: Vec<_> = self
                        .get_trailheads(neighbour)
                        .iter()
                        .map(|trailhead| {
                            let mut result = vec![tile];
                            result.extend(trailhead);
                            result
                        })
                        .collect();
                    Some(trailheads)
                }
                _ => None,
            })
            .flatten()
            .collect()
    }
}

fn main() {
    let data = include_str!("../../data/day10");

    println!("Part 1: {}", sum_trailhead_scores(data));
    println!("Part 2: {}", sum_trailhead_ratings(data));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use itertools::assert_equal;

    #[test]
    fn gets_tile() {
        let map = indoc! {"
            0123
            1234
            8765
            9876
        "};

        assert_eq!(Some(9), Map(map.to_string()).get_tile((0, 3)));
        assert_eq!(Some(2), Map(map.to_string()).get_tile((1, 1)));
        assert_eq!(Some(6), Map(map.to_string()).get_tile((2, 2)));

        assert_eq!(None, Map(map.to_string()).get_tile((-1, -1)));
        assert_eq!(None, Map(map.to_string()).get_tile((4, 0)));
        assert_eq!(None, Map(map.to_string()).get_tile((0, 4)));
    }

    #[test]
    fn gets_trailheads() {
        let map = indoc! {"
            ...0...
            ...1...
            ...2...
            6543456
            7.....7
            8.....8
            9.....9
        "};

        let expected = vec![
            vec![
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (4, 3),
                (5, 3),
                (6, 3),
                (6, 4),
                (6, 5),
                (6, 6),
            ],
            vec![
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (2, 3),
                (1, 3),
                (0, 3),
                (0, 4),
                (0, 5),
                (0, 6),
            ],
        ];

        assert_equal(expected, Map(map.to_string()).get_trailheads((3, 0)));
    }

    #[test]
    fn gets_intertwining_trailheads() {
        let map = indoc! {"
            10..9..
            2...8..
            3...7..
            4567654
            ...8..3
            ...9..2
            .....01
        "};
        let map = Map(map.to_string());

        let expected = vec![vec![
            (1, 0),
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 3),
            (2, 3),
            (3, 3),
            (3, 4),
            (3, 5),
        ]];
        assert_equal(expected, map.get_trailheads((1, 0)));

        let expected = vec![
            vec![
                (5, 6),
                (6, 6),
                (6, 5),
                (6, 4),
                (6, 3),
                (5, 3),
                (4, 3),
                (4, 2),
                (4, 1),
                (4, 0),
            ],
            vec![
                (5, 6),
                (6, 6),
                (6, 5),
                (6, 4),
                (6, 3),
                (5, 3),
                (4, 3),
                (3, 3),
                (3, 4),
                (3, 5),
            ],
        ];
        assert_equal(expected, map.get_trailheads((5, 6)));
    }

    #[test]
    fn sums_trailhead_scores() {
        let map = indoc! {"
            10..9..
            2...8..
            3...7..
            4567654
            ...8..3
            ...9..2
            .....01
        "};

        assert_eq!(3, sum_trailhead_scores(map));

        let map = indoc! {"
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
        "};

        assert_eq!(36, sum_trailhead_scores(map))
    }

    #[test]
    fn sums_trailhead_ratings_test() {
        let map = indoc! {"
            .....0.
            ..4321.
            ..5..2.
            ..6543.
            ..7..4.
            ..8765.
            ..9....
        "};

        assert_eq!(3, sum_trailhead_ratings(map));

        let map = indoc! {"
            ..90..9
            ...1.98
            ...2..7
            6543456
            765.987
            876....
            987....
        "};

        assert_eq!(13, sum_trailhead_ratings(map));
    }
}
