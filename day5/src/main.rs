use std::{fs, fmt::{Debug, Display}};

#[derive(Clone, Copy)]
struct Crate(char);

impl Debug for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_ref())
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_ref())
    }
}

fn get_stack_nr(index: usize) -> usize {
    return index / 4 + 1;
}

#[derive(Debug)]
struct CrateStacks {
    crate_stacks: Vec<Vec<Crate>>
}

impl CrateStacks {
    fn new() -> Self {
        CrateStacks { crate_stacks: vec!(vec!()) }
    }

    fn place_crate(&mut self, crt: Crate, stack_nr: usize) {
        if let Some(stack) = self.crate_stacks.get_mut(stack_nr - 1) {
            stack.push(crt);
        } else {
            self.crate_stacks.push(vec![crt]);
        }
    }

    // Assumes that the vector at `stack_nr` already exists
    fn place_crates(&mut self, crates: &mut Vec<Crate>, stack_nr: usize) {
        if let Some(stack) = self.crate_stacks.get_mut(stack_nr - 1) {
            stack.append(crates);
        } else {
            self.crate_stacks.push(crates.to_owned());
        }
    }

    fn move_crate(&mut self, from_stack: usize, to_stack: usize) {
        let stack = self.crate_stacks.get_mut(from_stack - 1).unwrap();
        let crt = stack.pop().unwrap();
        self.place_crate(crt, to_stack);
    }

    fn move_crates(&mut self, from_stack: usize, to_stack: usize, count: usize) {
        let stack = self.crate_stacks.get_mut(from_stack - 1).unwrap();
        let range = stack.len() - count..stack.len();
        let mut crates: Vec<Crate> = stack.drain(range).collect();
        self.place_crates(&mut crates, to_stack);
    }

    fn move_crate_from_string(&mut self, string: &str) {
        let action: Vec<&str> = string.split(' ').collect();
        let [times, from, to] = [action[1], action[3], action[5]].map(|a| a.parse::<usize>().unwrap());
        for _ in 0..times {
            self.move_crate(from, to);
        }
    }

    fn move_crates_from_string(&mut self, string: &str) {
        let action: Vec<&str> = string.split(' ').collect();
        let [count, from, to] = [action[1], action[3], action[5]].map(|a| a.parse::<usize>().unwrap());
        self.move_crates(from, to, count);
    }

    fn get_top_crates(&self) -> Vec<&Crate> {
        self.crate_stacks.iter().map(|stack| {
            stack.last().unwrap()
        }).collect()
    }

    fn top_crates_to_string(&self) -> String {
        self
            .get_top_crates()
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

fn initialize() -> (CrateStacks, String) {
    let file_path = "data.txt";
    let data = fs::read_to_string(file_path).expect("File not found");
    let (crates, moves) = data.split_once("\n\n").unwrap();

    let mut crate_stacks = CrateStacks::new();

    // Initialize crate stack configurations
    for line in crates.lines().rev().skip(1) {
        for (i, window) in line.chars().collect::<Vec<char>>().windows(3).enumerate() {
            if let ('[', crt, ']') = (window[0], window[1], window[2]) {
                crate_stacks.place_crate(Crate(crt), get_stack_nr(i));
            }
        }
    }

    return (crate_stacks, moves.to_string());
}

fn part_one() {
    let (mut crate_stacks, moves) = initialize();

    // Perform moves
    for action in moves.lines() {
        crate_stacks.move_crate_from_string(action);
    }

    // Get top crates
    let top_crates = crate_stacks.top_crates_to_string();

    println!("Part 1: {:?}", top_crates);
}

fn part_two() {
    let (mut crate_stacks, moves) = initialize();

    // Perform moves
    for action in moves.lines() {
        crate_stacks.move_crates_from_string(action);
    }

    // Get top crates
    let top_crates = crate_stacks.top_crates_to_string();

    println!("Part 2: {:?}", top_crates);

}

fn main() {
    part_one();
    part_two();
}
