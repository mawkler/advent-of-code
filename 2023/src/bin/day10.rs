use self::Direction::{East, North, South, West};
use indoc::indoc;
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
                    let Pipe(d1, d2) = pipe;

                    if d1.get_opposite() == *direction || d2.get_opposite() == *direction {
                        Some(neighbour_coordinate)
                    } else {
                        None
                    }
                }
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

    fn follow_pipe(self, coordinate: &Coordinate, direction: &Direction) -> PipeIterator {
        PipeIterator {
            maze: self,
            pipe_coordinate: *coordinate,
            flow_direction: *direction,
        }
    }
}

// TODO: change to using references, and not cloning
struct PipeIterator {
    maze: Maze,
    pipe_coordinate: Coordinate,
    flow_direction: Direction,
}

impl Iterator for PipeIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        let pipe_tile = self.maze.get_tile(&self.pipe_coordinate)?;

        let neighbour_coordinate = self
            .maze
            .get_connected_pipe(&self.pipe_coordinate, &self.flow_direction)?;
        let neighbour_pipe = self.maze.get_pipe(&neighbour_coordinate)?;

        self.pipe_coordinate = neighbour_coordinate;
        self.flow_direction = *neighbour_pipe.get_end_direction(&self.flow_direction);

        Some(neighbour_coordinate)
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

// let _ = [North, East, South, West].iter().filter(|&direction| {
//     let todo!();
// });

fn main() {
    let data = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    let maze: Maze = data.into();
    let start = maze.find_start();
    let iter = maze.follow_pipe(&start, &South);
    println!("start = {:#?}", start);

    for pipe in iter {
        println!("pipe = {:#?}", pipe);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Coordinate, Direction, Maze};
    use indoc::indoc;
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
}
