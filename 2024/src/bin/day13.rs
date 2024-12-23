use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Add,
};

fn find_fewest_tokens(machines: &str) -> usize {
    parse_machines(machines)
        .flat_map(|machine| machine.fewest_prize_tokens())
        .sum()
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Vector(u32, u32);

#[derive(PartialEq, Eq, Debug, Clone)]
struct Machine {
    button_a: Vector,
    button_b: Vector,
    prize: Vector,
    memo: HashMap<Vector, HashSet<Buttons>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Buttons(HashMap<Button, usize>);

impl Buttons {
    fn add(self, button: Button) -> Self {
        let count = self.0.get(&button).unwrap_or(&0);
        let button = [(button, *count + 1)];
        let buttons = self.0.into_iter().chain(button.iter().cloned());

        Buttons(buttons.collect())
    }

    fn token_cost(&self) -> usize {
        self.0
            .iter()
            .map(|(button, count)| button.clone() as usize * count)
            .sum()
    }
}

impl Hash for Buttons {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (key, value) in self.0.iter().sorted_by_key(|&(key, _)| key) {
            key.hash(state);
            value.hash(state);
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Ord, PartialOrd)]
enum Button {
    A = 3,
    B = 1,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Machine {
    fn new(button_a: Vector, button_b: Vector, prize: Vector) -> Self {
        Self {
            button_a,
            button_b,
            prize,
            memo: HashMap::new(),
        }
    }

    fn fewest_prize_tokens(mut self) -> Option<usize> {
        let tokens = self
            .find_prizes(Vector(0, 0), Buttons(HashMap::new()))
            .into_iter()
            .unique()
            .map(|buttons| buttons.token_cost())
            .min()?;

        Some(tokens)
    }

    fn find_prizes(&mut self, coordinate: Vector, pressed: Buttons) -> HashSet<Buttons> {
        if let Some(button_presses) = self.memo.get(&coordinate) {
            return button_presses.clone();
        }

        let pressed_as = *pressed.0.get(&Button::A).unwrap_or(&0);
        let pressed_bs = *pressed.0.get(&Button::B).unwrap_or(&0);

        let done = pressed_as > 100
            || pressed_bs > 100
            || coordinate.0 > self.prize.0
            || coordinate.1 > self.prize.1;

        if done {
            return HashSet::new();
        }

        if coordinate == self.prize {
            self.memo
                .entry(coordinate)
                .or_default()
                .insert(pressed.clone());

            return HashSet::from([pressed]);
        }

        let button_presses = [
            self.find_prizes(coordinate + self.button_a, pressed.clone().add(Button::A)),
            self.find_prizes(coordinate + self.button_b, pressed.add(Button::B)),
        ];
        let button_presses: HashSet<_> = button_presses.into_iter().flatten().collect();

        self.memo
            .entry(coordinate)
            .or_default()
            .extend(button_presses.clone());

        button_presses
    }
}

fn parse_machines(machines: &str) -> impl Iterator<Item = Machine> + use<'_> {
    machines.split("\n\n").map(parse_machine)
}

fn parse_machine(machine: &str) -> Machine {
    let mut lines = machine.lines();
    let button_a = parse_button(lines.next().unwrap());
    let button_b = parse_button(lines.next().unwrap());

    let (_, prize_coordinate) = lines.next().unwrap().split_once("X=").unwrap();
    let (prize_x, prize_y) = prize_coordinate.split_once(", Y=").unwrap();
    let prize = Vector(prize_x.parse().unwrap(), prize_y.parse().unwrap());

    Machine::new(button_a, button_b, prize)
}

fn parse_button(button: &str) -> Vector {
    let (_, movement) = button.split_once("X+").unwrap();
    let (x, y) = movement.split_once(", Y+").unwrap();

    Vector(x.parse().unwrap(), y.parse().unwrap())
}

fn main() {
    let data = include_str!("../../data/day13");

    println!("Part 1: {}", find_fewest_tokens(data));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use itertools::Itertools;

    #[test]
    fn parses_button() {
        let button = "Button A: X+94, Y+34";
        assert_eq!(Vector(94, 34), parse_button(button));
    }

    #[test]
    fn parses_machines() {
        let machines = indoc! {"
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176
        "};

        let expected = vec![
            Machine::new(Vector(94, 34), Vector(22, 67), Vector(8400, 5400)),
            Machine::new(Vector(26, 66), Vector(67, 21), Vector(12748, 12176)),
        ];
        assert_eq!(expected, parse_machines(machines).collect_vec());
    }

    #[test]
    fn add_button() {
        let buttons = Buttons(HashMap::from([(Button::B, 1)]));
        let buttons = buttons.add(Button::B);

        assert_eq!(Buttons(HashMap::from([(Button::B, 2)])), buttons)
    }

    #[test]
    fn finds_fewest_tokens() {
        let machine = indoc! {"
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400
        "};

        assert_eq!(Some(280), parse_machine(machine).fewest_prize_tokens());
    }
}
