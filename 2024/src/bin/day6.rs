use core::time;
use itertools::Itertools;
use std::{fmt::Display, thread::sleep};

type Coordinate = (u32, u32);

struct Lab {
    map: Map,
    guard: Guard,
}

struct Guard {
    position: Coordinate,
    direction: Direction,
}

impl Guard {
    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }
}

enum State {
    Done,
    NotDone,
}

impl Lab {
    pub fn count_visited_tiles(&mut self) -> usize {
        self.simulate();
        self.map
            .0
            .iter()
            .map(|line| line.iter().filter(|&tile| *tile == Tile::Visited).count())
            .sum()
    }

    fn simulate(&mut self) {
        // TODO: perhaps just inline `move_guard()` here?
        while let State::NotDone = self.move_guard() {
            sleep(time::Duration::from_millis(3));
            self.map.set_tile_visited(self.guard.position);
        }
    }

    fn new(map: &str) -> Self {
        let map = Map::from(map);
        let guard_coordinate = map
            .0
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    // The guard is on the only visited coordinate
                    .find(|&(_, tile)| *tile == Tile::Visited)
                    .map(|(x, _)| (x as u32, y as u32))
            })
            .unwrap();

        let guard = Guard {
            position: guard_coordinate,
            direction: Direction::Up,
        };

        Self { map, guard }
    }

    fn move_guard(&mut self) -> State {
        let Some(new_position) = self.get_new_guard_coordinate() else {
            return State::Done;
        };

        if let Tile::Obstacle = self.map.get_tile(new_position) {
            self.guard.turn_right();
        } else {
            self.guard.position = new_position;
        }

        State::NotDone
    }

    fn get_new_guard_coordinate(&self) -> Option<Coordinate> {
        let width = self.map.0.first().expect("Map has tiles").len();
        let height = self.map.0.len();

        let (x, y) = self.guard.position;
        let (x, y) = (x as i32, y as i32);

        let (new_x, new_y) = match self.guard.direction {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        };

        if new_x.is_negative() || new_y.is_negative() {
            return None;
        }

        if new_x >= width as i32 || new_y >= height as i32 {
            return None;
        }

        Some((new_x as u32, new_y as u32))
    }
}

impl Display for Lab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lab: String = self
            .map
            .0
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(x, position)| {
                        if self.guard.position == (x as u32, y as u32) {
                            self.guard.direction.to_string()
                        } else {
                            position.to_string()
                        }
                    })
                    .collect::<String>()
            })
            .join("\n");

        write!(f, "{lab}")
    }
}

#[derive(Debug)]
struct Map(Vec<Vec<Tile>>);

impl Map {
    fn get_tile(&self, (x, y): Coordinate) -> &Tile {
        let line = self.0.get(y as usize).expect("Map has tiles");
        line.get(x as usize).expect("Map has tiles")
    }

    fn set_tile_visited(&mut self, (x, y): Coordinate) {
        let line = self.0.get_mut(y as usize).expect("Map has tiles");
        let tile = line.get_mut(x as usize).expect("Map has tiles");

        *tile = Tile::Visited;
    }
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction = match self {
            Direction::Up => "^",
            Direction::Right => ">",
            Direction::Down => "v",
            Direction::Left => "<",
        };
        write!(f, "{direction}")
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Obstacle,
    Visited,
    Unvisited,
}

impl From<char> for Tile {
    fn from(tile: char) -> Self {
        match tile {
            '.' => Tile::Unvisited,
            '#' => Tile::Obstacle,
            '^' => Tile::Visited,
            other => panic!("Invalid input character '{other}'"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tile = match self {
            Tile::Obstacle => "#",
            Tile::Visited => "X",
            Tile::Unvisited => ".",
        };
        write!(f, "{tile}")
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();

        Map(map)
    }
}

fn main() {
    let data = include_str!("../../data/day6");
    let mut lab = Lab::new(data);

    println!("Part 1: {}", lab.count_visited_tiles());
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const MAP: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#..."
    };

    #[test]
    fn gets_initial_guard_tile() {
        let guard_position = Lab::new(MAP).guard.position;

        assert_eq!(guard_position, (4, 6));
    }

    #[test]
    fn moves_guard() {
        let map = indoc! {"
            ....#.....
            ....^....#
            ..........
            ..#.......
            .......#..
            ..........
            .#........
            ........#.
            #.........
            ......#...
        "};

        let mut lab = Lab::new(map);
        assert_eq!((4, 1), lab.guard.position);
        assert_eq!(Direction::Up, lab.guard.direction);

        lab.move_guard();

        assert_eq!((4, 1), lab.guard.position);
        assert_eq!(Direction::Right, lab.guard.direction);

        lab.move_guard();

        assert_eq!((5, 1), lab.guard.position);
        assert_eq!(Direction::Right, lab.guard.direction);
    }

    #[test]
    fn displays_lab() {
        let lab = Lab::new(MAP).to_string();

        assert_eq!(MAP, lab);
    }
}
