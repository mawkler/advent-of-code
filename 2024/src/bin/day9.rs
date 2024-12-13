use itertools::Itertools;

// Part 1
pub fn calculate_checksum(blocks: &str) -> usize {
    let mut disk_map = parse_disk_map(blocks);
    disk_map.move_file_blocks();

    disk_map.calculate_checksum()
}

// Part 2
pub fn calculate_checksum2(blocks: &str) -> usize {
    let mut disk_map = parse_disk_map(blocks);
    disk_map.move_files();

    disk_map.calculate_checksum()
}

#[derive(Debug)]
struct DiskMap(Vec<Block>);

#[derive(Clone, Debug, PartialEq)]
enum Block {
    FreeSpace,
    Id(usize),
}

impl DiskMap {
    fn calculate_checksum(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(position, block)| match block {
                Block::FreeSpace => 0,
                Block::Id(id) => position * id,
            })
            .sum()
    }

    fn swap_blocks(&mut self, from: usize, to: usize) {
        self.0.swap(from, to);
    }

    fn move_file_blocks(&mut self) {
        loop {
            let last_file_from_right_pos = self
                .0
                .iter()
                .rev()
                .position(|block| matches!(block, Block::Id(_)))
                .unwrap();

            let last_file_pos = self.0.len() - 1 - last_file_from_right_pos;
            let first_free_block_pos = self
                .0
                .iter()
                .position(|block| matches!(block, Block::FreeSpace))
                .unwrap();

            if last_file_pos <= first_free_block_pos {
                return;
            }

            self.swap_blocks(first_free_block_pos, last_file_pos);
        }
    }

    fn get_last_file_id(&self) -> usize {
        let Block::Id(last_id) = self
            .0
            .iter()
            .rev()
            .find(|block| matches!(block, Block::Id(_)))
            .unwrap()
        else {
            unreachable!()
        };

        *last_id
    }

    fn move_files(&mut self) {
        let last_file_id = self.get_last_file_id();

        for id in (0..=last_file_id).rev() {
            let Some((file_start, file_end)) = self.find_file_position(id) else {
                continue;
            };

            let file_size = file_end + 1 - file_start;
            let Some(free_space) = self.get_first_available_space(file_size) else {
                continue;
            };

            if free_space < file_start {
                self.swap_file(file_start, file_end, free_space);
            }
        }
    }

    fn find_file_position(&self, block_id: usize) -> Option<(usize, usize)> {
        let size = self.0.len();
        let last_file: Vec<_> = self
            .0
            .iter()
            .rev()
            .enumerate()
            .filter(|(_, block)| matches!(block, Block::Id(id) if *id == block_id))
            .map(move |(position, _)| size - 1 - position)
            .collect();

        Some((*last_file.last()?, *last_file.first()?))
    }

    fn get_first_available_space(&self, size: usize) -> Option<usize> {
        let (position, _) = self
            .0
            .windows(size)
            .find_position(|blocks| blocks.iter().all(|block| matches!(block, Block::FreeSpace)))?;

        Some(position)
    }

    fn swap_file(&mut self, file_start: usize, file_end: usize, to: usize) {
        let file_length = file_end + 1 - file_start;
        let file: Vec<_> = self.0.drain(file_start..file_end + 1).collect();
        let free_space: Vec<_> = self.0.drain(to..to + file_length).collect();

        self.0.splice(to..to, file);
        self.0.splice(file_start..file_start, free_space);
    }
}

fn parse_disk_map(disk_map: &str) -> DiskMap {
    // We need a Vec to be able to call `.chunks()`
    let blocks: Vec<_> = disk_map.chars().collect();
    let disk_map = blocks
        .chunks(2)
        .enumerate()
        .flat_map(|(id, chunk)| {
            let (files, free_space) = (chunk[0], chunk.get(1));

            let expanded_free_space = free_space.map(|free_space| {
                let free_space_size = free_space.to_digit(10).expect("Is numeric");
                expand_blocks(Block::FreeSpace, free_space_size)
            });

            let files_size = files.to_digit(10).expect("Is numeric");
            let expanded_files = expand_blocks(Block::Id(id), files_size);

            let expanded_blocks: Vec<_> = [Some(expanded_files), expanded_free_space]
                .into_iter()
                .flatten()
                .flatten()
                .collect();
            Some(expanded_blocks)
        })
        .flatten()
        .collect();

    DiskMap(disk_map)
}

fn expand_blocks(block: Block, size: u32) -> Vec<Block> {
    std::iter::repeat_n(block, size as usize).collect()
}

fn main() {
    let data = include_str!("../../data/day9");

    assert_eq!(6431472344710, calculate_checksum2(data));

    println!("Part 1: {}", calculate_checksum(data));
    println!("Part 2: {}", calculate_checksum2(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PartialEq for DiskMap {
        fn eq(&self, other: &Self) -> bool {
            itertools::equal(self.0.clone(), other.0.clone())
        }
    }

    impl From<&str> for DiskMap {
        fn from(blocks: &str) -> Self {
            let blocks = blocks
                .chars()
                .map(|block| match block {
                    '.' => Block::FreeSpace,
                    id => {
                        let id = id.to_digit(10).expect("Is numeric");
                        Block::Id(id as usize)
                    }
                })
                .collect();

            DiskMap(blocks)
        }
    }

    impl core::fmt::Display for DiskMap {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let disk_map: String = self
                .0
                .iter()
                .map(|block| match block {
                    Block::FreeSpace => ".".to_string(),
                    Block::Id(id) => id.to_string(),
                })
                .collect();

            write!(f, "{disk_map}")
        }
    }

    #[test]
    fn parses_blocks() {
        assert_eq!(DiskMap::from("0..111....22222"), parse_disk_map("12345"));

        let blocks = "2333133121414131402";
        let expected = "00...111...2...333.44.5555.6666.777.888899";
        assert_eq!(DiskMap::from(expected), parse_disk_map(blocks));
    }

    #[test]
    fn swaps_block() {
        let mut disk_map = DiskMap::from("12345");

        disk_map.swap_blocks(0, 1);

        assert_eq!(DiskMap::from("21345"), disk_map)
    }

    #[test]
    fn moves_file_blocks() {
        let mut disk_map = parse_disk_map("12345");
        disk_map.move_file_blocks();

        assert_eq!(DiskMap::from("022111222......"), disk_map);

        let mut disk_map = parse_disk_map("2333133121414131402");
        disk_map.move_file_blocks();

        let expected = DiskMap::from("0099811188827773336446555566..............");
        assert_eq!(expected, disk_map);
    }

    #[test]
    fn calculates_checksum() {
        assert_eq!(1928, calculate_checksum("2333133121414131402"));
    }

    #[test]
    fn moves_file_blocks_with_zeros() {
        let mut disk_map = parse_disk_map("101111");
        disk_map.move_file_blocks();

        assert_eq!(DiskMap::from("012.."), disk_map);
    }

    #[test]
    fn moves_file_blocks_with_tens() {
        let mut disk_map = parse_disk_map("101010101010101010111");
        disk_map.move_file_blocks();

        let expected = DiskMap(vec![
            Block::Id(0),
            Block::Id(1),
            Block::Id(2),
            Block::Id(3),
            Block::Id(4),
            Block::Id(5),
            Block::Id(6),
            Block::Id(7),
            Block::Id(8),
            Block::Id(9),
            Block::Id(10),
            Block::FreeSpace,
        ]);

        assert_eq!(expected, disk_map);
    }

    #[test]
    fn calculates_checksum_with_zeros() {
        assert_eq!(5, calculate_checksum("101011"));
    }

    #[test]
    fn finds_file_position() {
        let disk_map = DiskMap::from("000..111");
        let last_file = disk_map.find_file_position(1);

        assert_eq!(Some((5, 7)), last_file);

        let disk_map = DiskMap::from("000..111.....222222....");
        let last_file = disk_map.find_file_position(2);

        assert_eq!(Some((13, 18)), last_file);
    }

    #[test]
    fn swaps_file() {
        let mut disk_map = DiskMap::from("000...111");
        disk_map.swap_file(6, 8, 3);

        assert_eq!(DiskMap::from("000111..."), disk_map);

        let mut disk_map = DiskMap::from("0...111...22....");
        disk_map.swap_file(10, 11, 1);

        assert_eq!(DiskMap::from("022.111........."), disk_map)
    }

    #[test]
    fn finds_first_available_space() {
        let disk_map = DiskMap::from("..000.....2222..");

        assert_eq!(Some(5), disk_map.get_first_available_space(4));
    }

    #[test]
    fn moves_files() {
        let mut disk_map = DiskMap::from("000.....2222..");
        disk_map.move_files();
        assert_eq!(DiskMap::from("0002222......."), disk_map);

        let mut disk_map = DiskMap::from("000.....2222..33...4444.5");
        disk_map.move_files();
        assert_eq!(DiskMap::from("00054444222233..........."), disk_map);
    }

    #[test]
    fn moves_file_of_size_1() {
        let mut disk_map = DiskMap::from("000.1111..2");
        disk_map.move_files();
        assert_eq!(DiskMap::from("00021111..."), disk_map);
    }

    #[test]
    fn moves_files2() {
        let mut disk_map = parse_disk_map("2333133121414131402");
        println!("   input: {}", &disk_map);

        disk_map.move_files();
        let expected = DiskMap::from("00992111777.44.333....5555.6666.....8888..");
        println!("expected: {}", expected);
        println!("     got: {}", disk_map);

        assert_eq!(expected, disk_map);
    }

    #[test]
    fn moves_files3() {
        let mut disk_map = DiskMap::from("00.1112...333.44");

        disk_map.move_files();

        let expected = DiskMap::from("002111.44.333...");
        assert_eq!(expected, disk_map, "expected: {expected}, got: {disk_map}");
    }

    #[test]
    fn doesnt_move_file_that_doesnt_fit() {
        let mut disk_map = DiskMap::from("000...2222..");

        disk_map.move_files();

        assert_eq!(DiskMap::from("000...2222.."), disk_map);
    }

    #[test]
    fn calculates_checksum2() {
        assert_eq!(2858, calculate_checksum2("2333133121414131402"));
    }
}
