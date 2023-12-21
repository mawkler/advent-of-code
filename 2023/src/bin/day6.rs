type Time = u32;
type Distance = u64;

#[derive(Debug)]
struct Race {
    time: Time,
    record_distance: Distance,
}

impl Race {
    fn get_best_charge_up_times(&self) -> impl Iterator<Item = Time> + '_ {
        (1..self.time).filter(|&charge_up_time| self.run(charge_up_time) > self.record_distance)
    }

    fn run(&self, charge_up_time: Time) -> Distance {
        (charge_up_time * (self.time - charge_up_time)) as Distance
    }
}

impl From<(u64, u64)> for Race {
    fn from((time, record_distance): (u64, u64)) -> Self {
        Race {
            time: time as Time,
            record_distance,
        }
    }
}

fn parse_line(line: &str) -> impl Iterator<Item = u64> + '_ {
    let (_, numbers) = line.split_once(':').expect("Must exist");
    numbers
        .split_ascii_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
}

fn parse(string: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    let mut lines = string.lines();
    let times = lines.next().expect("Must exist");
    let distances = lines.next().expect("Must exist");

    let times = parse_line(times);
    let distances = parse_line(distances);

    times.into_iter().zip(distances)
}

fn main() {
    let data = include_str!("../../data/day6");
    let product: usize = parse(data)
        .map(Race::from)
        .map(|race| race.get_best_charge_up_times().count())
        .product();

    println!("Part 1: {}", product);
}

#[cfg(test)]
mod tests {
    use crate::{parse, Race};
    use indoc::indoc;

    #[test]
    fn parses_input() {
        let data = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};
        let result: Vec<_> = parse(data).collect();

        assert_eq!(result, vec![(7, 9), (15, 40), (30, 200)]);
    }

    #[test]
    fn beats_record() {
        let race = Race {
            time: 7,
            record_distance: 9,
        };
        let times: Vec<_> = race.get_best_charge_up_times().collect();

        assert_eq!(times, vec![2, 3, 4, 5]);
    }
}
