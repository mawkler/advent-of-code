use itertools::Itertools;

type Id = u64;

fn parse_range(range: &str) -> (Id, Id) {
    let (lower, upper) = range.split_once('-').expect("range has `-`");
    let lower: Id = lower.parse().expect("is numeric");
    let upper: Id = upper.parse().expect("is numeric");
    (lower, upper)
}

fn is_invalid_dual(id: &str) -> bool {
    if !id.len().is_multiple_of(2) {
        return false;
    }

    let (left, right) = id.split_at(id.len() / 2);

    left == right
}

// Part 1
fn count_invalid_ids(str: &str) -> u64 {
    str.strip_suffix('\n')
        .unwrap_or(str)
        .split(',')
        .map(|range| -> Id {
            let (lower, upper) = parse_range(range);

            (lower..=upper)
                .filter(|id| is_invalid_dual(&id.to_string()))
                .sum()
        })
        .sum()
}

fn has_all_equal_substrings(str: &str, substring_size: usize) -> bool {
    str.chars()
        .chunks(substring_size)
        .into_iter()
        .map(Iterator::collect::<String>)
        .all_equal()
}

fn is_invalid_any_amount(id: &str) -> bool {
    (1..id.len()).any(|substring_size| has_all_equal_substrings(id, substring_size))
}

// Part 2
fn count_invalid_ids_any_amount(str: &str) -> u64 {
    str.strip_suffix('\n')
        .unwrap_or(str)
        .split(',')
        .map(|range| -> Id {
            let (lower, upper) = parse_range(range);

            (lower..=upper)
                .filter(|id| is_invalid_any_amount(&id.to_string()))
                .sum()
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input/day2");

    println!("Part 1: {}", count_invalid_ids(input));
    println!("Part 2: {}", count_invalid_ids_any_amount(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalidates_id_dual() {
        assert!(is_invalid_dual("1010"));
        assert!(!is_invalid_dual("1234"));
    }

    #[test]
    fn counts_invalid_ids() {
        let invalid_ids = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,\
        2121212118-2121212124";

        assert_eq!(count_invalid_ids(invalid_ids), 1227775554);
    }

    #[test]
    fn validates_all_equal_substrings() {
        assert!(has_all_equal_substrings("1010", 2));
        assert!(!has_all_equal_substrings("1010", 3));

        assert!(has_all_equal_substrings("123123123", 3));
        assert!(!has_all_equal_substrings("123123123", 2));
    }

    #[test]
    fn invalidates_invalid_ids() {
        assert!(is_invalid_any_amount("123123"));
        assert!(is_invalid_any_amount("99"));
    }

    #[test]
    fn counts_invalid_ids_any_amount() {
        let invalid_ids = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,\
        2121212118-2121212124";

        assert_eq!(count_invalid_ids_any_amount(invalid_ids), 4174379265);
    }
}
