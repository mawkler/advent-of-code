use itertools::Itertools;

type Coordinate = (u32, u32);
type Area = u64;

fn parse(input: &str) -> impl Iterator<Item = Coordinate> {
    input.lines().map(|line| {
        let (x, y) = line.split_once(',').expect("has comma");
        let x = x.parse().expect("is numeric");
        let y = y.parse().expect("is numeric");

        (x, y)
    })
}

fn get_rectangle_area(corner1: &Coordinate, corner2: &Coordinate) -> Area {
    let x_distance = (corner1.0 as i32).abs_diff(corner2.0 as i32);
    let y_distance = (corner1.1 as i32).abs_diff(corner2.1 as i32);

    (x_distance as Area + 1) * (y_distance as Area + 1)
}

fn get_largest_rectangle_area(input: &str) -> Area {
    let rectangle_areas = parse(input).permutations(2).map(|pair| {
        let (c1, c2) = pair.iter().collect_tuple().expect("is pair");
        get_rectangle_area(c1, c2)
    });

    rectangle_areas.max().expect("iterator is non-empty")
}

pub fn main() {
    let input = include_str!("../../input/day9");

    println!("Part 1: {}", get_largest_rectangle_area(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_largest_rectangle_area() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(get_largest_rectangle_area(input), 50);
    }
}
