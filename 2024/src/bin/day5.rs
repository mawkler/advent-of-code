use std::{
    cmp::{self},
    fmt::Debug,
    rc::Rc,
};

// Part 1
pub fn sum_middle_page_numbers(input: &str) -> u32 {
    get_valid_page_lines(input)
        .map(|line| get_line_middle(&line).number)
        .sum()
}

// Part 2
pub fn sum_middle_of_sorted_page_numbers(input: &str) -> u32 {
    get_invalid_page_lines(input)
        .map(|mut line| {
            sort(&mut line);
            get_line_middle(&line).number
        })
        .sum()
}

#[derive(Clone)]
struct Page {
    number: u32,
    rules: Rc<Vec<(u32, u32)>>,
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl Debug for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Page")
            .field("number", &self.number)
            .finish()
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let self_is_less = self
            .rules
            .iter()
            .find(|(before, after)| *before == self.number && *after == other.number);

        if self_is_less.is_some() {
            return Some(cmp::Ordering::Less);
        }

        let other_is_less = self
            .rules
            .iter()
            .find(|(before, after)| *before == other.number && *after == self.number);

        if other_is_less.is_some() {
            return Some(cmp::Ordering::Greater);
        }

        unreachable!("Every page should have an ordering tule");
    }
}

fn get_line_middle<T: Clone>(line: &[T]) -> T {
    let middle = line.len() / 2;
    line[middle].clone()
}

fn sort(pages: &mut [Page]) {
    pages.sort_by(|a, b| a.partial_cmp(b).expect("Input always be sortable"))
}

fn get_valid_page_lines(input: &str) -> impl Iterator<Item = Vec<Page>> {
    let valid_pages = parse(input);
    valid_pages
        .into_iter()
        .filter(|pages_line| is_valid_page_line(pages_line))
}

fn get_invalid_page_lines(input: &str) -> impl Iterator<Item = Vec<Page>> {
    let valid_pages = parse(input);
    valid_pages
        .into_iter()
        .filter(|pages_line| !is_valid_page_line(pages_line))
}

fn parse(input: &str) -> Vec<Vec<Page>> {
    let (ordering_rules, pages) = input.split_once("\n\n").expect("Input has correct format");
    let ordering_rules: Vec<_> = ordering_rules.lines().map(parse_rule).collect();

    let ordering_rules = Rc::new(ordering_rules);
    let valid_page_lines = pages
        .lines()
        .map(|line| parse_pages_line(line, ordering_rules.clone()))
        .collect();

    valid_page_lines
}

fn parse_rule(rule: &str) -> (u32, u32) {
    let (before, after) = rule.split_once("|").unwrap();
    let before = before.parse().expect("Must be numeric");
    let after = after.parse().expect("Must be numeric");

    (before, after)
}

fn parse_pages_line(line: &str, ordering_rules: Rc<Vec<(u32, u32)>>) -> Vec<Page> {
    line.split(",")
        .map(|page| {
            let number = page.parse().expect("Must be numeric");
            let rules = Rc::clone(&ordering_rules);
            Page { number, rules }
        })
        .collect()
}

fn is_valid_page_line(pages_line: &[Page]) -> bool {
    pages_line
        .iter()
        .enumerate()
        .map(|(page_index, page)| {
            let mut relevant_page_rules =
                page.rules.iter().filter(|(_, after)| *after == page.number);

            relevant_page_rules.all(|(before, _)| {
                let mut trailing_pages = pages_line.iter().skip(page_index + 1);
                !trailing_pages.any(|page| page.number == *before)
            })
        })
        .all(|valid_page| valid_page)
}

fn main() {
    let data = include_str!("../../data/day5");

    println!("Part 1: {}", sum_middle_page_numbers(data));
    println!("Part 2: {}", sum_middle_of_sorted_page_numbers(data));
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
        let page_lines = parse(INPUT);
        let invalid_line = page_lines.into_iter().nth(3).unwrap();

        assert!(!is_valid_page_line(&invalid_line))
    }

    #[test]
    fn gets_valid_page_lines() {
        let valid_pages = get_valid_page_lines(INPUT);

        let expected = [
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
        ]
        .map(|pages_line| {
            pages_line
                .into_iter()
                .map(|number| {
                    let rules = Rc::new(vec![]);
                    Page { number, rules }
                })
                .collect::<Vec<_>>()
        });

        assert_equal(expected, valid_pages);
    }

    #[test]
    fn sums_middle_page_numbers() {
        assert_eq!(143, sum_middle_page_numbers(INPUT));
    }

    #[test]
    fn compares_pages() {
        let rules = parse(INPUT);
        let rules = &rules.first().unwrap().first().unwrap().rules;

        for (before, after) in rules.iter() {
            let page1 = Page {
                number: *before,
                rules: rules.clone(),
            };
            let page2 = Page {
                number: *after,
                rules: rules.clone(),
            };

            assert!(page1 < page2);
        }
    }

    #[test]
    fn sorts_pages() {
        let rules = parse(INPUT);
        let rules = &rules.first().unwrap().first().unwrap().rules;

        let mut pages = [61, 13, 29].map(|number| Page {
            number,
            rules: rules.clone(),
        });
        pages.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    #[test]
    fn sums_middle_of_sorted_page_numbers() {
        assert_eq!(123, sum_middle_of_sorted_page_numbers(INPUT));
    }
}
