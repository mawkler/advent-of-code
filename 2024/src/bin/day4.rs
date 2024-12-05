struct WordSearch(String);

impl WordSearch {
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

    fn count_xmas_forward_backward(line: String) -> usize {
        let xmas = "XMAS";
        let line_backward: String = line.chars().rev().collect();

        line_backward.matches(xmas).count() + line.matches(xmas).count()
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

#[derive(Clone)]
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

    println!("Part 1: {}", word_search.count_xmas());
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
        let word_search = indoc! {"
            abc
            def
            ghi
        "};

        let word_search = WordSearch(word_search.to_string());
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
}
