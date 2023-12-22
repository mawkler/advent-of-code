use crate::Card::{Eight, Five, Four, Nine, Seven, Six, Three, Two, A, J, K, Q, T};
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashSet};

#[derive(Debug, PartialEq, PartialOrd, Ord, Copy, Clone, Eq, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => T,
            'J' => J,
            'Q' => Q,
            'K' => K,
            'A' => A,
            other => panic!("Unrecognized card {} found", other),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Hand([Card; 5]);

impl Hand {
    /// Finds all card types that there are exactly n of
    fn find_n_of_a_kind(&self, n: usize) -> HashSet<&Card> {
        self.0
            .iter()
            .filter(|card| self.0.iter().filter(|c| c == card).count() == n)
            .collect()
    }

    fn has_n_of_a_kind(&self, n: usize) -> bool {
        !self.find_n_of_a_kind(n).is_empty()
    }

    fn has_full_house(&self) -> bool {
        self.has_n_of_a_kind(3) && self.has_n_of_a_kind(2)
    }

    fn has_two_pair(&self) -> bool {
        self.find_n_of_a_kind(2).len() >= 2
    }

    fn has_one_pair(&self) -> bool {
        self.find_n_of_a_kind(2).len() == 1
    }

    fn has_high_card(&self) -> bool {
        self.find_n_of_a_kind(1).len() == 5
    }
}

impl From<&str> for Hand {
    fn from(string: &str) -> Self {
        let char_array: [char; 5] = string.chars().collect::<Vec<_>>().try_into().unwrap();

        Hand(char_array.map(Card::from))
    }
}

impl From<&Hand> for u32 {
    fn from(hand: &Hand) -> u32 {
        if hand.has_n_of_a_kind(5) {
            7
        } else if hand.has_n_of_a_kind(4) {
            6
        } else if hand.has_full_house() {
            5
        } else if hand.has_n_of_a_kind(3) {
            4
        } else if hand.has_two_pair() {
            3
        } else if hand.has_one_pair() {
            2
        } else if hand.has_high_card() {
            1
        } else {
            0
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ordering = u32::from(self).cmp(&other.into());
        if let Ordering::Equal = ordering {
            let (s, o) = self
                .0
                .iter()
                .zip(other.0.iter())
                .find(|(&s, &o)| s != o)
                .unwrap();

            s.cmp(o)
        } else {
            ordering
        }
    }
}

fn get_rankings(
    hands: impl Iterator<Item = (Hand, u32)>,
) -> impl Iterator<Item = (usize, (Hand, u32))> {
    hands
        .sorted_by_key(|pair| pair.0.clone())
        .enumerate()
        .map(|(rank, pair)| (rank + 1, pair))
}

fn parse_line(line: &str) -> (Hand, u32) {
    let (hand, bid) = line.split_once(' ').unwrap();
    let hand: Hand = hand.into();
    let bid: u32 = bid.parse().unwrap();

    (hand, bid)
}

fn main() {
    let hands = include_str!("../../data/day7").lines().map(parse_line);
    let total_winnings: u32 = get_rankings(hands)
        .map(|(ranking, (_, bid))| ranking as u32 * bid)
        .sum();

    println!("Part 1 = {}", total_winnings);
}

#[cfg(test)]
mod tests {
    use crate::{
        parse_line,
        Card::{Eight, Five, Seven, Six, Three, Two, A, J, K, Q, T},
        Hand,
    };
    use indoc::indoc;
    use itertools::Itertools;

    const DATA: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn finds_n_of_a_kind() {
        let hand = Hand([K; 5]);
        assert!(hand.has_n_of_a_kind(5));
        assert!(!hand.has_n_of_a_kind(4));
        assert!(!hand.has_n_of_a_kind(3));

        let hand = Hand([K, K, K, K, Two]);
        assert!(!hand.has_n_of_a_kind(5));
        assert!(hand.has_n_of_a_kind(4));
        assert!(!hand.has_n_of_a_kind(3));
    }

    #[test]
    fn finds_full_house() {
        let hand = Hand([Eight, Eight, Q, Q, Q]);
        assert!(hand.has_full_house());

        let hand = Hand([Eight, Two, Q, Q, Q]);
        assert!(!hand.has_full_house());
    }

    #[test]
    fn finds_two_pair() {
        let hand = Hand([Eight, Eight, Q, Q, Five]);
        assert!(hand.has_two_pair());

        let hand = Hand([Eight, Eight, Q, Q, Q]);
        assert!(!hand.has_two_pair());

        let hand = Hand([Eight, Two, Q, Q, Q]);
        assert!(!hand.has_two_pair());
    }

    #[test]
    fn finds_one_pair() {
        let hand = Hand([Eight, Eight, K, Q, A]);
        assert!(hand.has_one_pair());

        let hand = Hand([Eight, Eight, Q, Q, K]);
        assert!(!hand.has_one_pair());

        let hand = Hand([Eight, Two, Q, Q, Q]);
        assert!(!hand.has_one_pair());

        let hand = Hand([Q, Q, Q, Q, Q]);
        assert!(!hand.has_one_pair());
    }

    #[test]
    fn finds_high_card() {
        let hand = Hand([T, K, A, Three, J]);
        assert!(hand.has_high_card());

        let hand = Hand([Q, Q, Q, Q, Q]);
        assert!(!hand.has_one_pair());
    }

    #[test]
    fn parses_lines() {
        let result: Vec<_> = DATA.lines().map(parse_line).collect();
        let expected = vec![
            (Hand([Three, Two, T, Three, K]), 765),
            (Hand([T, Five, Five, J, Five]), 684),
            (Hand([K, K, Six, Seven, Seven]), 28),
            (Hand([K, T, J, J, T]), 220),
            (Hand([Q, Q, Q, J, A]), 483),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn compares_hands() {
        let hand1 = Hand([Three, Two, T, Three, K]);
        let hand2 = Hand([T, Five, Five, J, Five]);
        let hand3 = Hand([K, K, Six, Seven, Seven]);
        let hand4 = Hand([K, T, J, J, T]);
        let hand5 = Hand([Q, Q, Q, J, A]);

        assert!(hand1 < hand2)
    }

    #[test]
    fn compares_two_pairs() {
        let hand1: Hand = "KK677".into();
        let hand2: Hand = "KTJJT".into();

        assert!(hand1 > hand2);

        let hand1: Hand = "T55J5".into();
        let hand2: Hand = "QQQJA".into();

        assert!(hand1 < hand2);

        let hand1: Hand = "KTJJT".into();
        let hand2: Hand = "QQQJA".into();

        assert!(hand1 < hand2);
    }

    #[test]
    fn sorts_lines() {
        let result = DATA
            .lines()
            .map(parse_line)
            .sorted_by_key(|pair| pair.0.clone())
            .collect_vec();

        let expected = vec![
            (Hand([Three, Two, T, Three, K]), 765),
            (Hand([K, T, J, J, T]), 220),
            (Hand([K, K, Six, Seven, Seven]), 28),
            (Hand([T, Five, Five, J, Five]), 684),
            (Hand([Q, Q, Q, J, A]), 483),
        ];

        assert_eq!(result, expected);
    }
}
