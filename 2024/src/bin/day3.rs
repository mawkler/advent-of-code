use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

pub fn sum_multiplications(string: &str) -> u32 {
    parse_muls(string)
        .into_iter()
        .map(|mul| mul.0 * mul.1)
        .sum()
}

pub fn sum_multiplications_with_toggles(string: &str) -> u32 {
    parse_muls_with_toggles(string)
        .into_iter()
        .map(|mul| mul.0 * mul.1)
        .sum()
}

fn parse_mul(i: &str) -> IResult<&str, (u32, u32)> {
    preceded(
        tag("mul"),
        delimited(
            char('('),
            separated_pair(
                map_res(digit1, str::parse),
                char(','),
                map_res(digit1, str::parse),
            ),
            char(')'),
        ),
    )(i)
}

fn parse_muls(string: &str) -> Vec<(u32, u32)> {
    (0..string.len())
        .filter_map(|position| {
            if let Ok((_, mul)) = parse_mul(&string[position..]) {
                Some(mul)
            } else {
                None
            }
        })
        .collect()
}

fn parse_toggle(i: &str) -> IResult<&str, bool> {
    map(alt((tag("don't"), tag("do"))), |toggle| toggle == "do")(i)
}

fn parse_muls_with_toggles(string: &str) -> Vec<(u32, u32)> {
    let mut enabled = true;

    (0..string.len())
        .filter_map(|position| {
            let substring = &string[position..];

            if let Ok((_, toggle)) = parse_toggle(substring) {
                enabled = toggle;
                return None;
            }

            if !enabled {
                return None;
            }

            match parse_mul(substring) {
                Ok((_, mul)) => Some(mul),
                Err(_) => None,
            }
        })
        .collect()
}

fn main() {
    let data = include_str!("../../data/day3");

    println!("Part 1: {}", sum_multiplications(data));
    println!("Part 2: {}", sum_multiplications_with_toggles(data));
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn parses() {
        let memory = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_equal(
            vec![(2, 4), (5, 5), (11, 8), (8, 5)],
            parse_muls_with_toggles(memory),
        );
    }

    #[test]
    fn sums() {
        let memory = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(161, sum_multiplications(memory));
    }

    #[test]
    fn sums_with_toggles() {
        let memory = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(48, sum_multiplications_with_toggles(memory));
    }
}
