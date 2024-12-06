use itertools::Itertools;

#[derive(Clone)]
struct WordSearch(String);

impl WordSearch {
    // Part 1
    pub fn count_xmas(self) -> usize {
        let main_diagonals = Box::new(self.diagonals(&Diagonal::Main));
        let anti_diagonals = Box::new(self.diagonals(&Diagonal::Anti));
        let columns = Box::new(self.columns());
        let lines = Box::new(self.lines());

        let iterators: Vec<Box<dyn Iterator<Item = String>>> =
            vec![main_diagonals, anti_diagonals, columns, lines];

        iterators
            .into_iter()
            .map(|iter| iter.map(Self::count_xmas_forward_backward).sum::<usize>())
            .sum()
    }

    // Part 2
    pub fn count_cross_mas(&self) -> usize {
        let diagonal_mas_coordinates = [Diagonal::Main, Diagonal::Anti].iter().map(|direction| {
            self.diagonals(direction)
                .enumerate()
                .flat_map(move |(diagonal, line)| {
                    self.get_mas_coordinates(line, diagonal, direction)
                })
                .collect::<Vec<_>>()
        });

        let mas_counts_map = diagonal_mas_coordinates.into_iter().flatten().counts();

        mas_counts_map.values().filter(|&&count| count > 1).count()
    }

    fn get_mas_coordinates(
        &self,
        line: String,
        diagonal: usize,
        direction: &Diagonal,
    ) -> Vec<(usize, usize)> {
        line.match_indices("MAS")
            .chain(line.match_indices("SAM"))
            .map(move |(index_on_diagonal, _)| {
                self.coordinate_from_diagonal(
                    diagonal,
                    // The `+ 1` is to get the position of `A`
                    index_on_diagonal + 1,
                    direction,
                )
            })
            .collect::<Vec<_>>()
    }
    fn count_xmas_forward_backward(line: String) -> usize {
        let xmas = "XMAS";
        let line_backward: String = line.chars().rev().collect();

        line_backward.matches(xmas).count() + line.matches(xmas).count()
    }

    fn coordinate_from_diagonal(
        &self,
        diagonal: usize,
        position: usize,
        direction: &Diagonal,
    ) -> (usize, usize) {
        let width = self.get_width();

        match *direction {
            Diagonal::Anti => {
                let padding = if diagonal < width {
                    0
                } else {
                    diagonal - width + 1
                };

                (diagonal - position - padding, position + padding)
            }
            Diagonal::Main => {
                if diagonal < width {
                    let x = (width - 1) - diagonal + position;
                    (x, position)
                } else {
                    let y = diagonal - (width - 1) + position;
                    (position, y)
                }
            }
        }
    }

    fn get_letter(&self, x: i32, y: i32) -> Option<char> {
        if x.is_negative() || y.is_negative() {
            return None;
        }

        let (x, y) = (x as usize, y as usize);
        self.0.lines().nth(y).and_then(|line| line.chars().nth(x))
    }

    fn diagonals<'a>(&'a self, diagonal: &'a Diagonal) -> impl Iterator<Item = String> + 'a {
        DiagonalIterator::new(self, diagonal)
    }

    fn columns(&self) -> impl Iterator<Item = String> + '_ {
        ColumnIterator {
            word_search: self,
            position: 0,
        }
    }

    fn lines(&self) -> impl Iterator<Item = String> + '_ {
        self.0.lines().map(ToString::to_string)
    }

    fn get_height(&self) -> usize {
        self.0.lines().count()
    }

    fn get_width(&self) -> usize {
        self.0.lines().next().expect("Must have lines").len()
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Diagonal {
    Main, // Northeast
    Anti, // Northwest
}

struct DiagonalIterator<'a> {
    word_search: &'a WordSearch,
    position: usize,
    diagonal: &'a Diagonal,
}

impl<'a> DiagonalIterator<'a> {
    fn new(word_search: &'a WordSearch, diagonal: &'a Diagonal) -> Self {
        Self {
            word_search,
            position: 0,
            diagonal,
        }
    }

    fn get_diagonal(&self, position: usize) -> Option<String> {
        let height = self.word_search.get_height() as i32;
        let width = self.word_search.get_width() as i32;

        let x_y_pairs: Vec<_> = if *self.diagonal == Diagonal::Anti {
            let position: i32 = position as i32 + 1;
            (0..position).rev().zip(0..position).collect()
        } else {
            let position = position as i32;
            (width - 1 - position..width).zip(0..height).collect()
        };

        let diagonal: String = x_y_pairs
            .into_iter()
            .filter_map(|(x, y)| self.word_search.get_letter(x, y))
            .collect();

        if diagonal.is_empty() {
            None
        } else {
            Some(diagonal)
        }
    }
}

impl Iterator for DiagonalIterator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let diagonal = self.get_diagonal(self.position);
        self.position += 1;
        diagonal
    }
}

struct ColumnIterator<'a> {
    word_search: &'a WordSearch,
    position: usize,
}

impl Iterator for ColumnIterator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let height = self.word_search.get_height();
        let width = self.word_search.get_width();

        if self.position >= width {
            return None;
        }

        let column = (0..height)
            .flat_map(|y| self.word_search.get_letter(self.position as i32, y as i32))
            .collect();

        self.position += 1;
        Some(column)
    }
}

fn main() {
    let data = include_str!("../../data/day4");
    let word_search = WordSearch(data.to_string());

    println!("Part 1: {}", word_search.clone().count_xmas());
    println!("Part 2: {}", word_search.count_cross_mas());
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use itertools::{assert_equal, Itertools};

    const WORD_SEARCH: &str = indoc! {"
        ..X...
        .SAMX.
        .A..A.
        XMAS.S
        .X....
    "};

    const SMALL_WORD_SEARCH: &str = indoc! {"
        abc
        def
        ghi
    "};

    #[test]
    fn finds_letter() {
        let word_search = WordSearch(WORD_SEARCH.to_string());

        assert_eq!(Some('.'), word_search.get_letter(0, 0));
        assert_eq!(Some('X'), word_search.get_letter(2, 0));
        assert_eq!(Some('.'), word_search.get_letter(0, 4));
        assert_eq!(Some('S'), word_search.get_letter(1, 1));
        assert_eq!(Some('S'), word_search.get_letter(5, 3));
    }

    #[test]
    fn gets_anti_diagonal() {
        let word_search = WordSearch(WORD_SEARCH.to_string());
        let iterator = DiagonalIterator::new(&word_search, &Diagonal::Anti);

        let diagonal = iterator.get_diagonal(2);
        assert_eq!(diagonal, Some("XS.".to_string()));

        let diagonal = iterator.get_diagonal(5);
        assert_eq!(diagonal, Some(".X.AX".to_string()));
    }

    #[test]
    fn gets_anti_diagonals() {
        let word_search = WordSearch(WORD_SEARCH.to_string());
        let diagonals = word_search.diagonals(&Diagonal::Anti).collect_vec();

        let expected = vec![
            ".", "..", "XS.", ".AAX", ".M.M.", ".X.AX", ".AS.", "...", "S.", ".",
        ];
        assert_equal(expected, diagonals);
    }

    #[test]
    fn gets_main_diagonals() {
        let word_search = WordSearch(WORD_SEARCH.to_string());
        let diagonals = word_search.diagonals(&Diagonal::Main).collect_vec();

        assert_equal(
            vec![
                ".", "..", ".X.", "XMAS", ".A...", ".S.S.", ".AA.", ".M.", "XX", ".",
            ],
            diagonals,
        );
    }

    #[test]
    fn gets_columns() {
        let word_search = WordSearch(SMALL_WORD_SEARCH.to_string());
        let columns = word_search.columns().collect_vec();

        assert_equal(vec!["adg", "beh", "cfi"], columns);
    }

    #[test]
    fn counts_xmas() {
        let word_search = WordSearch(WORD_SEARCH.to_string());
        assert_eq!(4, word_search.count_xmas());
    }

    #[test]
    fn counts_xmas_large() {
        let word_search = indoc! {"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        "};
        let word_search = WordSearch(word_search.to_string());

        assert_eq!(18, word_search.count_xmas());
    }

    #[test]
    fn counts_mas() {
        let word_search = indoc! {"
            .M.S......
            ..A..MSMS.
            .M.S.MAA..
            ..A.ASMSM.
            .M.S.M....
            ..........
            S.S.S.S.S.
            .A.A.A.A..
            M.M.M.M.M.
            ..........
        "};
        let word_search = WordSearch(word_search.to_string());

        assert_eq!(9, word_search.count_cross_mas())
    }

    #[test]
    fn converts_anti_diagonals_to_coordinates() {
        let word_search = WordSearch(SMALL_WORD_SEARCH.to_string());
        let anti = &Diagonal::Anti;

        assert_eq!((1, 0), word_search.coordinate_from_diagonal(1, 0, anti));
        assert_eq!((0, 1), word_search.coordinate_from_diagonal(1, 1, anti));
        assert_eq!((2, 0), word_search.coordinate_from_diagonal(2, 0, anti));
        assert_eq!((1, 1), word_search.coordinate_from_diagonal(2, 1, anti));
        assert_eq!((0, 2), word_search.coordinate_from_diagonal(2, 2, anti));
        assert_eq!((1, 2), word_search.coordinate_from_diagonal(3, 1, anti));
        assert_eq!((2, 2), word_search.coordinate_from_diagonal(4, 0, anti));
    }

    #[test]
    fn converts_main_diagonals_to_coordinates() {
        let word_search = WordSearch(SMALL_WORD_SEARCH.to_string());
        let main = &Diagonal::Main;

        assert_eq!((1, 0), word_search.coordinate_from_diagonal(1, 0, main));
        assert_eq!((2, 1), word_search.coordinate_from_diagonal(1, 1, main));
        assert_eq!((0, 0), word_search.coordinate_from_diagonal(2, 0, main));
        assert_eq!((1, 1), word_search.coordinate_from_diagonal(2, 1, main));
        assert_eq!((2, 2), word_search.coordinate_from_diagonal(2, 2, main));
        assert_eq!((0, 1), word_search.coordinate_from_diagonal(3, 0, main));
        assert_eq!((1, 2), word_search.coordinate_from_diagonal(3, 1, main));
        assert_eq!((0, 2), word_search.coordinate_from_diagonal(4, 0, main));
    }
}
