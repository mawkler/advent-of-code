use itertools::Itertools;
use std::fmt::Display;

type Coordinate = (i32, i32);

struct Warehouse {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

struct Tile {
    tile_type: TileType,
    coordinate: Coordinate,
}

#[derive(Debug)]
enum TileType {
    Box,
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

impl Direction {
    fn get_neighbour(&self, (x, y): Coordinate) -> Coordinate {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
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
                TileType::Box => tile.coordinate.1 * 100 + tile.coordinate.0,
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

        if !self.push_tiles(robot, direction) {
            // No tiles were pushed
            return;
        }

        let neighbour = direction.get_neighbour(robot);
        *self.get_robot_mut() = neighbour;
    }

    /// Returns true if tiles were pushable, otherwise false
    fn push_tiles(&mut self, from: Coordinate, direction: &Direction) -> bool {
        let Some(tiles) = self.get_pushable_tiles(from, direction) else {
            // Tiles aren't pushable
            return false;
        };

        self.move_tiles(&tiles, direction);
        true
    }

    fn get_pushable_tiles(
        &mut self,
        from: Coordinate,
        direction: &Direction,
    ) -> Option<Vec<Coordinate>> {
        let mut tiles = vec![];
        let mut current = from;

        // TODO: can we make this less imperative and more functional?
        loop {
            let neighbour_pos = direction.get_neighbour(current);

            let Some(neighbour) = self.get_tile(neighbour_pos) else {
                return Some(tiles);
            };

            match neighbour.tile_type {
                TileType::Box => {
                    tiles.push(neighbour_pos);
                    current = neighbour_pos;
                    continue;
                }
                TileType::Wall => return None,
                TileType::Robot => {
                    unreachable!("There is only one robot, it can't have a robot neighbour")
                }
            }
        }
    }

    fn move_tiles(&mut self, coordinates: &[Coordinate], direction: &Direction) {
        for coordinate in coordinates {
            let tile = self.get_tile_mut(*coordinate).unwrap();
            tile.coordinate = direction.get_neighbour(tile.coordinate)
        }
    }

    fn move_tile(&mut self, tile: Coordinate, direction: Direction) {
        let new_position = direction.get_neighbour(tile);
        let tile = self.get_tile_mut(tile).unwrap();

        tile.move_to(new_position);
    }
}

impl From<&str> for Warehouse {
    fn from(warehouse: &str) -> Self {
        let mut lines = warehouse.lines();
        let height = lines.clone().count();

        Self {
            tiles: parse_tiles(warehouse).collect(),
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
    fn move_to(&mut self, coordinate: Coordinate) {
        self.coordinate = coordinate;
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tile = match self.tile_type {
            TileType::Box => "O",
            TileType::Wall => "#",
            TileType::Robot => "@",
        };
        write!(f, "{tile}")
    }
}

fn parse_tile(tile: char, coordinate: Coordinate) -> Option<Tile> {
    let tile_type = match tile {
        '#' => Some(TileType::Wall),
        'O' => Some(TileType::Box),
        '@' => Some(TileType::Robot),
        '.' => None,
        other => panic!("Unrecognized character '{other}'"),
    }?;

    let tile = Tile {
        tile_type,
        coordinate,
    };
    Some(tile)
}

fn parse_tiles(warehouse: &str) -> impl Iterator<Item = Tile> + use<'_> {
    warehouse.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .flat_map(move |(x, tile)| parse_tile(tile, (x as i32, y as i32)))
    })
}

fn parse_moves(moves: &str) -> impl Iterator<Item = Direction> + use<'_> {
    moves.chars().filter(|&m| m != '\n').map(|m| m.into())
}

fn parse_warehouse(input: &str) -> (Warehouse, impl Iterator<Item = Direction> + use<'_>) {
    let (warehouse, moves) = input.split_once("\n\n").unwrap();
    let directions = parse_moves(moves);

    (warehouse.into(), directions)
}

fn perform_moves(input: &str) -> String {
    let (mut warehouse, moves) = parse_warehouse(input);
    moves.for_each(|m| {
        //
        warehouse.move_robot(&m)
    });

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

        assert_eq!(warehouse, Warehouse::from(warehouse).to_string())
    }

    #[test]
    fn parses_moves() {
        let input = indoc! {"
            <vv>
            vvv<
            ><>v
        "};

        let expected = vec![
            Direction::Left,
            Direction::Down,
            Direction::Down,
            Direction::Right,
            Direction::Down,
            Direction::Down,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::Left,
            Direction::Right,
            Direction::Down,
        ];
        let moves = parse_moves(input);

        assert_equal(expected, moves);
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
        let warehouse = indoc! {"
            .OO.O.
            ......
        "};
        let mut warehouse = Warehouse::from(warehouse);

        let pushable_tiles = warehouse.get_pushable_tiles((0, 0), &Direction::Right);
        assert_eq!(Some(vec![(1, 0), (2, 0)]), pushable_tiles)
    }

    #[test]
    fn gets_unpushable_tiles() {
        let warehouse = indoc! {"
            OOO#
            ....
        "};
        let mut warehouse = Warehouse::from(warehouse);

        let pushable_tiles = warehouse.get_pushable_tiles((0, 0), &Direction::Right);
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
        let mut warehouse = Warehouse::from(warehouse);

        let expected = indoc! {"
            ....
            ....
            .@..
            .O..
            .O..
            .O.."
        };

        warehouse.move_robot(&Direction::Down);
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
