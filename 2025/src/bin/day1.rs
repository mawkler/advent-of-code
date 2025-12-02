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

fn count_passed_zeroes(start_number: i16, clicks: i16) -> i16 {
    let direction = clicks.signum(); // -1, 0, or 1

    // Couldn't get the math to math so we just check all the positions lol
    (1..=clicks.abs())
        .map(|click| (start_number + click * direction).rem_euclid(DIAL_NUMBERS))
        .filter(|&pos| pos == 0)
        .count() as i16
}

fn count_any_zeroes(data: &str) -> u32 {
    let rotations: Vec<_> = data.lines().map(parse_rotation).collect();

    let (_, zeroes) = rotations
        .iter()
        .fold((50, 0), |(number, zeroes), &(direction, distance)| {
            let clicks = direction as i16 * distance as i16;
            let passed_zeroes = count_passed_zeroes(number, clicks);
            let new_number = (number + clicks).rem_euclid(DIAL_NUMBERS);

            (new_number, zeroes + passed_zeroes)
        });

    zeroes as u32
}

fn main() {
    let data = include_str!("../../data/day1");

    let zeroes = count_zeroes(data);
    println!("Part 1: {zeroes}");

    let passed_zeroes = count_any_zeroes(data);
    println!("Part 2: {passed_zeroes}");
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

    #[test]
    fn counts_passed_zeroes() {
        assert_eq!(count_passed_zeroes(50, 100), 1);
        assert_eq!(count_passed_zeroes(50, 50), 1);
        assert_eq!(count_passed_zeroes(0, 100), 1);
        assert_eq!(count_passed_zeroes(1, 99), 1);
        assert_eq!(count_passed_zeroes(1, 98), 0);
        assert_eq!(count_passed_zeroes(1, -2), 1);
        assert_eq!(count_passed_zeroes(0, -2), 1);
        assert_eq!(count_passed_zeroes(3, -3), 1);
    }

    #[test]
    fn counts_any_zeroes() {
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

        assert_eq!(count_any_zeroes(rotations), 6);
    }
}
