type Coordinate = (i32, i32);

struct Map {
    map: String,
    height: usize,
    width: usize,
}

impl Map {
    pub fn new(map: String) -> Self {
        let height = map.lines().count();
        let width = map
            .chars()
            .position(|position| position == '\n')
            .expect("has multiple lines");

        Self { map, height, width }
    }

    fn get(&self, (x, y): Coordinate) -> Option<char> {
        let (x, y) = (x.try_into().ok()?, y.try_into().ok()?);
        let line = self.map.lines().nth(y)?;

        line.chars().nth(x)
    }

    fn is_occupied(&self, position: Coordinate) -> Option<bool> {
        Some(self.get(position).is_some_and(|x| x != '.'))
    }

    fn get_neighbour_positions(&self, (x, y): Coordinate) -> [Coordinate; 8] {
        [
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
            (x - 1, y + 1),
            (x - 1, y),
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
        ]
    }

    // ..@@.@@@@.
    // @@@.@.@.@@
    // @@@@@.@.@@
    // @.@@@@..@.
    // @@.@@@@.@@
    // .@@@@@@@.@
    // .@.@.@.@@@
    // @.@@@.@@@@
    // .@@@@@@@@.
    // @.@.@@@.@.

    // ..xx.xx@x.
    // x@@.@.@.@@
    // @@@@@.x.@@
    // @.@@@@..@.
    // x@.@@@@.@x
    // .@@@@@@@.@
    // .@.@.@.@@@
    // x.@@@.@@@@
    // .@@@@@@@@.
    // x.x.@@@.x.

    fn is_accessible(&self, position: Coordinate) -> bool {
        let neighbours = self.get_neighbour_positions(position);
        let occupied_neighbours = neighbours.iter().filter(|&&neighbour| {
            self.is_occupied(neighbour)
                .is_some_and(|is_occupied| is_occupied)
        });

        occupied_neighbours.count() < 4
    }

    fn count_accessible_rolls(&self) -> usize {
        let accessible_rolls = (0..self.height).flat_map(|y| {
            (0..self.width).map(move |x| {
                // Converting i32 coordinates to usize
                let (x, y) = match (x.try_into(), y.try_into()) {
                    (Ok(x), Ok(y)) => (x, y),
                    _ => return false,
                };

                if self.get((x, y)).is_some_and(|position| position != '@') {
                    // The position is not a roll
                    return false;
                }

                self.is_accessible((x, y))
            })
        });
        accessible_rolls.filter(|&position| position).count()
    }
}

fn main() {
    let input = include_str!("../../input/day4");
    let map = Map::new(input.to_string());

    println!("Part 1: {}", map.count_accessible_rolls());
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn initializes_map() {
        let map = Map::new(MAP.to_string());
        assert_eq!(map.height, 10);
        assert_eq!(map.width, 10);
    }

    #[test]
    fn validates_accessible_position() {
        let map = Map::new(MAP.to_string());

        let accessible_positions = [
            (2, 0),
            (3, 0),
            (5, 0),
            (6, 0),
            (8, 0),
            (6, 2),
            (0, 4),
            (9, 4),
            (0, 7),
            (0, 9),
            (2, 9),
            (8, 9),
        ];

        for (x, y) in accessible_positions {
            assert!(map.is_accessible((x, y)), "({x}, {y}) should be accessible");
        }

        assert!(!map.is_accessible((1, 1)))
    }

    #[test]
    fn counts_accessible_rolls() {
        let map = Map::new(MAP.to_string());

        assert_eq!(map.count_accessible_rolls(), 13);
    }
}
