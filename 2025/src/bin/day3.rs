fn maximum_joltage(battery_bank: &str) -> u32 {
    (1..battery_bank.len())
        .map(|pivot| {
            let (left, right) = battery_bank.split_at(pivot);
            let left_number = left.chars().last().expect("string is long enough");

            right
                .chars()
                .map(move |right_number| -> u32 {
                    format!("{left_number}{right_number}")
                        .parse()
                        .expect("both characters are numeric")
                })
                .max()
                .expect("iterator is non-empty")
        })
        .max()
        .expect("length of `battery_bank` > 1")
}

fn total_joltage(battery_banks: &str) -> u32 {
    battery_banks.lines().map(maximum_joltage).sum()
}

fn main() {
    let input = include_str!("../../input/day3");

    println!("Part 1: {}", total_joltage(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_maximum_joltage() {
        assert_eq!(maximum_joltage("987654321111111"), 98);
        assert_eq!(maximum_joltage("811111111111119"), 89);
        assert_eq!(maximum_joltage("234234234234278"), 78);
        assert_eq!(maximum_joltage("818181911112111"), 92);
    }
}
