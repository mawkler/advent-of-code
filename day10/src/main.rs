use self::Instruction::{Addx, Noop};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Instruction {
    Addx(i32, u32),
    Noop,
}

enum Output {
    Instruction(Instruction),
    Value(i32),
}

impl Instruction {
    fn from_string(string: &str) -> Self {
        let mut instruction = string.split_whitespace();
        match instruction.next() {
            Some("noop") => Noop,
            Some("addx") => Addx(instruction.next().unwrap().parse().unwrap(), 2),
            _ => panic!("No or unexpected word found"),
        }
    }

    fn process(&self, value_total: i32) -> Output {
        match self {
            Addx(value, 1) => Output::Value(value_total + value),
            Addx(value, cycles) => Output::Instruction(Addx(*value, cycles - 1)),
            Noop => Output::Value(value_total),
        }
    }
}

fn probe(cycles: u32) -> bool {
    cycles >= 20 && (cycles - 20) % 40 == 0
}

fn part_one() {
    let file_path = "data.txt";
    let file = File::open(file_path).expect("File not found");
    let mut instructions = BufReader::new(file).lines();

    let mut value_total = 1;
    let mut signal_strengths: Vec<i32> = vec![];
    let mut current_instruction: Option<Instruction> = None;

    for cycle in 1..221 {
        let instruction = if let Some(instruction) = current_instruction.take() {
            instruction
        } else {
            Instruction::from_string(&instructions.next().unwrap().unwrap())
        };

        if probe(cycle) {
            signal_strengths.push(value_total * cycle as i32);
        }

        let output = instruction.process(value_total);
        match output {
            Output::Value(value) => value_total = value,
            Output::Instruction(instruction) => current_instruction = Some(instruction),
        }
    }

    let sum: i32 = signal_strengths.iter().sum();
    assert_eq!(sum, 12520);
    println!("Part 1: {:?}", sum);
}

fn main() {
    part_one();
}
