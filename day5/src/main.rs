use std::{fs, collections::HashMap};

#[derive(Debug)]
struct ParseError;

#[derive(Debug, Clone, Copy)]
struct Crate(char);

fn get_stack_nr(index: usize) -> usize {
    if index <= 1 {
        return 1;
    } else {
        return (index - 1) / 3 + 1;
    }
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
            // TODO: if None, create stack
            self.crate_stacks.insert(stack_nr, vec![crt]);
        }
    }

    fn move_crate(&self, crt: Crate, from_stack: usize, to_stack: usize) {
        unimplemented!();
    }
}

fn part_one() {
    let file_path = "data.txt";
    let data = fs::read_to_string(file_path).expect("File not found");
    let (crates, _moves) = data.split_once("\n\n").unwrap();

    let mut crate_stacks = CrateStacks::new();

    for line in crates.lines().rev().skip(1) {
        for (i, window) in line.chars().collect::<Vec<char>>().windows(3).enumerate() {
            if let ('[', ']') = (window[0], window[2]) {
                let crt = Crate(window[1]);
                crate_stacks.place_crate(crt, get_stack_nr(i));
            }
        }
    }

    println!("crate_stacks: {:?}", crate_stacks);
}

fn main() {
    part_one();
    // part_two();
}
