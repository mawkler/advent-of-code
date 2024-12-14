type Stone = u64;

struct Stones(Vec<Stone>);

impl Stones {
    fn blink_times(mut self, times: Stone) -> Stones {
        for _ in 0..times {
            self = self.blink();
        }
        self
    }

    fn blink(self) -> Stones {
        let stones = self
            .0
            .into_iter()
            .flat_map(|stone| match stone {
                0 => vec![1],
                _ if has_even_digits(stone) => split_stone(stone).to_vec(),
                _ => vec![stone * 2024],
            })
            .collect();

        Stones(stones)
    }

    fn count(&self) -> usize {
        self.0.len()
    }
}

fn parse_stones(stones: &str) -> Stones {
    let stones = stones
        .split(" ")
        .map(|stone| stone.parse().unwrap())
        .collect();

    Stones(stones)
}

fn has_even_digits(stone: Stone) -> bool {
    stone.to_string().len() % 2 == 0
}

fn split_stone(stone: Stone) -> [Stone; 2] {
    let stone = stone.to_string();
    let (left, right) = stone.split_at(stone.len() / 2);

    [left.parse().unwrap(), right.parse().unwrap()]
}

fn blink_and_count(stones: &str) -> usize {
    parse_stones(stones).blink_times(25).count()
}

fn main() {
    let data = include_str!("../../data/day11");

    println!("Part 1: {}", blink_and_count(data));
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;

    #[test]
    fn splits_stone() {
        assert_eq!([253, 0], split_stone(253000));
    }

    #[test]
    fn blinks() {
        assert_equal([253000, 1, 7], parse_stones("125 17").blink().0);
        assert_equal([253, 0, 2024, 14168], parse_stones("253000 1 7").blink().0);

        let expected = [512072, 1, 20, 24, 28676032];
        assert_equal(expected, parse_stones("253 0 2024 14168").blink().0);
    }

    #[test]
    fn blinks_times() {
        assert_eq!(55312, parse_stones("125 17").blink_times(25).count());
    }
}
