use self::Instruction::{Addx, Noop};

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

/// (cycles, value)
type Output = (u32, i32);

impl Instruction {
    fn from_string(string: &str) -> Self {
        let mut instruction = string.split_whitespace();
        match instruction.next() {
            Some("noop") => Noop,
            Some("addx") => Addx(instruction.next().unwrap().parse().unwrap()),
            _ => panic!("No or unexpected word found"),
        }
    }

    fn process(&self) -> Output {
        match self {
            Addx(value) => (2, *value),
            Noop => (1, 0),
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

    let mut cycles_total = 0;
    let mut value_total = 0;
    let mut signal_strengths: Vec<i32> = vec![];

    while cycles_total <= 220 {
        let instruction = Instruction::from_string(&instructions.next().unwrap().unwrap());
        // TODO: we need to be able to probe in the middle of process()
        let (cycles, value) = instruction.process();

        cycles_total += cycles;
        value_total += value;

        if probe(cycles_total) {
            signal_strengths.push(value_total * cycles_total as i32);
        }
    }
}

// fn part_one() {
//     let file_path = "data.txt";
//     let file = File::open(file_path).expect("File not found");
//     let instructions = BufReader::new(file).lines();

//     let probe_cycles = vec![20, 60, 100, 140, 180, 220];
//     let mut signal_strengths: Vec<i32> = vec![];

//     let sum = instructions.fold((0, 0), |accum, instruction| {
//         let instruction = Instruction::from_string(&instruction.unwrap());
//         let (cycles, value) = instruction.process(accum.1);
//         if probe_cycles.contains(&accum.0) {
//             signal_strengths.push(cycles as i32 * value);
//         }
//         (cycles + accum.0, value)
//     });
//     println!("sum: {:?}", sum);
//     println!("signal_strengths: {:?}", signal_strengths);
// }

fn main() {
    part_one();
}
