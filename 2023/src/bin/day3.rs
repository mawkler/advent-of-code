type Schematic = Vec<Vec<char>>;

#[derive(Debug)]
struct Engine {
    schematic: Schematic,
}

impl From<&str> for Engine {
    fn from(string: &str) -> Self {
        let schematic: Schematic = string.lines().map(|line| line.chars().collect()).collect();

        Engine { schematic }
    }
}

fn get_adjacent_positions() -> Vec<(i32, i32)> {
    (-1..=1)
        .flat_map(|y| (-1..=1).map(move |x| (x, y)))
        .filter(|&position| position != (0, 0))
        .collect()
}

impl Engine {
    fn has_adjacent_symbol(&self, x: usize, y: usize) -> bool {
        self.get_adjacent_characters(x, y)
            .iter()
            .any(|char| char.is_some_and(|&char| char != '.' && !char.is_ascii_digit()))
    }

    fn get(&self, x: i32, y: i32) -> Option<&char> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;

        let char = self.schematic.get(y)?.get(x)?;
        Some(char)
    }

    fn get_adjacent_characters(&self, x: usize, y: usize) -> Vec<Option<&char>> {
        get_adjacent_positions()
            .into_iter()
            .map(|(x_offset, y_offset)| {
                let x = x as i32 + x_offset;
                let y = y as i32 + y_offset;
                self.get(x, y)
            })
            .collect()
    }

    fn parse_line(&self, line_index: i32) -> Vec<u32> {
        let mut numbers: Vec<u32> = vec![];
        let mut found_adjacent = false;
        let mut number = String::new();
        let size = self
            .schematic
            .first()
            .expect("Should have at least one line")
            .len();

        for x in 0..size as i32 {
            let char = self.get(x, line_index).unwrap().to_owned();

            if char.is_ascii_digit() {
                number.push(char);
                if self.has_adjacent_symbol(x as usize, line_index as usize) {
                    found_adjacent = true;
                }
            } else {
                if found_adjacent {
                    numbers.push(number.parse().expect("Should be parseable"));
                }
                number.clear();
                found_adjacent = false;
            }
        }

        if found_adjacent && !number.is_empty() {
            numbers.push(number.parse().expect("Should be parseable"));
        }

        numbers
    }

    fn parse(&self) -> Vec<u32> {
        self.schematic
            .iter()
            .enumerate()
            .flat_map(|(y, _)| self.parse_line(y as i32))
            .collect()
    }
}

fn main() {
    let schematic = include_str!("../../data/day3");
    let engine = Engine::from(schematic);
    let result = engine.parse();
    let sum: u32 = result.iter().sum();

    println!("Part 1: {:#?}", sum);

    let result = engine.parse();
    let sum: u32 = result.iter().sum();

    println!("Part 1: {:#?}", sum);
}

#[test]
fn finds_adjacent_symbols() {
    let schematic = r#"467..114..
...*......"#;
    let engine = Engine::from(schematic);
    let result: Vec<_> = (0..9).map(|i| engine.has_adjacent_symbol(i, 0)).collect();
    println!("result = {:#?}", result);

    assert_eq!(
        result,
        [false, false, true, true, true, false, false, false, false,]
    );
}

#[test]
fn parses_line() {
    let schematic = r#"46.0.114..
...*....*."#;
    let engine = Engine::from(schematic);
    let result = engine.parse_line(0);

    assert_eq!(result, vec![0, 114,]);
}

#[test]
fn parse_schematic() {
    let schematic = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    let engine = Engine::from(schematic);
    let result = engine.parse();

    assert_eq!(result, vec![467, 35, 633, 617, 592, 755, 664, 598,]);
}

#[test]
fn parses_line_with_number_at_the_end() {
    let schematic = r#".......987
...*.....*"#;
    let engine = Engine::from(schematic);
    let result = engine.parse_line(0);
    println!("result = {:#?}", result);

    assert_eq!(result, vec![987]);
}
