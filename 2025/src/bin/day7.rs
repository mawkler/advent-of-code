mod part1 {
    use std::collections::HashSet;

    pub fn count_splits(str: &str) -> u32 {
        let mut lines = str.lines();
        let first_line = lines.next().expect("has lines");
        let start_index = first_line
            .chars()
            .position(|c| c == 'S')
            .expect("has an 'S'");

        let starting_splits = (HashSet::from([start_index]), 0);
        let (_, num_splits) =
            lines.fold(starting_splits, |(beam_columns, mut split_count), line| {
                let new_beam_columns = beam_columns
                    .iter()
                    .flat_map(|&c| {
                        let char = line.chars().nth(c).expect("column exists");
                        if char == '^' {
                            split_count += 1;
                            vec![c - 1, c + 1]
                        } else {
                            vec![c]
                        }
                    })
                    .collect();

                (new_beam_columns, split_count)
            });

        num_splits
    }
}

fn main() {
    let input = include_str!("../../input/day7");

    println!("Part 1: {}", part1::count_splits(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        assert_eq!(part1::count_splits(input), 21);
    }
}
