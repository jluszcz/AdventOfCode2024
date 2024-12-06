use std::iter::Iterator;

use anyhow::Result;
use log::{debug, trace};

use util::Neighbor;

#[derive(Debug, Default)]
struct Grid(Vec<Vec<char>>);

impl Grid {
    fn xmas_occurrences_from(&self, x: usize, y: usize) -> usize {
        let word: Vec<char> = "XMAS".chars().collect();

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

    fn count_xmas_occurrences(&self) -> usize {
        let mut occurrences = 0;

        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                occurrences += self.xmas_occurrences_from(x, y);
            }
        }

        occurrences
    }

    fn mas_on_diagonal(&self, p1: (usize, usize), p2: (usize, usize)) -> bool {
        (self.0[p1.1][p1.0] == 'S' && self.0[p2.1][p2.0] == 'M')
            || (self.0[p1.1][p1.0] == 'M' && self.0[p2.1][p2.0] == 'S')
    }

    fn x_mas_occurrences_from(&self, x: usize, y: usize) -> bool {
        if self.0[y][x] != 'A' {
            trace!("({x}, {y}) is not 'A', skipping");
            return false;
        }

        let mut upper_left = None;
        let mut upper_right = None;
        let mut lower_left = None;
        let mut lower_right = None;
        for neighbor in util::grid_neighbors(&self.0, x, y, true) {
            match neighbor {
                Neighbor::UpperRight(_, _) => upper_right = Some(neighbor),
                Neighbor::UpperLeft(_, _) => upper_left = Some(neighbor),
                Neighbor::LowerRight(_, _) => lower_right = Some(neighbor),
                Neighbor::LowerLeft(_, _) => lower_left = Some(neighbor),
                _ => (),
            }
        }

        match (upper_left, lower_right) {
            (Some(Neighbor::UpperLeft(x1, y1)), Some(Neighbor::LowerRight(x2, y2))) => {
                if !self.mas_on_diagonal((x1, y1), (x2, y2)) {
                    return false;
                }
            }
            _ => return false,
        }

        match (upper_right, lower_left) {
            (Some(Neighbor::UpperRight(x1, y1)), Some(Neighbor::LowerLeft(x2, y2))) => {
                if !self.mas_on_diagonal((x1, y1), (x2, y2)) {
                    return false;
                }
            }
            _ => return false,
        }

        true
    }

    fn count_x_mas_occurrences(&self) -> usize {
        let mut occurrences = 0;

        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                occurrences += if self.x_mas_occurrences_from(x, y) {
                    1
                } else {
                    0
                };
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
            grid.0.push(line);
        }
        grid
    }
}

fn main() -> Result<()> {
    let input = util::init()?;
    let grid = Grid::from(input);

    let occurrences = grid.count_xmas_occurrences();
    println!("XMAS Occurrences: {occurrences}");

    let occurrences = grid.count_x_mas_occurrences();
    println!("X-MAS Occurrences: {occurrences}");

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

        assert_eq!(1, grid.xmas_occurrences_from(2, 0));
        assert_eq!(1, grid.xmas_occurrences_from(4, 1));
        assert_eq!(1, grid.xmas_occurrences_from(0, 3));
        assert_eq!(1, grid.xmas_occurrences_from(1, 4));

        Ok(())
    }

    #[test]
    fn part_1_example() -> Result<()> {
        let input = util::init_test()?;
        let grid = Grid::from(input);

        assert_eq!(1, grid.xmas_occurrences_from(4, 0));
        assert_eq!(1, grid.xmas_occurrences_from(5, 0));
        assert_eq!(1, grid.xmas_occurrences_from(4, 1));
        assert_eq!(2, grid.xmas_occurrences_from(9, 3));
        assert_eq!(1, grid.xmas_occurrences_from(0, 4));
        assert_eq!(2, grid.xmas_occurrences_from(6, 4));
        assert_eq!(1, grid.xmas_occurrences_from(0, 5));
        assert_eq!(1, grid.xmas_occurrences_from(6, 5));
        assert_eq!(1, grid.xmas_occurrences_from(1, 9));
        assert_eq!(2, grid.xmas_occurrences_from(3, 9));
        assert_eq!(3, grid.xmas_occurrences_from(5, 9));
        assert_eq!(2, grid.xmas_occurrences_from(9, 9));

        assert_eq!(18, grid.count_xmas_occurrences());

        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = util::init_test()?;
        let grid = Grid::from(input);

        assert!(grid.x_mas_occurrences_from(2, 1));

        assert_eq!(9, grid.count_x_mas_occurrences());

        Ok(())
    }
}
