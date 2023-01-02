#![allow(unused)] // TODO: remove this

use self::Dimension::{Horizontal, Vertical};
use self::Direction::{Down, Left, Right, Up};
use core::cmp::Ordering;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl PartialEq<Coordinate> for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq<(usize, usize)> for Coordinate {
    fn eq(&self, (x, y): &(usize, usize)) -> bool {
        self.x == *x && self.y == *y
    }
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

    fn size(&self) -> usize {
        self.0.len()
    }

    /// Prepends `prepend_vec` to `vec`
    fn prepend<T>(vec: &mut Vec<T>, prepend_vec: Vec<T>) {
        vec.splice(0..0, prepend_vec);
    }

    fn needs_expanding(&self, coordinate: &Coordinate, move_direction: &Direction) -> bool {
        let max = self.size() - 1;
        match move_direction {
            Up => coordinate.y >= max,
            Down => coordinate.y <= max,
            Left => coordinate.x <= max,
            Right => coordinate.x >= max,
        }
    }

    /// Doubles the size of the matrix
    fn expand(&mut self) {
        let margin = self.size() / 2; // In case size is odd
        let new_size = self.size() + margin * 2;

        for row in &mut self.0 {
            Self::prepend(row, Self::new_empty_vector(margin));
            row.append(&mut Self::new_empty_vector(margin));
        }

        for _ in 0..margin {
            self.0.insert(0, Self::new_empty_vector(new_size));
            self.0.push(Self::new_empty_vector(new_size));
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
        todo!()
    }
}

#[derive(Debug)]
enum Dimension {
    Horizontal,
    Vertical,
}

impl Dimension {
    fn from_direction(direction: &Direction) -> Dimension {
        match direction {
            Up | Down => Self::Vertical,
            Left | Right => Self::Horizontal,
        }
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
        match direction {
            Up => self.roap.head.y = head.y + 1,
            Down => self.roap.head.y = head.y - 1,
            Left => self.roap.head.x = head.x - 1,
            Right => self.roap.head.x = head.x + 1,
        };
    }

    fn move_roap_tail(&mut self, direction: &Direction) {
        let tail = &self.roap.tail;
        match direction {
            Up => self.roap.tail.y = tail.y + 1,
            Down => self.roap.tail.y = tail.y - 1,
            Left => self.roap.tail.x = tail.x - 1,
            Right => self.roap.tail.x = tail.x + 1,
        };
    }

    // TODO: use count in move_roap_count instead
    // fn move_roap_head_count(&mut self, direction: Direction, count: u32) {
    //     for _ in 0..count {
    //         self.move_roap_head(&direction)
    //     }
    // }

    fn diagonal_tail_adjustment(
        horizontal_diff: i32,
        vertical_diff: i32,
        head_move_direction: &Direction,
    ) -> Option<Direction> {
        let move_dimension = Dimension::from_direction(head_move_direction);

        match move_dimension {
            Vertical if horizontal_diff > 0 => Some(Right),
            Vertical if horizontal_diff < 0 => Some(Left),
            Horizontal if vertical_diff > 0 => Some(Up),
            Horizontal if vertical_diff < 0 => Some(Down),
            _ => None,
        }
    }

    fn tail_adjustment(&self, head_direction: &Direction) -> Vec<Direction> {
        let (head, tail) = (self.roap.head, self.roap.tail);
        let x_diff = head.x as i32 - tail.x as i32;
        let y_diff = head.y as i32 - tail.y as i32;

        let tail_movement = if x_diff > 1 {
            Some(Right)
        } else if x_diff < -1 {
            Some(Left)
        } else if y_diff > 1 {
            Some(Up)
        } else if y_diff < -1 {
            Some(Down)
        } else {
            None
        };

        let tail_adjustment = Self::diagonal_tail_adjustment(x_diff, y_diff, head_direction);

        [tail_movement, tail_adjustment]
            .into_iter()
            .flatten()
            .collect::<Vec<Direction>>()
    }

    fn move_roap(&mut self, direction: &Direction) {
        if self.matrix.needs_expanding(&self.roap.head, direction) {
            self.expand();
        }

        self.move_roap_head(&direction);
        let tail_adjustment = self.tail_adjustment(&direction);
        for adjustment in tail_adjustment {
            self.move_roap_tail(&adjustment);
        }

        self.matrix.set(&self.roap.tail, true);
    }

    fn expand(&mut self) {
        let matrix_size_pre = self.matrix.size();
        self.matrix.expand();
        let matrix_size_post = self.matrix.size();
        let offset = (matrix_size_post - matrix_size_pre) / 2;
        self.roap.head.x += offset;
        self.roap.head.y += offset;
        self.roap.tail.x += offset;
        self.roap.tail.y += offset;
    }
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut matrix = &self
            .matrix
            .0
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, v)| match (x, y) {
                        _ if self.roap.head == (x, y) => "H",
                        _ if self.roap.tail == (x, y) => "T",
                        _ if *v => "#",
                        _ => ".",
                    })
                    .collect::<Vec<&str>>()
                    .join(" ")
            })
            .rev()
            .collect::<Vec<String>>()
            .join("\n");
        let mut matrix_with_newline = "\n".to_string();
        matrix_with_newline.push_str(matrix);
        write!(f, "{}", matrix_with_newline)
    }
}

fn part_one() {
    let file_path = "data.txt";
    let file = File::open(file_path).expect("File not found");
    let motions = BufReader::new(file).lines();
}

fn main() {
    part_one();

    let mut s = Simulation::new();
    let c = Coordinate { x: 0, y: 0 };
    println!("{}", s);
    s.move_roap(&Up);
    println!("{}", s);
    s.move_roap(&Right);
    println!("{}", s);
    s.move_roap(&Right);
    println!("{}", s);
    s.expand();
    println!("{}", s);
    s.expand();
    println!("{}", s);
}
