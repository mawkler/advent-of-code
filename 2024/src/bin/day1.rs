fn sum(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();
    let pairs: Vec<(i32, i32)> = lines
        .iter()
        .map(|line| {
            let pairs: Vec<_> = line.split_whitespace().take(2).collect();

            (pairs[0].parse().unwrap(), pairs[1].parse().unwrap())
        })
        .collect();

    let (mut lefts, mut rights): (Vec<i32>, Vec<i32>) = pairs.into_iter().unzip();

    lefts.sort();
    rights.sort();

    lefts
        .into_iter()
        .zip(rights)
        .inspect(|f| {})
        .map(|(left, right)| (left - right).abs())
        .sum()
}

pub fn main() {
    let data = include_str!("../../data/day1");

    let sum = sum(data);
    dbg!(&sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "};
        assert_eq!(sum(input), 11);
    }
}
