use indoc::indoc;
use std::fmt::Display;

struct Maze(Vec<Vec<Tile>>);

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
        std::fmt::Display::fmt(
            &self
                .0
                .iter()
                .map(|row| row.iter().map(|tile| tile.to_string()).collect::<String>())
                .collect::<Vec<_>>()
                .join("\n"),
            f,
        )
    }
}

enum Pipe {
    Vertcial,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

enum Tile {
    Pipe(Pipe),
    Ground,
    Start,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Tile::Pipe(Pipe::Vertcial),
            '-' => Tile::Pipe(Pipe::Horizontal),
            'L' => Tile::Pipe(Pipe::NorthEast),
            'J' => Tile::Pipe(Pipe::NorthWest),
            '7' => Tile::Pipe(Pipe::SouthWest),
            'F' => Tile::Pipe(Pipe::SouthEast),
            '.' => Tile::Ground,
            'S' => Tile::Start,
            other => panic!("Unexpected character '{}' found", other),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pipe(pipe) => match pipe {
                Pipe::Vertcial => write!(f, "║"),
                Pipe::Horizontal => write!(f, "═"),
                Pipe::NorthEast => write!(f, "╚"),
                Pipe::NorthWest => write!(f, "╝"),
                Pipe::SouthWest => write!(f, "╗"),
                Pipe::SouthEast => write!(f, "╔"),
            },
            Self::Ground => write!(f, " "),
            Self::Start => write!(f, "S"),
        }
    }
}

fn main() {
    let data = indoc! {"
        .....
        .F-7.
        .|.|.
        .L-J.
        .....
    "};

    let maze: Maze = data.into();
    println!("{:?}", maze);
}
