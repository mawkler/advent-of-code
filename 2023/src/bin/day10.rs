use self::Direction::{East, North, South, West};
use indoc::indoc;
use itertools::Itertools;
use num::Integer;
use std::{fmt::Display, ops::Add};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinate(i32, i32);

impl Add for &Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: &Coordinate) -> Self::Output {
        Coordinate(self.0 + rhs.0, self.1 + rhs.1)
    }
}

struct Maze(Vec<Vec<Tile>>);

impl Maze {
    fn get_tile(&self, Coordinate(x, y): &Coordinate) -> Option<&Tile> {
        self.0.get(*y as usize).and_then(|row| row.get(*x as usize))
    }

    fn get_pipe(&self, coordinate: &Coordinate) -> Option<&Pipe> {
        match self.get_tile(coordinate) {
            Some(Tile::Pipe(pipe)) => Some(pipe),
            _ => None,
        }
    }

    fn get_connected_pipe(
        &self,
        coordinate: &Coordinate,
        direction: &Direction,
    ) -> Option<Coordinate> {
        let neighbour_coordinate = coordinate + &direction.get_delta();

        self.get_tile(&neighbour_coordinate)
            .and_then(move |neighbour_tile| match neighbour_tile {
                Tile::Pipe(pipe) => {
                    if pipe.points(&direction.get_opposite()) {
                        Some(neighbour_coordinate)
                    } else {
                        None
                    }
                }
                Tile::Start => Some(neighbour_coordinate),
                _ => None,
            })
    }

    fn find_start(&self) -> Coordinate {
        self.0
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .position(|tile| tile == &Tile::Start)
                    .map(|x| Coordinate(x as _, y as _))
            })
            .expect("Start tile should exist")
    }

    fn follow_pipe<'a>(&'a self, coordinate: Coordinate, direction: &'a Direction) -> PipeIterator {
        PipeIterator {
            maze: self,
            tile_coordinate: coordinate,
            flow_direction: direction,
            done: false,
        }
    }

    fn get_loop_length(&self, start: Coordinate, direction: Direction) -> Option<usize> {
        let iterator_start = &start + &direction.get_delta();
        self.follow_pipe(iterator_start, &direction)
            .enumerate()
            .last()
            .filter(|(_, last)| self.get_tile(last) == Some(&Tile::Start))
            .map(|(count, _)| count)
    }

    fn find_loop_length(&self) -> Option<usize> {
        let start = self.find_start();
        // We only need to check half of the directions, since the pipe is looping back
        [North, East]
            .into_iter()
            .find_map(|direction| self.get_loop_length(start, direction))
    }

    fn find_loop(&self) -> Option<Vec<Coordinate>> {
        let start = self.find_start();

        [North, East].into_iter().find_map(|direction| {
            let iterator_start = &start + &direction.get_delta();
            let pipe: Vec<_> = self.follow_pipe(iterator_start, &direction).collect();

            self.get_tile(pipe.last()?)
                .filter(|&tile| tile == &Tile::Start)
                .map(|_| pipe)
        })
    }

    fn is_on_pipe_loop(&self, coordinate: &Coordinate) -> bool {
        let pipe_loop = self.find_loop().expect("Loop should exist");
        pipe_loop.iter().any(|c| c == coordinate)
    }

    fn find_tiles_inside_loop(&self) {
        let tiles = self.0.iter().map(|row| {
            row.iter()
                .scan(0, |loop_crossings, tile| -> Option<bool> {
                    let crosses_pipe = match tile {
                        Tile::Pipe(pipe)
                            if !pipe.points(&East)
                                || pipe.points(&West) && !pipe.points(&East)
                                || pipe.points(&East) && !pipe.points(&West) =>
                        {
                            true
                        }
                        Tile::Start => true,
                        _ => false,
                    };

                    if matches!(tile, Tile::Pipe(_)) || matches!(tile, Tile::Start) {
                        if crosses_pipe {
                            *loop_crossings += 1;
                        }
                        println!("tile = {}, loop_crossings = {:#?}", tile, loop_crossings);
                        Some(false)
                    } else {
                        Some(loop_crossings.is_odd())
                    }
                })
                .collect_vec()
        });

        let tiles = tiles.collect_vec();
        // println!("tiles = {:?}", tiles);

        println!("{}", Maze::to_string(tiles));

        todo!();
    }

    fn to_string(maze: Vec<Vec<bool>>) -> String {
        maze.iter()
            .map(|row| {
                row.iter()
                    .map(|&tile| if tile { "I" } else { "O" })
                    .join("")
            })
            .join("\n")
    }
}

fn divide_rounding_up(a: i32, b: i32) -> i32 {
    a / b + (a % b).signum()
}

struct PipeIterator<'a> {
    maze: &'a Maze,
    flow_direction: &'a Direction,
    tile_coordinate: Coordinate,
    done: bool,
}

impl<'a> Iterator for PipeIterator<'a> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        match self.maze.get_tile(&self.tile_coordinate)? {
            Tile::Ground => None,
            Tile::Start => {
                self.done = true;
                Some(self.tile_coordinate)
            }
            Tile::Pipe(pipe) => {
                let tile_coordinate = self.tile_coordinate;

                self.flow_direction = pipe.get_end_direction(self.flow_direction);
                let neighbour_coordinate = self
                    .maze
                    .get_connected_pipe(&tile_coordinate, self.flow_direction)?;
                self.tile_coordinate = neighbour_coordinate;

                Some(tile_coordinate)
            }
        }
    }
}

impl From<&str> for Maze {
    fn from(string: &str) -> Self {
        let maze = string
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();

        Maze(maze)
    }
}

impl std::fmt::Debug for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let maze = &self
            .0
            .iter()
            .map(|row| row.iter().map(|tile| tile.to_string()).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        Display::fmt(maze, f)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_delta(&self) -> Coordinate {
        match self {
            North => Coordinate(0, -1),
            East => Coordinate(1, 0),
            South => Coordinate(0, 1),
            West => Coordinate(-1, 0),
        }
    }

    fn get_opposite(&self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Pipe(Direction, Direction);

impl Pipe {
    fn get_end_direction(&self, direction: &Direction) -> &Direction {
        if direction.get_opposite() == self.0 {
            &self.1
        } else {
            &self.0
        }
    }

    fn points(&self, direction: &Direction) -> bool {
        self.0 == *direction || self.1 == *direction
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe(North, South) => write!(f, "║"),
            Pipe(East, West) => write!(f, "═"),
            Pipe(North, East) => write!(f, "╚"),
            Pipe(North, West) => write!(f, "╝"),
            Pipe(South, West) => write!(f, "╗"),
            Pipe(South, East) => write!(f, "╔"),
            Pipe(d1, d2) => panic!("Unexpected pipe '{:?}/{:?}' found", d1, d2),
        }
    }
}

#[derive(PartialEq)]
enum Tile {
    Pipe(Pipe),
    Ground,
    Start,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Tile::Pipe(Pipe(North, South)),
            '-' => Tile::Pipe(Pipe(East, West)),
            'L' => Tile::Pipe(Pipe(North, East)),
            'J' => Tile::Pipe(Pipe(North, West)),
            '7' => Tile::Pipe(Pipe(South, West)),
            'F' => Tile::Pipe(Pipe(South, East)),
            '.' => Tile::Ground,
            'S' => Tile::Start,
            other => panic!("Unexpected character '{}' found", other),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pipe(pipe) => pipe.fmt(f),
            Self::Ground => write!(f, " "),
            Self::Start => write!(f, "S"),
        }
    }
}

fn main() {
    let data = include_str!("../../data/day10");

    let maze: Maze = data.into();
    let loop_length = maze.find_loop_length();

    let farthest_away_position = divide_rounding_up(loop_length.unwrap() as _, 2);
    println!("Part 1: {}", farthest_away_position);

    let maze: Maze = indoc! {"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "}
    .into();

    let loorp = maze.find_loop();
    // println!("loorp = {:#?}", loorp);

    maze.find_tiles_inside_loop();
}

#[cfg(test)]
mod tests {
    use crate::{Coordinate, Direction, Maze};
    use indoc::indoc;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;
    use Direction::{East, North, South, West};

    #[test]
    fn identifies_continuing_pipe() {
        let maze: Maze = indoc! {"
            .....
            .F-7.
            .|.|.
            .L-J.
            .....
        "}
        .into();

        let top_left = &Coordinate(1, 1);

        assert_eq!(
            maze.get_connected_pipe(top_left, &East),
            Some(Coordinate(2, 1))
        );
        assert_eq!(
            maze.get_connected_pipe(top_left, &South),
            Some(Coordinate(1, 2))
        );
        assert!(maze.get_connected_pipe(top_left, &West).is_none());
        assert!(maze.get_connected_pipe(top_left, &North).is_none());

        let bottom_right = Coordinate(3, 3);

        assert_eq!(
            maze.get_connected_pipe(&bottom_right, &West),
            Some(Coordinate(2, 3))
        );
        assert_eq!(
            maze.get_connected_pipe(&bottom_right, &North),
            Some(Coordinate(3, 2))
        );
    }

    #[test]
    fn finds_start() {
        let maze: Maze = indoc! {"
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        "}
        .into();

        assert_eq!(maze.find_start(), Coordinate(1, 1));

        let maze: Maze = indoc! {"
            .....
            .F-7.
            .|.|.
            .LSJ.
            .....
        "}
        .into();

        assert_eq!(maze.find_start(), Coordinate(2, 3));
    }

    #[test]
    fn follows_pipe() {
        let maze: Maze = indoc! {"
            .....
            .S-7.
            .L-J.
            .....
        "}
        .into();

        let result = maze.follow_pipe(Coordinate(1, 2), &South).collect_vec();
        let expected = [
            Coordinate(1, 2),
            Coordinate(2, 2),
            Coordinate(3, 2),
            Coordinate(3, 1),
            Coordinate(2, 1),
            Coordinate(1, 1),
        ];

        assert_eq!(result, expected);
    }
}
