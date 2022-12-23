#![allow(unused)]

use self::Direction::{Down, Left, Right, Up};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Matrix(Vec<Vec<bool>>);

impl Matrix {
    fn new() -> Matrix {
        Matrix(vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ])
    }

    fn get(&self, coordinate: &Coordinate) -> &bool {
        &self.0[coordinate.y][coordinate.x]
    }

    fn set(&mut self, coordinate: &Coordinate, value: bool) {
        self.0[coordinate.y][coordinate.x] = value;
    }

    fn new_empty_vector(size: usize) -> Vec<bool> {
        (0..size).map(|_| false).collect()
    }

    /// Doubles the size of the matrix
    fn expand(&mut self) {
        let size = self.0.len();

        for row in &mut self.0 {
            row.append(&mut Self::new_empty_vector(size));
        }

        for _ in 0..size {
            self.0.push(Self::new_empty_vector(size * 2))
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_str(string: &str) -> Self {
        Direction::Up
        // TODO
    }
}

#[derive(Debug)]
struct Roap {
    head: Coordinate,
    tail: Coordinate,
}

#[derive(Debug)]
struct Simulation {
    matrix: Matrix,
    roap: Roap,
}

impl Simulation {
    fn new() -> Self {
        let origo = Coordinate { x: 0, y: 0 };
        Simulation {
            matrix: Matrix::new(),
            roap: Roap {
                head: origo,
                tail: origo,
            },
        }
    }

    fn move_roap_head(&mut self, direction: &Direction) {
        let head = &self.roap.head;
        self.roap.head.y = match direction {
            Up => head.y + 1,
            Down => head.y - 1,
            Left => head.x - 1,
            Right => head.x + 1,
        };

        self.matrix.set(&self.roap.head, true);
    }

    fn move_roap_head_count(&mut self, direction: Direction, count: u32) {
        for _ in 0..count {
            self.move_roap_head(&direction)
        }
    }
}

fn part_one() {
    let file_path = "data.txt";
    let file = File::open(file_path).expect("File not found");
    let _motions = BufReader::new(file).lines();
}

fn main() {
    part_one();

    let mut s = Simulation::new();
    println!("s: {:#?}", s);
    s.move_roap_head_count(Up, 2);
    println!("s: {:#?}", s);

    // let c = Coordinate { x: 0, y: 0 };
    // let m = Matrix::new();
    // let _v = m.get(&c);
}
