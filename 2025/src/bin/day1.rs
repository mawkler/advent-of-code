#![feature(assert_matches)]

const DIAL_NUMBERS: i16 = 100;

type Distance = u16;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left = -1,
    Right = 1,
}

impl From<&str> for Direction {
    fn from(direction: &str) -> Self {
        match direction {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid direction '{direction}'"),
        }
    }
}

fn parse_rotation(rotation: &str) -> (Direction, Distance) {
    let (direction, distance) = rotation.split_at(1);
    let distance: Distance = distance
        .parse()
        .unwrap_or_else(|_| panic!("invalid distance '{distance}'"));

    (direction.into(), distance)
}

fn count_zeroes(data: &str) -> u32 {
    let rotations: Vec<_> = data.lines().map(parse_rotation).collect();

    let (_, zeroes) = rotations
        .iter()
        .fold((50, 0), |(number, zeroes), &(direction, distance)| {
            let new_number = (number + direction as i16 * distance as i16).rem_euclid(DIAL_NUMBERS);
            let zeroes = match new_number {
                0 => zeroes + 1,
                _ => zeroes,
            };

            (new_number, zeroes)
        });

    zeroes
}

fn main() {
    let data = include_str!("../../data/day1");

    let zeroes = count_zeroes(data);
    println!("Zeroes: {zeroes}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn parses_rotation() {
        let rotation = "R29";
        assert_matches!(parse_rotation(rotation), (Direction::Right, 29))
    }

    #[test]
    fn rotations() {
        let rotations = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(count_zeroes(rotations), 3);
    }
}
