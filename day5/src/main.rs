use std::{fs, collections::HashMap, fmt::{Debug, Display}};

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
    crate_stacks: HashMap<usize, Vec<Crate>>
}

impl CrateStacks {
    fn new() -> Self {
        CrateStacks { crate_stacks: HashMap::new() }
    }

    fn place_crate(&mut self, crt: Crate, stack_nr: usize) {
        if let Some(stack) = self.crate_stacks.get_mut(&stack_nr) {
            stack.push(crt);
        } else {
            self.crate_stacks.insert(stack_nr, vec![crt]);
        }
    }

    fn move_crate(&mut self, from_stack: usize, to_stack: usize) {
        let stack = self.crate_stacks.get_mut(&from_stack).unwrap();
        let crt = stack.pop().unwrap();
        self.place_crate(crt, to_stack);
    }

    fn move_from_string(&mut self, string: &str) {
        let action: Vec<&str> = string.split(' ').collect();
        let [times, from, to] = [action[1], action[3], action[5]].map(|a| a.parse::<usize>().unwrap());
        for _ in 0..times {
            self.move_crate(from, to);
        }
    }

    fn get_top_crates(&self) -> Vec<&Crate> {
        self.crate_stacks.iter().map(|(_, stack)| {
            stack.last().unwrap()
        }).collect()
    }
}

fn part_one() {
    let file_path = "data.txt";
    let data = fs::read_to_string(file_path).expect("File not found");
    let (crates, moves) = data.split_once("\n\n").unwrap();

    let mut crate_stacks = CrateStacks::new();

    // Initial crate stack configurations
    for line in crates.lines().rev().skip(1) {
        for (i, window) in line.chars().collect::<Vec<char>>().windows(3).enumerate() {
            if let ('[', crt, ']') = (window[0], window[1], window[2]) {
                crate_stacks.place_crate(Crate(crt), get_stack_nr(i));
            }
        }
    }

    // Moves
    for action in moves.lines() {
        crate_stacks.move_from_string(action);
    }

    let result: String = crate_stacks
        .get_top_crates()
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("");

    println!("Part 1: {:?}", result);
}

fn main() {
    part_one();
    // part_two();
}
