use itertools::Itertools;
use std::{collections::HashSet, fmt::Display};

// Part 1
pub fn count_visited_tiles(map: &str) -> usize {
    let mut lab = Lab::new(map);

    while let State::Simulating = lab.move_guard() {
        lab.map.set_tile_visited(lab.guard.position);
    }

    lab.map
        .0
        .iter()
        .map(|line| line.iter().filter(|&tile| *tile == Tile::Visited).count())
        .sum()
}

// Part 2
fn count_looping_obstacle_placements(map: &str) -> usize {
    // I create an initial lab here just to determine the height/width
    let lab = Lab::new(map);
    let width = lab.map.0.first().expect("Map has tiles").len() as u32;
    let height = lab.map.0.len() as u32;

    (0..height)
        .flat_map(|y| {
            (0..width)
                // Exclude guard start position
                .skip_while(move |&x| (x, y) == lab.guard_start_state.position)
                .map(move |x| {
                    let mut lab = Lab::new(map);
                    lab.map.place_obstacle_tile((x, y));
                    lab.simulate_guard_looping()
                })
        })
        .filter(|state| *state == State::LoopFound)
        .count()
}

type Coordinate = (u32, u32);

struct Lab {
    map: Map,
    guard: Guard,
    guard_start_state: Guard,
}

#[derive(Clone, PartialEq, Eq)]
struct Guard {
    position: Coordinate,
    direction: Direction,
    visited_states: HashSet<(Coordinate, Direction)>,
}

impl Guard {
    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    fn has_looped(&self) -> bool {
        let current_state = &(self.position, self.direction.clone());
        self.visited_states.contains(current_state)
    }
}

#[derive(PartialEq, Debug)]
enum State {
    Simulating,
    GuardLeft,
    LoopFound,
}

impl Lab {
    fn simulate(&mut self) {
        while let State::Simulating = self.move_guard() {
            self.map.set_tile_visited(self.guard.position);
        }
    }

    fn simulate_guard_looping(&mut self) -> State {
        loop {
            let lab_state = self.move_guard();

            if let State::GuardLeft = lab_state {
                return lab_state;
            }

            if self.guard.has_looped() {
                return State::LoopFound;
            }

            let guard_state = (self.guard.position, self.guard.direction.clone());
            self.guard.visited_states.insert(guard_state);
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

        let guard_direction = Direction::Up;
        let guard = Guard {
            position: guard_coordinate,
            direction: guard_direction.clone(),
            visited_states: HashSet::from([(guard_coordinate, guard_direction)]),
        };

        Self {
            map,
            guard_start_state: guard.clone(),
            guard,
        }
    }

    fn move_guard(&mut self) -> State {
        let Some(new_position) = self.get_new_guard_coordinate() else {
            return State::GuardLeft;
        };

        if let Tile::Obstacle = self.map.get_tile(new_position) {
            self.guard.turn_right();
        } else {
            self.guard.position = new_position;
        }

        State::Simulating
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
        let lab = self
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

    fn get_tile_mut(&mut self, (x, y): Coordinate) -> &mut Tile {
        let line = self.0.get_mut(y as usize).expect("Map has tiles");
        line.get_mut(x as usize).expect("Map has tiles")
    }

    fn set_tile_visited(&mut self, coordinate: Coordinate) {
        *self.get_tile_mut(coordinate) = Tile::Visited;
    }

    fn place_obstacle_tile(&mut self, coordinate: Coordinate) {
        *self.get_tile_mut(coordinate) = Tile::Obstacle;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

    println!("Part 1: {}", count_visited_tiles(data));
    println!("Part 2: {}", count_looping_obstacle_placements(data));
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

    #[test]
    fn detects_guard_left() {
        let mut lab = Lab::new(MAP);

        let result = lab.simulate_guard_looping();
        assert_eq!(State::GuardLeft, result);
    }

    #[test]
    fn detects_guard_loop() {
        let mut lab = Lab::new(MAP);

        lab.map.place_obstacle_tile((3, 6));
        let result = lab.simulate_guard_looping();

        assert_eq!(State::LoopFound, result);
    }

    #[test]
    fn detects_guard_loop_outside_starting_position() {
        let mut lab = Lab::new(MAP);

        lab.map.place_obstacle_tile((7, 7));
        let result = lab.simulate_guard_looping();

        assert_eq!(State::LoopFound, result);
    }

    #[test]
    fn counts_guard_loops() {
        let count = count_looping_obstacle_placements(MAP);

        assert_eq!(6, count);
    }
}
