use itertools::Itertools;
use std::iter;

// Part 1
pub fn sum_valid_equations(equations: &str) -> Number {
    parse_calibration_equations(equations)
        .filter(|(expected, numbers)| equation_is_valid(numbers, *expected))
        .map(|(result, _)| result)
        .sum()
}

// Part 2
pub fn sum_valid_equations_with_concatenation(equations: &str) -> Number {
    parse_calibration_equations(equations)
        .filter(|(expected, numbers)| equation_is_valid_with_concatenation(numbers, *expected))
        .map(|(result, _)| result)
        .sum()
}

type Number = i64;

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    Addition,
    Multiplication,
    Concatenation,
}

struct Equation {
    numbers: Vec<Number>,
    operators: Vec<Operator>,
}

impl Operator {
    fn execute(&self, left: Number, right: Number) -> Number {
        match self {
            Operator::Addition => left + right,
            Operator::Multiplication => left * right,
            Operator::Concatenation => format!("{left}{right}").parse().expect("Is numeric"),
        }
    }
}

impl Equation {
    fn evaluate(self) -> Number {
        let (head, tail) = self.numbers.split_at(1);
        let head = head.first().expect("Always has a head");

        tail.iter()
            .zip(self.operators)
            .fold(*head, |acc, (n, operator)| operator.execute(acc, *n))
    }
}

fn parse_calibration_equations(
    equations: &str,
) -> impl Iterator<Item = (Number, Vec<Number>)> + use<'_> {
    equations.lines().map(|line| {
        let (expected, numbers) = line.split_once(": ").expect("Is correctly formatted");
        let numbers = parse_equation_numbers(numbers);

        (expected.parse().expect("Is numeric"), numbers)
    })
}

fn parse_equation_numbers(equation: &str) -> Vec<Number> {
    equation
        .split(" ")
        .map(|n| n.parse().expect("Equation must contain only numbers"))
        .collect()
}

fn create_operator_permutations(length: usize) -> impl Iterator<Item = Vec<Operator>> {
    const OPERATORS: [Operator; 2] = [Operator::Addition, Operator::Multiplication];
    iter::repeat(OPERATORS.iter().cloned())
        .take(length)
        .multi_cartesian_product()
}

fn create_operator_permutations_with_concatenation(
    length: usize,
) -> impl Iterator<Item = Vec<Operator>> {
    const OPERATORS: [Operator; 3] = [
        Operator::Addition,
        Operator::Multiplication,
        Operator::Concatenation,
    ];

    iter::repeat(OPERATORS.iter().cloned())
        .take(length)
        .multi_cartesian_product()
}

fn equation_is_valid(numbers: &[Number], expected: Number) -> bool {
    create_operator_permutations(numbers.len() - 1)
        .filter(move |operators| equation_has_expected_output(numbers, operators, expected))
        .count()
        > 0
}

fn equation_is_valid_with_concatenation(numbers: &[Number], expected: Number) -> bool {
    create_operator_permutations_with_concatenation(numbers.len() - 1)
        .filter(move |operators| equation_has_expected_output(numbers, operators, expected))
        .count()
        > 0
}

fn equation_has_expected_output(
    numbers: &[Number],
    operators: &[Operator],
    expected: Number,
) -> bool {
    let numbers = numbers.to_vec();
    let operators = operators.to_vec();
    let result = Equation { numbers, operators }.evaluate();

    result == expected
}

fn main() {
    let data = include_str!("../../data/day7");

    println!("Part 1: {}", sum_valid_equations(data));
    println!("Part 2: {}", sum_valid_equations_with_concatenation(data));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use itertools::assert_equal;

    #[test]
    fn parses_equation() {
        let equation = "81 40 27";
        let equation = parse_equation_numbers(equation);

        assert_equal(equation, vec![81, 40, 27]);
    }

    #[test]
    fn evaluates_equation() {
        let equation = "81 40 27";
        let operators = vec![Operator::Addition, Operator::Multiplication];
        let numbers = parse_equation_numbers(equation);
        let equation = Equation { numbers, operators };

        assert_eq!(equation.evaluate(), 3267);
    }

    #[test]
    fn generates_operator_permutations() {
        let permutations: Vec<_> = create_operator_permutations(2).collect();

        let expected = vec![
            vec![Operator::Addition, Operator::Addition],
            vec![Operator::Addition, Operator::Multiplication],
            vec![Operator::Multiplication, Operator::Addition],
            vec![Operator::Multiplication, Operator::Multiplication],
        ];

        assert_equal(expected, permutations);
    }

    #[test]
    fn sums_valid_equations() {
        let equations = indoc! {"
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
        "};

        assert_eq!(3749, sum_valid_equations(equations));
    }

    #[test]
    fn sums_valid_equations_with_concatenation() {
        let equations = indoc! {"
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
        "};

        assert_eq!(11387, sum_valid_equations_with_concatenation(equations));
    }
}
