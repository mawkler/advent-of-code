mod part1 {
    pub fn solve_worksheet(worksheet: &str) -> u64 {
        let worksheet = worksheet.lines().map(|line| line.split_whitespace());
        let width = worksheet.clone().next().expect("has lines").count();

        (0..width)
            .map(|column| {
                worksheet
                    .clone()
                    .map(move |line| line.clone().nth(column).expect("`column` < `width`"))
            })
            .map(solve_column)
            .sum()
    }

    fn solve_column<'a>(mut columns: impl DoubleEndedIterator<Item = &'a str>) -> u64 {
        let operator = columns.next_back().expect("has multiple lines");
        let numbers = columns.map(|num| num.parse::<u64>().expect("is numeric"));

        let operator = match operator {
            "+" => Iterator::sum::<u64>,
            "*" => Iterator::product::<u64>,
            _ => panic!("unexpected operator '{operator}"),
        };
        operator(numbers)
    }
}

fn main() {
    let input = include_str!("../../input/day6");
    println!("Part 1: {}", part1::solve_worksheet(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_worksheet() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        assert_eq!(part1::solve_worksheet(input), 4277556);
    }
}
