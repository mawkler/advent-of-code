use itertools::Itertools;

pub fn count_safe_reports(reports: &str) -> u32 {
    parse_reports(reports)
        .into_iter()
        .filter(|report| is_safe(report))
        .count() as u32
}

fn parse_reports(reports: &str) -> Vec<Vec<u32>> {
    reports
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse().expect("All levels are numeric"))
                .collect()
        })
        .collect()
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

fn is_sorted(report: &[u32]) -> bool {
    report.iter().is_sorted() || report.iter().rev().is_sorted()
}

pub fn main() {
    let data = include_str!("../../data/day2");

    println!("Part 1: {}", count_safe_reports(data));
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn parses_reports() {
        let reports = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "};

        assert_eq!(2, count_safe_reports(reports));
    }
}
