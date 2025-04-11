use itertools::Itertools;
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

type Coordinate = (i32, i32);

struct Warehouse {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

#[derive(/* Debug,  */ PartialEq)]
struct Tile {
    tile_type: TileType,
    coordinate: Coordinate,
}

#[derive(Debug, PartialEq)]
enum TileType {
    BoxLeft,  // Left part of box
    BoxRight, // Right part of box
    Wall,
    Robot,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.tile_type, self.coordinate)
    }
}

impl Direction {
    fn get_neighbour_pos(&self, (x, y): Coordinate) -> Coordinate {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }
}

impl std::ops::Add<&Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, direction: &Direction) -> Coordinate {
        match direction {
            Direction::Up => (self.0, self.1 - 1),
            Direction::Down => (self.0, self.1 + 1),
            Direction::Left => (self.0 - 1, self.1),
            Direction::Right => (self.0 + 1, self.1),
        }
    }
}

impl From<char> for Direction {
    fn from(direction: char) -> Self {
        match direction {
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '^' => Direction::Up,
            other => panic!("Unrecognized character {other}"),
        }
    }
}

impl Warehouse {
    fn sum_box_gps_coordinates(&self) -> i32 {
        self.tiles
            .iter()
            .map(|tile| match tile.tile_type {
                TileType::BoxLeft => tile.coordinate.1 * 100 + tile.coordinate.0,
                _ => 0,
            })
            .sum()
    }

    fn get_tile(&self, coordinate: Coordinate) -> Option<&Tile> {
        self.tiles.iter().find(|t| t.coordinate == coordinate)
    }

    fn get_tile_mut(&mut self, coordinate: Coordinate) -> Option<&mut Tile> {
        self.tiles.iter_mut().find(|t| t.coordinate == coordinate)
    }

    fn get_robot(&self) -> Coordinate {
        self.tiles
            .iter()
            .find(|t| matches!(t.tile_type, TileType::Robot))
            .map(|t| t.coordinate)
            .unwrap()
    }

    fn get_robot_mut(&mut self) -> &mut Coordinate {
        self.tiles
            .iter_mut()
            .find(|t| matches!(t.tile_type, TileType::Robot))
            .map(|t| &mut t.coordinate)
            .unwrap()
    }

    fn move_robot(&mut self, direction: &Direction) {
        let robot = self.get_robot();

        if !self.push_boxes(robot, direction) {
            // Boxes couldn't be pushed
            return;
        }

        let neighbour = direction.get_neighbour_pos(robot);
        *self.get_robot_mut() = neighbour;
    }

    /// Returns true if tiles were pushable, otherwise false
    fn push_boxes(&mut self, from: Coordinate, direction: &Direction) -> bool {
        let Some(boxes) = self.get_pushable_boxes_aux(from, direction, HashSet::new()) else {
            // Boxes aren't pushable
            return false;
        };

        self.move_tiles(&boxes.into_iter().collect_vec(), direction);
        true
    }

    fn get_pushable_boxes(
        &self,
        from: Coordinate,
        direction: &Direction,
    ) -> Option<HashSet<(i32, i32)>> {
        self.get_pushable_boxes_aux(from, direction, HashSet::new())
    }

    fn get_pushable_boxes_aux(
        &self,
        from: Coordinate,
        direction: &Direction,
        mut boxes: HashSet<Coordinate>,
    ) -> Option<HashSet<Coordinate>> {
        let neighbour_pos = direction.get_neighbour_pos(from);

        let Some(neighbour) = self.get_tile(neighbour_pos) else {
            return Some(boxes);
        };

        let neighbour_sibling_pos = neighbour.get_box_sibling_pos();

        match neighbour.tile_type {
            TileType::BoxLeft | TileType::BoxRight => {
                boxes.insert(neighbour_pos);
                boxes.insert(neighbour_sibling_pos);
                self.get_pushable_boxes_aux(neighbour_pos, direction, boxes)
                    .and_then(|boxes| {
                        // TODO: we keep checking siblings back and forth when it's horizontal
                        self.get_pushable_boxes_aux(neighbour_sibling_pos, direction, boxes)
                    })
            }
            TileType::Wall => None,
            TileType::Robot => {
                unreachable!("There is only one robot, it can't have a robot neighbour")
            }
        }
    }

    fn move_tiles(&mut self, coordinates: &[Coordinate], direction: &Direction) {
        for coordinate in coordinates {
            let tile = self.get_tile_mut(*coordinate).unwrap();
            tile.coordinate = direction.get_neighbour_pos(tile.coordinate)
        }
    }

    fn move_tile(&mut self, tile: Coordinate, direction: Direction) {
        let new_position = direction.get_neighbour_pos(tile);
        let tile = self.get_tile_mut(tile).unwrap();

        tile.move_to(new_position);
    }
}

impl From<&str> for Warehouse {
    fn from(warehouse: &str) -> Self {
        let mut lines = warehouse.lines();
        let height = lines.clone().count();

        let tiles = parse_tiles(warehouse).collect();

        Self {
            tiles,
            width: lines.nth(0).unwrap().len(),
            height,
        }
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let warehouse: String = (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(move |x| {
                        if let Some(tile) = self.get_tile((x as i32, y as i32)) {
                            tile.to_string()
                        } else {
                            ".".to_string()
                        }
                    })
                    .join("")
            })
            .join("\n");

        write!(f, "{warehouse}")
    }
}

impl Tile {
    fn new(tile_type: TileType, coordinate: Coordinate) -> Self {
        Self {
            tile_type,
            coordinate,
        }
    }

    fn move_to(&mut self, coordinate: Coordinate) {
        self.coordinate = coordinate;
    }

    fn get_box_sibling_pos(&self) -> Coordinate {
        match self.tile_type {
            TileType::BoxLeft => (self.coordinate.0 + 1, self.coordinate.1),
            TileType::BoxRight => (self.coordinate.0 - 1, self.coordinate.1),
            _ => panic!("{self} is not a box"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tile = match self.tile_type {
            TileType::BoxLeft => "[",
            TileType::BoxRight => "]",
            TileType::Wall => "#",
            TileType::Robot => "@",
        };
        write!(f, "{tile}")
    }
}

fn parse_tile(tile: char, coordinate: Coordinate) -> (Option<Tile>, Option<Tile>) {
    let (left_tile, right_tile) = match tile {
        '#' => (Some(TileType::Wall), Some(TileType::Wall)),
        'O' => (Some(TileType::BoxLeft), Some(TileType::BoxRight)),
        '@' => (Some(TileType::Robot), None),
        '.' => (None, None),
        other => panic!("Unrecognized character '{other}'"),
    };

    let left_tile = left_tile.map(|tile| Tile::new(tile, coordinate));
    let right_tile = right_tile.map(|tile| Tile::new(tile, (coordinate.0 + 1, coordinate.1)));

    (left_tile, right_tile)
}

fn parse_tiles(warehouse: &str) -> impl Iterator<Item = Tile> + use<'_> {
    warehouse.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().flat_map(move |(x, tile)| {
            let (left, right) = parse_tile(tile, (x as i32 * 2, y as i32));
            [left, right].into_iter().flatten()
        })
    })
}

fn parse_warehouse(input: &str) -> (Warehouse, impl Iterator<Item = Direction> + use<'_>) {
    fn parse_moves(moves: &str) -> impl Iterator<Item = Direction> + use<'_> {
        moves.chars().filter(|&m| m != '\n').map(|m| m.into())
    }

    let (warehouse, moves) = input.split_once("\n\n").unwrap();
    let directions = parse_moves(moves);

    (warehouse.into(), directions)
}

fn perform_moves(input: &str) -> String {
    let (mut warehouse, moves) = parse_warehouse(input);
    moves.for_each(|m| warehouse.move_robot(&m));

    warehouse.to_string()
}

fn main() {
    let data = include_str!("../../data/day15");

    let warehouse = perform_moves(data);
    let gps_sum = Warehouse::from(warehouse.as_str()).sum_box_gps_coordinates();

    println!("Part 1: {}", gps_sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use itertools::assert_equal;

    impl Warehouse {
        fn parse_parsed_tile(tile: char, coordinate: Coordinate) -> std::option::Option<Tile> {
            let tile = match tile {
                '#' => Some(TileType::Wall),
                '[' => Some(TileType::BoxLeft),
                ']' => Some(TileType::BoxRight),
                '@' => Some(TileType::Robot),
                '.' => None,
                other => panic!("Unrecognized character '{other}'"),
            };

            tile.map(|tile| Tile::new(tile, coordinate))
        }

        fn from_parsed(warehouse: &str) -> Self {
            let tiles = warehouse.lines().enumerate().flat_map(|(y, line)| {
                line.chars().enumerate().flat_map(move |(x, tile)| {
                    Warehouse::parse_parsed_tile(tile, (x as i32, y as i32))
                })
            });

            let lines = warehouse.lines();
            let height = lines.clone().count();

            Self {
                tiles: tiles.collect(),
                width: warehouse.lines().nth(0).unwrap().len(),
                height,
            }
        }
    }

    #[test]
    fn parses_tiles() {
        let warehouse = indoc! {"
            .O.#
        "};
        let warehouse = parse_tiles(warehouse).collect_vec();

        let expected = [
            Tile {
                tile_type: TileType::BoxLeft,
                coordinate: (2, 0),
            },
            Tile {
                tile_type: TileType::BoxRight,
                coordinate: (3, 0),
            },
            Tile {
                tile_type: TileType::Wall,
                coordinate: (6, 0),
            },
            Tile {
                tile_type: TileType::Wall,
                coordinate: (7, 0),
            },
        ];

        assert_equal(expected, warehouse)
    }

    #[test]
    fn parses_and_displays_warehouse() {
        let warehouse = indoc! {"
            ##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########"
        };

        let expected_warehouse = indoc! {"
            ####################
            ##....[]....[]..[]##
            ##............[]..##
            ##..[][]....[]..[]##
            ##....[]@.....[]..##
            ##[]##....[]......##
            ##[]....[]....[]..##
            ##..[][]..[]..[][]##
            ##........[]......##
            ####################"
        };
        let warehouse = Warehouse::from(warehouse).to_string();
        assert_eq!(expected_warehouse, warehouse)
    }

    #[test]
    fn moves_tile() {
        let warehouse = indoc! {"
            @..#
            ....
        "};
        let mut warehouse = Warehouse::from(warehouse);

        warehouse.move_tile((0, 0), Direction::Right);

        let expected = indoc! {"
            .@.#
            ...."
        };

        assert_eq!(expected, warehouse.to_string())
    }

    #[test]
    fn moves_tile_queue() {
        let warehouse = indoc! {"
            OO..
            ....
        "};
        let mut warehouse = Warehouse::from(warehouse);

        warehouse.move_tiles(&[(0, 0), (1, 0)], &Direction::Right);

        let expected = indoc! {"
            .OO.
            ...."
        };
        assert_eq!(expected, warehouse.to_string())
    }

    #[test]
    fn gets_pushable_tiles() {
        let warehouse = Warehouse::from_parsed(indoc! {"
            ..[][]..
            ........
        "});

        let pushable_tiles = warehouse.get_pushable_boxes((1, 0), &Direction::Right);
        assert_eq!(
            HashSet::from([(2, 0), (3, 0), (4, 0), (5, 0)]),
            pushable_tiles.unwrap(),
        );
    }

    #[test]
    fn gets_pushable_tiles_upwards() {
        let warehouse = indoc! {"
            []
            []
            ..
        "};
        let warehouse = Warehouse::from_parsed(warehouse);

        let pushable_tiles = warehouse.get_pushable_boxes((1, 2), &Direction::Up);
        assert_eq!(
            HashSet::from([(0, 0), (1, 0), (0, 1), (1, 1)]),
            pushable_tiles.unwrap(),
        );
    }

    #[test]
    fn gets_pushable_diagonal_tiles() {
        let warehouse = indoc! {"
            .[]
            [].
            ...
        "};
        let warehouse = Warehouse::from_parsed(warehouse);

        let pushable_tiles = warehouse.get_pushable_boxes((0, 2), &Direction::Up);
        assert_eq!(
            HashSet::from([(0, 1), (1, 1), (1, 0), (2, 0)]),
            pushable_tiles.unwrap(),
        );
    }

    #[test]
    fn gets_pushable_diagonal_tiles_longer() {
        let warehouse = indoc! {"
            [][]
            .[].
            ....
        "};
        let warehouse = Warehouse::from_parsed(warehouse);

        let pushable_tiles = warehouse.get_pushable_boxes((1, 2), &Direction::Up);
        assert_eq!(
            HashSet::from([(0, 0), (1, 0), (2, 0), (3, 0), (1, 1), (2, 1)]),
            pushable_tiles.unwrap(),
        );
    }

    #[test]
    fn gets_pushable_diagonal_tiles_longest() {
        let warehouse = indoc! {"
            []..[]
            .[][].
            ..[]..
            ......
        "};
        let warehouse = Warehouse::from_parsed(warehouse);

        let pushable_tiles = warehouse.get_pushable_boxes((3, 3), &Direction::Up);
        assert_eq!(
            HashSet::from([
                (0, 0),
                (1, 0),
                (4, 0),
                (5, 0),
                (1, 1),
                (2, 1),
                (3, 1),
                (4, 1),
                (2, 2),
                (3, 2)
            ]),
            pushable_tiles.unwrap(),
        );
    }

    #[test]
    fn gets_unpushable_tiles() {
        let warehouse = indoc! {"
            OOO#
            ....
        "};
        let warehouse = Warehouse::from(warehouse);

        let pushable_tiles = warehouse.get_pushable_boxes((0, 0), &Direction::Right);
        assert_eq!(None, pushable_tiles)
    }

    #[test]
    fn moves_robot() {
        let warehouse = indoc! {"
            ....
            .@..
            .O..
            .O..
            ....
            .O..
        "};
        let warehouse = Warehouse::from(warehouse);
        println!("{}", &warehouse);

        warehouse.move_robot(&Direction::Down);
        println!();
        println!("{}", &warehouse);

        let expected = indoc! {"
            ........
            ........
            ..@.....
            ..[]....
            ..[]....
            ..[]...."
        };
        assert_eq!(expected, warehouse.to_string());
    }

    #[test]
    fn performs_moves() {
        let input = indoc! {"
            ########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########

            <^^>>>vv<v>>v<<
        "
        };

        let expected = indoc! {"
            ########
            #....OO#
            ##.....#
            #.....O#
            #.#O@..#
            #...O..#
            #...O..#
            ########"
        };
        assert_eq!(expected, perform_moves(input));
    }

    #[test]
    fn performs_many_moves() {
        let input = indoc! {"
            ##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########

            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        "
        };

        let expected = indoc! {"
            ##########
            #.O.O.OOO#
            #........#
            #OO......#
            #OO@.....#
            #O#.....O#
            #O.....OO#
            #O.....OO#
            #OO....OO#
            ##########"
        };
        assert_eq!(expected, perform_moves(input));
    }

    #[test]
    fn sums_box_gps_coordinates() {
        let warehouse = indoc! {"
            ##########
            #.O.O.OOO#
            #........#
            #OO......#
            #OO@.....#
            #O#.....O#
            #O.....OO#
            #O.....OO#
            #OO....OO#
            ##########"
        };
        assert_eq!(10092, Warehouse::from(warehouse).sum_box_gps_coordinates())
    }
}
