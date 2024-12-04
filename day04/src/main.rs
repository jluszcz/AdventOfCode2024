use std::iter::Iterator;
use std::sync::LazyLock;

use anyhow::Result;
use log::{debug, trace};

static XMAS: LazyLock<Vec<char>> = LazyLock::new(|| "XMAS".chars().collect());

#[derive(Debug, Default)]
struct Grid(Vec<Vec<char>>);

impl Grid {
    fn push(&mut self, line: Vec<char>) {
        self.0.push(line);
    }

    fn occurrences_from(&self, word: &[char], x: usize, y: usize) -> usize {
        let mut occurrences = 0;

        if self.0[y][x] != word[0] {
            return occurrences;
        }

        for neighbor in util::grid_neighbors(&self.0, x, y, true) {
            trace!(
                "Matched {} in {word:?} at ({x}, {y}), checking {neighbor:?}",
                word[0]
            );

            let mut neighbor = neighbor;
            let mut i = 1;
            while i < word.len() {
                let expected = word[i];

                let (n_x, n_y) = neighbor.into();

                let actual = self.0[n_y][n_x];

                // Going in the direction of neighbor didn't find the word
                if actual != expected {
                    trace!("Failed to match {expected} in {word:?} at ({n_x}, {n_y}): {actual}");
                    break;
                }

                i += 1;

                // Figure out the next neighbor to check
                if let Some(n) = neighbor.next(&self.0) {
                    neighbor = n;
                } else {
                    break;
                }

                trace!("Matched {expected} in {word:?} at ({n_x}, {n_y}), checking {neighbor:?}");
            }

            // If we've successfully gone along a direction to the point we reached the end of the
            // word, we're done
            if i == word.len() {
                debug!("Found {word:?} from ({x}, {y}) via {neighbor:?}");
                occurrences += 1;
            }
        }

        occurrences
    }

    fn count_occurrences(&self) -> usize {
        let mut occurrences = 0;

        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                occurrences += self.occurrences_from(&XMAS, x, y);
            }
        }

        occurrences
    }
}

impl From<Vec<String>> for Grid {
    fn from(value: Vec<String>) -> Self {
        let mut grid = Grid::default();
        for line in value {
            let line = line.chars().collect::<Vec<_>>();
            grid.push(line);
        }
        grid
    }
}

fn main() -> Result<()> {
    let input = util::init()?;
    let grid = Grid::from(input);

    let occurrences = grid.count_occurrences();
    println!("Occurrences: {occurrences}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_simplified() -> Result<()> {
        util::init_test_logger()?;

        let input = r"..X...
.SAMX.
.A..A.
XMAS.S
.X....";

        let input = input.split("\n").map(|s| s.to_string()).collect::<Vec<_>>();
        let grid = Grid::from(input);

        assert_eq!(1, grid.occurrences_from(&XMAS, 2, 0));
        assert_eq!(1, grid.occurrences_from(&XMAS, 4, 1));
        assert_eq!(1, grid.occurrences_from(&XMAS, 0, 3));
        assert_eq!(1, grid.occurrences_from(&XMAS, 1, 4));

        Ok(())
    }

    #[test]
    fn part_1_example() -> Result<()> {
        let input = util::init_test()?;
        let grid = Grid::from(input);

        assert_eq!(1, grid.occurrences_from(&XMAS, 4, 0));
        assert_eq!(1, grid.occurrences_from(&XMAS, 5, 0));
        assert_eq!(1, grid.occurrences_from(&XMAS, 4, 1));
        assert_eq!(2, grid.occurrences_from(&XMAS, 9, 3));
        assert_eq!(1, grid.occurrences_from(&XMAS, 0, 4));
        assert_eq!(2, grid.occurrences_from(&XMAS, 6, 4));
        assert_eq!(1, grid.occurrences_from(&XMAS, 0, 5));
        assert_eq!(1, grid.occurrences_from(&XMAS, 6, 5));
        assert_eq!(1, grid.occurrences_from(&XMAS, 1, 9));
        assert_eq!(2, grid.occurrences_from(&XMAS, 3, 9));
        assert_eq!(3, grid.occurrences_from(&XMAS, 5, 9));
        assert_eq!(2, grid.occurrences_from(&XMAS, 9, 9));

        assert_eq!(18, grid.count_occurrences());

        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2_example() -> Result<()> {
        Ok(())
    }
}
