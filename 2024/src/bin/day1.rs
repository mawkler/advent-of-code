pub fn sum_distances(input: &str) -> i32 {
    let (mut lefts, mut rights) = parse_columns(input);

    lefts.sort();
    rights.sort();

    lefts
        .into_iter()
        .zip(rights)
        .map(|(left, right)| (left - right).abs())
        .sum()
}

pub fn sum_similarities(input: &str) -> i32 {
    let (lefts, rights) = parse_columns(input);

    lefts
        .into_iter()
        .map(|left| {
            let count = rights.iter().filter(|&right| left == *right).count();
            left * count as i32
        })
        .sum()
}

fn parse_columns(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines: Vec<_> = input.lines().collect();
    let pairs: Vec<(i32, i32)> = lines
        .iter()
        .map(|line| {
            let pairs: Vec<_> = line.split_whitespace().take(2).collect();

            (pairs[0].parse().unwrap(), pairs[1].parse().unwrap())
        })
        .collect();

    pairs.into_iter().unzip()
}

fn main() {
    let data = include_str!("../../data/day1");

    println!("Part 1: {}", sum_distances(data));
    println!("Part 2: {}", sum_similarities(data));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "};

    #[test]
    fn sums_distances() {
        assert_eq!(sum_distances(INPUT), 11);
    }

    #[test]
    fn sums_similarities() {
        assert_eq!(sum_similarities(INPUT), 31);
    }
}
