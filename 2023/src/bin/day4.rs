use indexmap::IndexMap;
use indoc::indoc;
use std::cell::RefCell;

type Id = u16;
type Wins = u8;
type Count = u32;

#[derive(Debug)]
struct Card {
    wins: Wins,
    count: Count,
}

#[derive(Default)]
struct Pile {
    cards: IndexMap<Id, RefCell<Card>>,
}

impl Pile {
    fn generate_cards(&self) {
        for &id in self.cards.keys() {
            let card = self.cards.get(&id).expect("ID should exist").borrow();
            let id = id as Count;
            let copy_ids = id + 1..=id + card.wins as u32;

            for id in copy_ids {
                self.cards
                    .get(&(id as Id))
                    .expect("ID should exist")
                    .borrow_mut()
                    .count += card.count;
            }
        }
    }

    fn count_card_copies(&self) -> Count {
        self.cards
            .iter()
            .map(|(_, count)| count.borrow().count)
            .sum()
    }
}

impl From<Vec<Vec<Id>>> for Pile {
    fn from(value: Vec<Vec<Id>>) -> Self {
        let cards: IndexMap<Id, RefCell<Card>> = value
            .into_iter()
            .enumerate()
            .map(|(i, card_numbers)| {
                let generated_cards = card_numbers.len();
                let wins = generated_cards as Wins;
                let card = Card { wins, count: 1 };

                ((i + 1) as Id, card.into())
            })
            .collect();

        Pile { cards }
    }
}

impl std::fmt::Debug for Pile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Pile");

        self.cards
            .iter()
            .map(|(id, count_refcell)| (id.to_string(), count_refcell.borrow()))
            .for_each(|(key, value)| {
                debug_struct.field(&key, &value);
            });

        debug_struct.finish()
    }
}

fn to_numbers(string: &str) -> Vec<Id> {
    string
        .split_whitespace()
        .map(|number| number.parse().expect("Should be a number"))
        .collect()
}

fn find_winning_numbers(winning_numbers: Vec<Id>, your_numbers: Vec<Id>) -> Vec<Id> {
    your_numbers
        .into_iter()
        .filter(|number| winning_numbers.contains(number))
        .collect()
}

fn parse_winning_numbers(line: &str) -> Vec<Id> {
    let (card, number_list) = line.split_once(':').expect(": always exists");
    let (left_list, right_list) = number_list.split_once('|').expect("| always exists");
    let card_nr = card.split_whitespace().nth(1).expect("Should exist");

    let left: Vec<_> = to_numbers(left_list);
    let right: Vec<_> = to_numbers(right_list);
    let _card_nr: Id = card_nr.parse().expect("Should be a number");

    find_winning_numbers(left, right)
}

fn card_value(numbers: &Vec<Id>) -> Id {
    let count = numbers.len() as Id;
    if count == 0 {
        0
    } else {
        Id::pow(2, (count - 1).into())
    }
}

fn main() {
    let cards = include_str!("../../data/day4");
    let cards: Vec<_> = cards.lines().map(parse_winning_numbers).collect();

    // Day 1
    let sum: Id = cards.iter().map(card_value).sum();
    println!("Day 1: {}", sum);

    // Day 2
    let pile: Pile = cards.into();
    pile.generate_cards();
    let sum = pile.count_card_copies();
    println!("Day 2: {}", sum);
}

const _CARDS: &str = indoc! {"
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"};

#[test]
fn parses_winning_numbers() {
    let winning_numbers: Vec<_> = _CARDS.lines().map(parse_winning_numbers).collect();
    assert_eq!(
        winning_numbers,
        vec![
            vec![83, 86, 17, 48],
            vec![61, 32],
            vec![21, 1],
            vec![84],
            vec![],
            vec![],
        ]
    );
}

#[test]
fn instantiates_card_pile() {
    let cards: Vec<_> = _CARDS.lines().map(parse_winning_numbers).collect();
    let pile: Pile = cards.into();

    let has_all_card_ids =
        (1..=pile.cards.len()).all(|card_id| pile.cards.get(&(card_id as Id)).is_some());
    assert!(has_all_card_ids)
}

#[test]
fn generates_card_pile() {
    let cards: Vec<_> = _CARDS.lines().map(parse_winning_numbers).collect();
    let pile: Pile = cards.into();
    pile.generate_cards();

    let counts: Vec<_> = pile
        .cards
        .iter()
        .map(|(_, count)| count.borrow().count)
        .collect();

    assert_eq!(counts, vec![1, 2, 4, 8, 14, 1]);
}
