type Id = u64;

fn parse_range(range: &str) -> (Id, Id) {
    let (lower, upper) = range.split_once('-').expect("range has `-`");
    let lower: Id = lower.parse().expect("is numeric");
    dbg!(&upper);
    let upper: Id = upper.parse().expect("is numeric");
    (lower, upper)
}

fn count_invalid_ids(str: &str) -> Id {
    str.strip_suffix('\n')
        .unwrap_or(str)
        .split(',')
        .map(|range| -> Id {
            let (lower, upper) = parse_range(range);

            (lower..=upper)
                .filter(|id| is_invalid(&id.to_string()))
                .sum()
        })
        .sum()
}

fn is_invalid(id: &str) -> bool {
    if !id.len().is_multiple_of(2) {
        return false;
    }

    let (left, right) = id.split_at(id.len() / 2);

    left == right
}

fn main() {
    let input = include_str!("../../input/day2");

    println!("Part 1: {}", count_invalid_ids(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalidates_id() {
        assert!(is_invalid("1010"));
        assert!(!is_invalid("1234"));
    }

    #[test]
    fn counts_invalid_ids() {
        let invalid_ids = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,\
        2121212118-2121212124";
        dbg!(&invalid_ids);

        assert_eq!(count_invalid_ids(invalid_ids), 1227775554);
    }
}
