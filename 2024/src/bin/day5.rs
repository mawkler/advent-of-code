type Page = u32;

pub fn sum_middle_page_numbers(input: &str) -> u32 {
    get_valid_page_lines(input)
        .map(|line| {
            let middle = line.len() / 2;
            line[middle]
        })
        .sum()
}

pub fn get_valid_page_lines(input: &str) -> impl Iterator<Item = Vec<Page>> {
    let (ordering_rules, valid_pages) = parse(input);
    valid_pages
        .into_iter()
        .filter(move |pages_line| is_valid_page_line(pages_line, &ordering_rules))
}

fn parse(input: &str) -> (Vec<(Page, Page)>, Vec<Vec<Page>>) {
    let (ordering_rules, pages) = input.split_once("\n\n").expect("Input has correct format");
    let ordering_rules: Vec<_> = ordering_rules
        .lines()
        .map(|rule| {
            let (before, after) = rule.split_once("|").unwrap();
            let before = before.parse().expect("Must be numeric");
            let after = after.parse().expect("Must be numeric");
            (before, after)
        })
        .collect();

    let valid_page_lines = pages
        .lines()
        .map(|line| {
            line.split(",")
                .map(|page| page.parse().expect("Must be numeric"))
                .collect()
        })
        .collect();

    (ordering_rules, valid_page_lines)
}

fn is_valid_page_line(pages_line: &[Page], ordering_rules: &[(Page, Page)]) -> bool {
    pages_line
        .iter()
        .enumerate()
        .map(|(page_index, &page)| {
            let mut relevant_page_rules = ordering_rules
                .iter()
                .filter(|(before, after)| *after == page);

            let is_valid = relevant_page_rules.all(|(before, after)| {
                let mut trailing_pages = pages_line.iter().skip(page_index + 1);

                !trailing_pages.any(|&page| page == *before)
            });
            is_valid
        })
        .all(|valid_page| valid_page)
}

fn main() {
    let data = include_str!("../../data/day5");

    println!("Part 1: {}", sum_middle_page_numbers(data));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use itertools::assert_equal;

    const INPUT: &str = indoc! {"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
        "};

    #[test]
    fn invalidates_invalid_line() {
        let (ordering_rules, page_lines) = parse(INPUT);
        let invalid_line = page_lines.into_iter().nth(3).unwrap();
        dbg!(&invalid_line);

        assert!(!is_valid_page_line(&invalid_line, &ordering_rules))
    }

    #[test]
    fn gets_valid_page_lines() {
        let valid_pages = get_valid_page_lines(INPUT);

        let expected = [
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
        ];
        assert_equal(expected, valid_pages);
    }

    #[test]
    fn sums_middle_page_numbers() {
        assert_eq!(143, sum_middle_page_numbers(INPUT));
    }
}
