use std::fmt::Debug;

use anyhow::{anyhow, Result};
use log::{log_enabled, trace};
use log::Level::Trace;

use util::Direction;

#[derive(Copy, Clone, Debug)]
struct GuardState {
    direction: Direction,
    position: (usize, usize),
    has_left: bool,
}

impl GuardState {
    fn new(x: usize, y: usize) -> Self {
        Self {
            direction: Direction::Up,
            position: (x, y),
            has_left: false,
        }
    }

    fn rotate(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct LabState {
    obstacles: Vec<Vec<bool>>,
    visited: Vec<Vec<bool>>,
    guard: GuardState,
}

impl LabState {
    fn visited_positions(&self) -> usize {
        self.visited
            .iter()
            .map(|r| r.iter().map(|v| if *v { 1 } else { 0 }).sum::<usize>())
            .sum()
    }

    fn advance_until_guard_leaves(self) -> Result<Self> {
        let mut state = self;
        if log_enabled!(Trace) {
            trace!("\n{}", state.debug_current_state(true));
        }

        while !state.guard.has_left {
            state = state.advance()?;
            if log_enabled!(Trace) {
                trace!("\n{}", state.debug_current_state(true));
            }
        }
        Ok(state)
    }

    fn advance(self) -> Result<Self> {
        if self.guard.has_left {
            return Err(anyhow!("Guard has left"));
        }

        let obstacles = self.obstacles;
        let mut visited = self.visited;
        let mut guard = self.guard;

        loop {
            visited[guard.position.1][guard.position.0] = true;

            if let Some(neighbor) = util::neighbor_in_direction(
                &obstacles,
                guard.direction,
                guard.position.0,
                guard.position.1,
            ) {
                let (x, y) = neighbor.into();

                if obstacles[y][x] {
                    guard.rotate();
                    break;
                }

                guard.position = (x, y);
            } else {
                guard.has_left = true;
                break;
            }
        }

        Ok(Self {
            obstacles,
            visited,
            guard,
        })
    }

    fn debug_current_state(&self, show_path: bool) -> String {
        let mut lines = Vec::with_capacity(self.obstacles.len());

        for y in 0..self.obstacles.len() {
            let mut line = String::new();
            for x in 0..self.obstacles[y].len() {
                if self.obstacles[y][x] {
                    line.push('#');
                } else if self.guard.position == (x, y) {
                    line.push(char::from(self.guard.direction));
                } else if show_path && self.visited[y][x] {
                    line.push('X');
                } else {
                    line.push('.');
                }
            }
            lines.push(line);
        }

        lines.join("\n")
    }
}

impl TryFrom<Vec<String>> for LabState {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut obstacles = Vec::new();
        let mut visited = Vec::new();

        let mut guard = None;

        for (y, line) in value.into_iter().enumerate() {
            let mut obstacles_row = vec![false; line.len()];
            let mut visited_row = vec![false; line.len()];

            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => (),
                    '#' => obstacles_row[x] = true,
                    '^' => {
                        visited_row[x] = true;
                        guard = Some(GuardState::new(x, y));
                    }
                    _ => return Err(anyhow!("Invalid character in grid: {c}")),
                }
            }

            obstacles.push(obstacles_row);
            visited.push(visited_row);
        }

        Ok(LabState {
            obstacles,
            visited,
            guard: guard.ok_or_else(|| anyhow!("Guard not found"))?,
        })
    }
}

fn main() -> Result<()> {
    let input = util::init()?;

    let lab_state = LabState::try_from(input)?.advance_until_guard_leaves()?;

    let visited_positions = lab_state.visited_positions();
    println!("Visited Positions: {visited_positions}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = util::init_test()?;

        let lab_state = LabState::try_from(input)?.advance_until_guard_leaves()?;

        assert_eq!(41, lab_state.visited_positions());

        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2_example() -> Result<()> {
        let _input = util::init_test()?;

        Ok(())
    }
}
