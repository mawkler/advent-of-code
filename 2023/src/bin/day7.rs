use crate::Card::{Eight, Five, Four, Nine, Seven, Six, Three, Two, A, J, K, Q, T};
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashSet};

#[derive(Debug, PartialEq, PartialOrd, Ord, Copy, Clone, Eq, Hash)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
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
            .filter(|&card| self.0.iter().filter(|&c| c == card || *c == J).count() >= n)
            .collect()
    }

    fn has_n_of_a_kind(&self, n: usize) -> bool {
        let res = !self.find_n_of_a_kind(n).is_empty();
        res
    }

    fn has_full_house(&self) -> bool {
        // NOTE: may return `false` if jokers already caused hand to be a `four of a kind`
        let card_frequencies = self.0.iter().counts();
        let joker_count = card_frequencies.get(&J).unwrap_or(&0);
        let mut card_frequencies = card_frequencies
            .iter()
            .filter(|(&card, _)| card != &J)
            .sorted_by_key(|(_, &count)| count)
            .rev();

        let most_frequent = card_frequencies.next().unwrap();

        if most_frequent.1 + joker_count == 3 {
            let (_, second_most_frequent_count) = card_frequencies.next().unwrap();
            second_most_frequent_count == &2
        } else {
            false
        }
    }

    fn has_two_pair(&self) -> bool {
        // NOTE: ignores some cases where the hand would be better than two pair
        let card_frequencies = self.0.iter().counts();
        let mut card_frequencies = card_frequencies
            .iter()
            .sorted_by_key(|(_, &count)| count)
            .rev();

        let (&most_frequent_card, &most_frequent_count) = card_frequencies.next().unwrap();

        if most_frequent_card != &J && most_frequent_count >= 2 {
            let second_most_frequent = card_frequencies.next().unwrap();
            *second_most_frequent.0 == &J || second_most_frequent.1 >= &2
        } else {
            false
        }
    }

    fn has_one_pair(&self) -> bool {
        !self.find_n_of_a_kind(2).is_empty()
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
            let (self_card, other_card) = self
                .0
                .iter()
                .zip(other.0.iter())
                .find(|(&self_card, &other_card)| self_card != other_card)
                .unwrap();

            self_card.cmp(other_card)
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
    let data = include_str!("../../data/day7");

    let hands = data.lines().map(parse_line);
    let total_winnings: u32 = get_rankings(hands)
        .map(|(ranking, (_, bid))| ranking as u32 * bid)
        .sum();

    println!("Part 2 = {}", total_winnings);
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

        let hand = Hand([K, K, K, K, Two]);
        assert!(hand.has_n_of_a_kind(4));
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
    }

    #[test]
    fn finds_one_pair() {
        let hand = Hand([Eight, Eight, K, Q, A]);
        assert!(hand.has_one_pair());
    }

    #[test]
    fn finds_high_card() {
        let hand = Hand([T, K, A, Three, Two]);
        assert!(hand.has_high_card());
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
    fn compares_more_hands() {
        assert!(Hand::from("KK677") < Hand::from("KTJJT"));
        assert!(Hand::from("T55J5") < Hand::from("QQQJA"));
        assert!(Hand::from("KTJJT") > Hand::from("QQQJA"));
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
            (Hand([K, K, Six, Seven, Seven]), 28),
            (Hand([T, Five, Five, J, Five]), 684),
            (Hand([Q, Q, Q, J, A]), 483),
            (Hand([K, T, J, J, T]), 220),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn handles_jokers() {
        assert!(Hand::from("JJJJJ").has_n_of_a_kind(5));
        assert!(Hand::from("KJJJJ").has_n_of_a_kind(5));
        assert!(Hand::from("T55J5").has_n_of_a_kind(4));
        assert!(Hand::from("KTJJT").has_n_of_a_kind(4));
        assert!(Hand::from("QQQJA").has_n_of_a_kind(4));
        assert!(Hand::from("JKKKT").has_n_of_a_kind(4));
        assert!(Hand::from("JKK5T").has_n_of_a_kind(3));

        assert!(Hand::from("KKJQQ").has_full_house());
        assert!(Hand::from("KJJQQ").has_n_of_a_kind(4));
        assert!(Hand::from("22299").has_full_house());

        assert!(Hand::from("9JT3K").has_one_pair());
        assert!(Hand::from("32T3K").has_one_pair());

        assert!(Hand::from("KK677").has_two_pair());
        assert!(Hand::from("JJ627").has_n_of_a_kind(3));

        assert!(Hand::from("J2345").has_high_card());
        assert!(Hand::from("J2345").has_high_card());
    }

    #[test]
    fn handles_cards_from_real_data() {
        assert!(Hand::from("2J299").has_full_house());
        assert!(Hand::from("47TJ4").has_n_of_a_kind(3));
        assert!(Hand::from("2J299") > Hand::from("47TJ4"));
        assert!(!Hand::from("47TJ4").has_full_house());
        assert!(Hand::from("J8228").has_full_house());
        assert!(Hand::from("33663").has_full_house());
        assert!(Hand::from("J9285").has_one_pair());
        assert!(Hand::from("5825J").has_n_of_a_kind(3));
        assert!(Hand::from("8JA6Q").has_one_pair());
        assert!(Hand::from("QJ777").has_n_of_a_kind(4));
        assert!(Hand::from("JJJJJ").has_n_of_a_kind(5));

        assert!(Hand::from("KJKKK").has_n_of_a_kind(5));
        assert!(Hand::from("JK9T3").has_n_of_a_kind(2));
        assert!(Hand::from("82J22").has_n_of_a_kind(4));
        assert!(Hand::from("Q266J").has_n_of_a_kind(3));
        assert!(Hand::from("682J6").has_n_of_a_kind(3));
        assert!(Hand::from("9J999").has_n_of_a_kind(5));
        assert!(Hand::from("K269J").has_n_of_a_kind(2));
        assert!(Hand::from("77JJJ").has_n_of_a_kind(5));
    }
}
