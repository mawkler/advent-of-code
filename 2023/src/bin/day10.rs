use self::Direction::{East, North, South, West};
use indoc::indoc;
use std::{fmt::Display, ops::Add};

#[derive(Debug, PartialEq)]
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

    fn get_connected_pipe(&self, coordinate: &Coordinate, direction: Direction) -> Option<&Pipe> {
        let neighbour_coordinate = coordinate + &direction.get_delta();

        self.get_tile(&neighbour_coordinate)
            .and_then(|neighbour_tile| match neighbour_tile {
                Tile::Pipe(pipe) => {
                    let Pipe(d1, d2) = pipe;

                    if d1.get_opposite() == direction || d2.get_opposite() == direction {
                        Some(pipe)
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

#[derive(Debug, PartialEq)]
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
    let next = maze.get_connected_pipe(&start, East);
    println!("next = {:#?}", next);
    println!("start = {:#?}", start);
}

#[cfg(test)]
mod tests {
    use crate::{Coordinate, Direction, Maze, Pipe};
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
            maze.get_connected_pipe(top_left, East),
            Some(&Pipe(East, West))
        );
        assert_eq!(
            maze.get_connected_pipe(top_left, South),
            Some(&Pipe(North, South))
        );
        assert!(maze.get_connected_pipe(top_left, West).is_none());
        assert!(maze.get_connected_pipe(top_left, North).is_none());

        let bottom_right = &Coordinate(3, 3);

        assert_eq!(
            maze.get_connected_pipe(bottom_right, West),
            Some(&Pipe(East, West))
        );
        assert_eq!(
            maze.get_connected_pipe(bottom_right, North),
            Some(&Pipe(North, South))
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
