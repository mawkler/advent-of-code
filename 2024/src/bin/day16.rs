use std::collections::HashMap;

enum Tile {
    Wall,
    Empty,
}

type Coordinate = (usize, usize);

struct Maze {
    layout: Vec<Vec<Tile>>,
    start: Coordinate,
    end: Coordinate,
}

fn parse(maze: String) -> Maze {
    let mut layout = HashMap::new();
    let mut start = None;
    let mut end = None;

    let _ = maze.lines().enumerate().map(|(y, line)| {
        let _ = line.chars().enumerate().map(|(x, c)| {
            let coordinate = (x, y);
            let tile = match c {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'S' => {
                    start = Some((x, y));
                    Tile::Empty
                }
                'E' => {
                    end = Some((x, y));
                    Tile::Empty
                }
                c => panic!("unrecognized character {c}"),
            };

            layout.insert(coordinate, tile);
        });
    });

    todo!()
}

// fn main() {}

use std::fmt::Display;

fn displayable<T: Display>(t: T) -> dyn Display {
    t
}

fn main() {
    let s = String::from("hello");
    let mut s2 = displayable(s);
    s2.push_str(" world");
    println!("{s2}");
}
