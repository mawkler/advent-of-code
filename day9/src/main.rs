use self::Dimension::{Horizontal, Vertical};
use self::Direction::{Down, Left, Right, Up};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new() -> Coordinate {
        Coordinate { x: 0, y: 0 }
    }
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
            Down => coordinate.y <= 0,
            Left => coordinate.x <= 0,
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

    fn add_coordinate_margin(coordinate: &mut Coordinate, margin: usize) {
        coordinate.x += margin;
        coordinate.y += margin;
    }

    fn count_visited_coordinates(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|visited| **visited).count())
            .sum()
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
        match string {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => panic!("Direction not recognized"),
        }
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
struct Roap(Vec<Coordinate>);

impl Roap {
    fn new(size: u32) -> Roap {
        Roap((0..size).map(|_| Coordinate::new()).collect())
    }

    fn head(&self) -> &Coordinate {
        self.0.first().unwrap()
    }

    fn head_mut(&mut self) -> &mut Coordinate {
        self.0.first_mut().unwrap()
    }

    fn tail(&self) -> &Coordinate {
        self.0.last().unwrap()
    }

    fn tail_mut(&mut self) -> &mut Coordinate {
        self.0.last_mut().unwrap()
    }
}

struct Simulation {
    matrix: Matrix,
    roap: Roap,
}

impl Simulation {
    fn new(roap_size: u32) -> Self {
        Simulation {
            matrix: Matrix::new(),
            roap: Roap::new(roap_size),
        }
    }

    fn move_roap_knot(knot: &mut Coordinate, direction: &Direction) {
        match direction {
            Up => knot.y += 1,
            Down => knot.y -= 1,
            Left => knot.x -= 1,
            Right => knot.x += 1,
        };
    }

    fn diagonal_tail_adjustment(
        horizontal_diff: i32,
        vertical_diff: i32,
        head_move_direction: &Direction,
    ) -> Option<Direction> {
        if horizontal_diff.abs() <= 1 && vertical_diff.abs() <= 1 {
            return None;
        }

        match Dimension::from_direction(head_move_direction) {
            Vertical if horizontal_diff > 0 => Some(Right),
            Vertical if horizontal_diff < 0 => Some(Left),
            Horizontal if vertical_diff > 0 => Some(Up),
            Horizontal if vertical_diff < 0 => Some(Down),
            _ => None,
        }
    }

    fn trail_knot_adjustment(
        &self,
        lead: &Coordinate,
        trail: &Coordinate,
        lead_direction: &Direction,
    ) -> Vec<Direction> {
        let x_diff = lead.x as i32 - trail.x as i32;
        let y_diff = lead.y as i32 - trail.y as i32;

        let trail_movement = if x_diff > 1 {
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

        let trail_adjustment = Self::diagonal_tail_adjustment(x_diff, y_diff, lead_direction);

        [trail_movement, trail_adjustment]
            .into_iter()
            .flatten()
            .collect::<Vec<Direction>>()
    }

    fn get_knot_pair(&self, trail_index: usize) -> (&Coordinate, &Coordinate) {
        let (left, right) = self.roap.0.split_at(trail_index);
        let lead = left.last().unwrap();
        let trail = right.first().unwrap();
        (lead, trail)
    }

    fn get_knot_pair_mut(&mut self, trail_index: usize) -> (&mut Coordinate, &mut Coordinate) {
        let (left, right) = self.roap.0.split_at_mut(trail_index);
        let lead = left.last_mut().unwrap();
        let trail = right.first_mut().unwrap();
        (lead, trail)
    }

    fn move_roap(&mut self, direction: &Direction) {
        if self.matrix.needs_expanding(&self.roap.head(), direction) {
            self.expand();
        }

        Self::move_roap_knot(&mut self.roap.head_mut(), &direction);

        for i in 1..self.roap.0.len() {
            let (lead, trail) = self.get_knot_pair(i);
            let tail_adjustment = self.trail_knot_adjustment(lead, trail, &direction);

            let (_, trail) = self.get_knot_pair_mut(i);
            for adjustment in &tail_adjustment {
                Self::move_roap_knot(trail, &adjustment);
            }
        }

        self.matrix.set(&self.roap.tail(), true);
    }

    fn move_roap_count(&mut self, direction: Direction, count: u32) {
        for _ in 0..count {
            self.move_roap(&direction);
            println!("self: {:?}", self);
        }
    }

    fn expand(&mut self) {
        let matrix_size_pre = self.matrix.size();
        self.matrix.expand();
        let matrix_size_post = self.matrix.size();

        let margin = (matrix_size_post - matrix_size_pre) / 2;
        for knot in &mut self.roap.0 {
            Matrix::add_coordinate_margin(knot, margin);
        }
    }

    fn knot_string_from_coordinate(&self, coordinate: &Coordinate) -> Option<String> {
        self.roap
            .0
            .iter()
            .position(|knot| knot == coordinate)
            .map(|position| {
                if position == 0 {
                    "H".to_string()
                } else {
                    position.to_string()
                }
            })
    }
}

impl fmt::Debug for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let matrix = &self
            .matrix
            .0
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, v)| {
                        if let Some(s) = self.knot_string_from_coordinate(&Coordinate { x, y }) {
                            s
                        } else if *v {
                            "#".to_string()
                        } else {
                            ".".to_string()
                        }
                    })
                    .collect::<Vec<String>>()
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
    let movements = BufReader::new(file).lines();

    let mut simulation = Simulation::new(10);

    movements.for_each(|movement| {
        let movement = movement.unwrap();
        let (direction, count) = movement.split_once(' ').unwrap();
        let direction = Direction::from_str(direction);
        let count: u32 = count.parse().unwrap();

        simulation.move_roap_count(direction, count);
    });

    let count = simulation.matrix.count_visited_coordinates();

    assert_eq!(count, 6087);
    println!("count: {:?}", count);
}

fn main() {
    part_one();
}
