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
            .map_while(|(position, id)| match id {
                Block::FreeSpace => None,
                Block::Id(id) => Some(position * id),
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

fn move_file_blocks(blocks: &str) -> DiskMap {
    let mut blocks = parse_disk_map(blocks);
    blocks.move_file_blocks();

    blocks
}

fn expand_blocks(block: Block, size: u32) -> Vec<Block> {
    std::iter::repeat_n(block, size as usize).collect()
}

fn calculate_checksum(blocks: &str) -> usize {
    let mut blocks = parse_disk_map(blocks);
    blocks.move_file_blocks();

    blocks.calculate_checksum()
}

fn main() {
    let data = include_str!("../../data/day9");

    println!("Part 1: {}", calculate_checksum(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PartialEq for DiskMap {
        fn eq(&self, other: &Self) -> bool {
            itertools::equal(self.0.clone(), other.0.clone())
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
    fn swaps_character() {
        let mut disk_map = DiskMap::from("12345");

        disk_map.swap_blocks(0, 1);

        assert_eq!(DiskMap::from("21345"), disk_map)
    }

    #[test]
    fn moves_file_blocks() {
        assert_eq!(DiskMap::from("022111222......"), move_file_blocks("12345"));

        let expected = DiskMap::from("0099811188827773336446555566..............");
        assert_eq!(expected, move_file_blocks("2333133121414131402"));
    }

    #[test]
    fn calculates_checksum() {
        assert_eq!(1928, calculate_checksum("2333133121414131402"));
    }

    #[test]
    fn moves_file_blocks_with_zeros() {
        assert_eq!(DiskMap::from("012.."), move_file_blocks("101111"));
    }

    #[test]
    fn moves_file_blocks_with_tens() {
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

        assert_eq!(expected, move_file_blocks("101010101010101010111"));
    }

    #[test]
    fn calculates_checksum_with_zeros() {
        assert_eq!(5, calculate_checksum("101011"));
    }
}
