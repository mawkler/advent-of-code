use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map_res,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

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

fn parse(string: &str) -> Vec<(u32, u32)> {
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

fn sum_multiplications(string: &str) -> u32 {
    parse(string).into_iter().map(|m| m.0 * m.1).sum()
}

fn main() {
    let data = include_str!("../../data/day3");

    println!("Part 1: {}", sum_multiplications(data));
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    const MEMORY: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn parses() {
        let result = parse(MEMORY);
        assert_equal(vec![(2, 4), (5, 5), (11, 8), (8, 5)], result);
    }
}
