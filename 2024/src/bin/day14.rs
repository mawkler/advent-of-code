use itertools::Itertools;
use nom::{
    self,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt},
    sequence::{preceded, separated_pair, tuple},
};
use std::fmt::Display;

// Part 1
fn safety_factor_after_100_seconds(map: &str) -> usize {
    let mut map = Map::new(map, 101, 103);

    map.update(100);

    map.calculate_safety_factor()
}

// Part 2
fn find_christmas_tree(map: &str) -> usize {
    let mut map = Map::new(map, 101, 103);
    let mut count = 0;

    loop {
        map.update(1);
        count += 1;

        if map.has_christmas_tree() {
            println!("{map}");
            return count;
        }
    }
}

#[derive(Debug)]
struct Map {
    robots: Vec<Robot>,
    width: i32,
    height: i32,
}

type Vector = (i32, i32);

#[derive(Debug, PartialEq)]
struct Robot {
    position: Vector,
    velocity: Vector,
}

impl Robot {
    fn move_position(&mut self, max_width: i32, max_height: i32) {
        let (x, y) = (&mut self.position.0, &mut self.position.1);

        *x = positive_modulo(*x + self.velocity.0, max_width);
        *y = positive_modulo(*y + self.velocity.1 + max_height, max_height);
    }
}

impl Map {
    fn new(robots: &str, width: i32, height: i32) -> Self {
        Map {
            robots: parse_robots(robots).collect(),
            width,
            height,
        }
    }

    fn count_robots(&self, x: i32, y: i32) -> usize {
        self.robots.iter().filter(|r| r.position == (x, y)).count()
    }

    fn draw_tile(&self, y: i32, x: i32) -> String {
        let count = self.count_robots(x, y);
        if count > 0 {
            count.to_string()
        } else {
            ".".to_string()
        }
    }

    fn update(&mut self, times: u32) {
        for _ in 0..times {
            for r in self.robots.iter_mut() {
                r.move_position(self.width, self.height)
            }
        }
    }

    fn calculate_safety_factor(&self) -> usize {
        let half_width = self.width / 2;
        let half_height = self.height / 2;

        let quadrants = [
            iter_quadrant(0, half_width, 0, half_height),
            iter_quadrant(half_width + 1, self.width, 0, half_height),
            iter_quadrant(half_width + 1, self.width, half_height + 1, self.height),
            iter_quadrant(0, half_width, half_height + 1, self.height),
        ];

        quadrants
            .into_iter()
            .map(|q| q.map(|(x, y)| self.count_robots(x, y)).sum())
            .reduce(|acc, q| acc * q)
            .unwrap()
    }

    fn has_christmas_tree(&self) -> bool {
        (0..self.height).step_by(3).any(|y| {
            (0..self.width)
                .step_by(2)
                .any(|x| self.is_christmas_tree(x, y))
        })
    }

    fn is_christmas_tree(&self, x: i32, y: i32) -> bool {
        [
            (x, y),
            (x + 1, y),
            (x, y + 1),
            (x + 1, y + 1),
            (x, y + 2),
            (x + 1, y + 2),
            (x, y + 3),
            (x + 1, y + 3),
        ]
        .iter()
        .all(|(x, y)| self.count_robots(*x, *y) > 0)
    }
}

fn iter_quadrant(
    x_from: i32,
    x_to: i32,
    y_from: i32,
    y_to: i32,
) -> impl Iterator<Item = (i32, i32)> {
    (y_from..y_to).flat_map(move |y| (x_from..x_to).map(move |x| (x, y)))
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map = (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(move |x| self.draw_tile(y, x))
                    .collect::<String>()
            })
            .join("\n");

        write!(f, "{map}")
    }
}

fn positive_modulo(n: i32, modulus: i32) -> i32 {
    ((n % modulus) + modulus) % modulus
}

fn parse_robots(robots: &str) -> impl Iterator<Item = Robot> + use<'_> {
    robots.lines().map(|robot| {
        let (_, (position, velocity)) = parse_robot(robot).unwrap();

        Robot { position, velocity }
    })
}

fn parse_robot(i: &str) -> nom::IResult<&str, (Vector, Vector)> {
    fn parse_vector(i: &str) -> nom::IResult<&str, (i32, i32)> {
        separated_pair(parse_signed_number, tag(","), parse_signed_number)(i)
    }

    fn parse_signed_number(i: &str) -> nom::IResult<&str, i32> {
        map_res(tuple((opt(tag("-")), digit1)), |(sign, n)| {
            format!("{}{n}", sign.unwrap_or_default()).parse()
        })(i)
    }

    separated_pair(
        preceded(tag("p="), parse_vector),
        tag(" "),
        preceded(tag("v="), parse_vector),
    )(i)
}

fn main() {
    let data = include_str!("../../data/day14");
    println!("Part 1: {}", safety_factor_after_100_seconds(data));
    println!("Part 2: {}", find_christmas_tree(data));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use itertools::{assert_equal, Itertools};

    #[test]
    fn parses_robots() {
        let robots = indoc! {"
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
        "};

        let expected = [
            Robot {
                position: (0, 4),
                velocity: (3, -3),
            },
            Robot {
                position: (6, 3),
                velocity: (-1, -3),
            },
        ];
        assert_equal(expected, parse_robots(robots).collect_vec());
    }

    #[test]
    fn draws_map() {
        let robots = indoc! {"
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
        "};
        let map = Map::new(robots, 11, 7);

        let expected = indoc! {"
            1.12.......
            ...........
            ...........
            ......11.11
            1.1........
            .........1.
            .......1..."
        };
        assert_eq!(expected, map.to_string())
    }

    #[test]
    fn moves_robot() {
        let mut map = Map::new("p=2,4 v=2,-3", 11, 7);

        map.update(1);

        assert_eq!(
            &Robot {
                position: (4, 1),
                velocity: (2, -3)
            },
            map.robots.first().unwrap()
        );
    }

    #[test]
    fn wraps_robot_around_edge() {
        let mut map = Map::new("p=4,1 v=2,-3", 11, 7);

        map.update(1);

        let expected = indoc! {"
            ...........
            ...........
            ...........
            ...........
            ...........
            ......1....
            ..........."
        };
        assert_eq!(expected, map.to_string());

        map.calculate_safety_factor();
    }

    #[test]
    fn moves_robots_100_times() {
        let robots = indoc! {"
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
        "};
        let mut map = Map::new(robots, 11, 7);

        map.update(100);

        let expected = indoc! {"
            ......2..1.
            ...........
            1..........
            .11........
            .....1.....
            ...12......
            .1....1...."
        };
        assert_eq!(expected, map.to_string());
    }

    #[test]
    fn calculates_safety_factor() {
        let robots = indoc! {"
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
        "};
        let mut map = Map::new(robots, 11, 7);

        map.update(100);

        assert_eq!(12, map.calculate_safety_factor());
    }
}
