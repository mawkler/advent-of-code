fn to_numbers(string: &str) -> Vec<u32> {
    string
        .split_whitespace()
        .map(|number| number.parse().expect("Should be a number"))
        .collect()
}

fn find_winning_numbers(winning_numbers: Vec<u32>, your_numbers: Vec<u32>) -> Vec<u32> {
    your_numbers
        .into_iter()
        .filter(|number| winning_numbers.contains(number))
        .collect()
}

fn parse_winning_numbers(line: &str) -> Vec<u32> {
    let (card, number_list) = line.split_once(':').expect(": always exists");
    let (left_list, right_list) = number_list.split_once('|').expect("| always exists");
    let card_nr = card.split_whitespace().nth(1).expect("Should exist");

    let left: Vec<_> = to_numbers(left_list);
    let right: Vec<_> = to_numbers(right_list);
    let card_nr: u32 = card_nr.parse().expect("Should be a number");

    find_winning_numbers(left, right)
}

fn card_value(numbers: Vec<u32>) -> u32 {
    let count = numbers.len() as u32;
    if count == 0 {
        0
    } else {
        u32::pow(2, count - 1)
    }
}

fn main() {
    let cards = include_str!("../../data/day4");
    let sum: u32 = cards
        .lines()
        .map(|line| {
            let card_numbers = parse_winning_numbers(line);
            card_value(card_numbers)
        })
        .sum();

    println!("Part 1: {}", sum);
}

#[test]
fn parses_winning_numbers() {
    use indoc::indoc;

    let data = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    let winning_numbers: Vec<_> = data.lines().map(parse_winning_numbers).collect();
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
