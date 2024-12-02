use itertools::Itertools;

pub fn count_safe_reports(reports: &str) -> u32 {
    parse_reports(reports)
        .filter(|report| is_safe(report))
        .count() as u32
}

pub fn count_safe_reports_dampened(reports: &str) -> u32 {
    parse_reports(reports)
        .filter(|report| {
            if is_safe(report) {
                return true;
            }

            is_safe_dampened(report)
        })
        .count() as u32
}

fn parse_reports(reports: &str) -> impl Iterator<Item = Vec<u32>> + use<'_> {
    reports.lines().map(|line| {
        line.split_whitespace()
            .map(|level| level.parse().expect("All levels are numeric"))
            .collect()
    })
}

fn is_safe(report: &[u32]) -> bool {
    if !is_sorted(report) {
        return false;
    }

    report.iter().tuple_windows().all(|(&level, &next_level)| {
        let delta = level.abs_diff(next_level);
        (1..=3).contains(&delta)
    })
}

// Dampening: one unsafe report is acceptable
fn is_safe_dampened(report: &[u32]) -> bool {
    (0..report.len()).any(|n| {
        let dampened_report = report
            .iter()
            .enumerate()
            .filter(move |(i, _)| *i != n)
            .map(|(i, report)| *report)
            .collect_vec();

        is_safe(dampened_report.as_slice())
    })
}

fn is_sorted(report: &[u32]) -> bool {
    report.iter().is_sorted() || report.iter().rev().is_sorted()
}

pub fn main() {
    let data = include_str!("../../data/day2");

    println!("Part 1: {}", count_safe_reports(data));
    println!("Part 2: {}", count_safe_reports_dampened(data));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const REPORTS: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    fn parses_reports() {
        assert_eq!(2, count_safe_reports(REPORTS));
    }

    #[test]
    fn parses_reports_dampened() {
        assert_eq!(4, count_safe_reports_dampened(REPORTS));
    }

    #[test]
    fn test_is_safe_dampened() {
        assert!(is_safe_dampened(vec![1, 3, 2, 4, 5].as_slice()));
        assert!(is_safe_dampened(vec![8, 6, 4, 4, 1].as_slice()));
    }
}
