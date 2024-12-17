use itertools::Itertools;
use std::iter;

// Part 1
pub fn sum_valid_equations(equations: &str) -> Number {
    let operators = [Operator::Addition, Operator::Multiplication];
    parse_calibration_equations(equations)
        .filter(|(expected, numbers)| equation_is_valid(*expected, numbers, &operators))
        .map(|(result, _)| result)
        .sum()
}

// Part 2
pub fn sum_valid_equations_with_concatenation(equations: &str) -> Number {
    let operators = [
        Operator::Addition,
        Operator::Multiplication,
        Operator::Concatenation,
    ];
    parse_calibration_equations(equations)
        .filter(|(expected, numbers)| equation_is_valid(*expected, numbers, &operators))
        .map(|(expected, _)| expected)
        .sum()
}

type Number = u64;

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    Addition,
    Multiplication,
    Concatenation,
}

// RHS of an equation
struct Expression {
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

impl Expression {
    fn evaluate(&self) -> Number {
        let (head, tail) = self.numbers.split_at(1);
        let head = head.first().expect("Always has a head");

        tail.iter()
            .zip(self.operators.clone())
            .fold(*head, |acc, (&number, operator)| {
                operator.execute(acc, number)
            })
    }
}

impl PartialEq<Number> for Expression {
    fn eq(&self, number: &Number) -> bool {
        self.evaluate() == *number
    }
}

fn parse_calibration_equations(
    equations: &str,
) -> impl Iterator<Item = (Number, Vec<Number>)> + use<'_> {
    equations.lines().map(|line| {
        let (expected, numbers) = line.split_once(": ").expect("Is correctly formatted");

        let expected = expected.parse().expect("Is numeric");
        let numbers = numbers
            .split(" ")
            .map(|n| n.parse().expect("Is numeric"))
            .collect();

        (expected, numbers)
    })
}

fn equation_is_valid(expected: Number, numbers: &[Number], operators: &[Operator]) -> bool {
    create_operator_permutations(operators, numbers.len() - 1).any(|operators| {
        let numbers = numbers.to_vec();
        let operators = operators.to_vec();

        Expression { numbers, operators } == expected
    })
}

fn create_operator_permutations(
    operators: &[Operator],
    length: usize,
) -> impl Iterator<Item = Vec<Operator>> + use<'_> {
    iter::repeat_n(operators.iter().cloned(), length).multi_cartesian_product()
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
    fn evaluates_expression() {
        let operators = vec![Operator::Addition, Operator::Multiplication];
        let numbers = vec![81, 40, 27];
        let expression = Expression { numbers, operators };

        assert_eq!(expression.evaluate(), 3267);
    }

    #[test]
    fn generates_operator_permutations() {
        let operators = [Operator::Addition, Operator::Multiplication];
        let permutations: Vec<_> = create_operator_permutations(&operators, 2).collect();

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
