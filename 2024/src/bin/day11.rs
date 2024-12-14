use itertools::Itertools;
use std::{collections::HashMap, iter};

type Stone = u64;
type Count = usize;

#[derive(Debug, Clone)]
struct Stones {
    stones: HashMap<Stone, Count>,
    cache: HashMap<Stone, Vec<Stone>>,
}

impl std::fmt::Display for Stones {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stones = self.get_stones().join(" ");
        write!(f, "{stones}")
    }
}

impl Stones {
    fn new(stones: Vec<Stone>) -> Self {
        let cache = HashMap::new();
        let stones = stones.into_iter().counts();
        Self { stones, cache }
    }

    fn blink_times(mut self, times: Stone) -> Self {
        for _ in 0..times {
            self.blink();
        }

        self
    }

    fn blink_stone(&self, stone: Stone) -> Vec<Stone> {
        self.cache
            .get(&stone)
            .cloned()
            .unwrap_or_else(|| match stone {
                0 => vec![1],
                _ if has_even_digits(stone) => split_stone(stone).to_vec(),
                _ => vec![stone * 2024],
            })
    }

    fn blink(&mut self) -> usize {
        self.stones
            .clone()
            .iter()
            .filter(|(_, &count)| count > 0)
            .for_each(|(stone, count)| {
                *self.stones.get_mut(stone).unwrap() -= count;

                let stones = self.blink_stone(*stone);

                stones.iter().for_each(|&stone| {
                    *self.stones.entry(stone).or_default() += count;
                });

                self.cache.insert(*stone, stones);
            });

        self.count_stones()
    }

    fn count_stones(&self) -> usize {
        self.stones.iter().map(|(_, &count)| count).sum()
    }

    fn get_stones(&self) -> impl Iterator<Item = &Stone> {
        self.stones
            .iter()
            .filter(|(_, &count)| count > 0)
            .flat_map(|(stone, &count)| iter::repeat_n(stone, count))
    }
}

fn parse_stones(stones: &str) -> Stones {
    let stones = stones
        .split(" ")
        .map(|stone| stone.parse().unwrap())
        .collect();

    Stones::new(stones)
}

fn has_even_digits(stone: Stone) -> bool {
    stone.to_string().len() % 2 == 0
}

fn split_stone(stone: Stone) -> [Stone; 2] {
    let stone = stone.to_string();
    let (left, right) = stone.split_at(stone.len() / 2);

    [left.parse().unwrap(), right.parse().unwrap()]
}

fn main() {
    let data = include_str!("../../data/day11");

    println!(
        "Part 1: {}",
        parse_stones(data).blink_times(25).count_stones()
    );
    println!(
        "Part 2: {}",
        parse_stones(data).blink_times(75).count_stones()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;

    fn assert_iterators_eq<T: Eq + std::hash::Hash + std::fmt::Debug + std::cmp::Ord>(
        iter1: impl IntoIterator<Item = T>,
        iter2: impl IntoIterator<Item = T>,
    ) {
        let vec1: Vec<_> = iter1.into_iter().sorted().collect();
        let vec2: Vec<_> = iter2.into_iter().sorted().collect();

        assert_equal(vec1, vec2);
    }

    #[test]
    fn splits_stone() {
        assert_eq!([253, 0], split_stone(253000));
    }

    #[test]
    fn blinks_stones() {
        let mut stones = parse_stones("125 17");
        stones.blink();

        assert_iterators_eq(&[253000, 1, 7], stones.get_stones());

        let mut stones = parse_stones("253000 1 7");
        stones.blink();

        assert_iterators_eq(&[253, 0, 2024, 14168], stones.get_stones());

        let mut stones = parse_stones("253 0 2024 14168");
        stones.blink();

        assert_iterators_eq(&[512072, 1, 20, 24, 28676032], stones.get_stones());

        let mut stones = parse_stones("512072 1 20 24 28676032");
        stones.blink();

        let expected = &[512, 72, 2024, 2, 0, 2, 4, 2867, 6032];
        assert_iterators_eq(expected, stones.get_stones());

        let mut stones = parse_stones("512 72 2024 2 0 2 4 2867 6032");
        stones.blink();

        assert_iterators_eq(
            &[1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32],
            stones.get_stones(),
        );

        let mut stones = parse_stones("1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32");
        stones.blink();

        let expected = &[
            2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3,
            2,
        ];
        assert_iterators_eq(expected, stones.get_stones());
    }

    #[test]
    fn blinks_times() {
        assert_eq!(55312, parse_stones("125 17").blink_times(25).count_stones());
    }
}
