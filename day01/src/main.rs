use std::str::FromStr;
use anyhow::{anyhow, Result};
use log::info;

#[derive(Debug, Default)]
struct Locations {
    lhs: Vec<usize>,
    rhs: Vec<usize>,
}

impl Locations {
    fn total_distance(mut self) -> usize {
        self.lhs.sort();
        self.rhs.sort();

        let mut distance = 0;
        for (i, j) in self.lhs.into_iter().zip(self.rhs.into_iter()) {
            if i > j {
                distance += i - j;
            } else {
                distance += j - i;
            }
        }

        distance
    }
}

impl TryFrom<Vec<String>> for Locations {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut locations = Locations::default();

        for line in value {
            for (i, val) in line.split_ascii_whitespace().enumerate() {
                match i {
                    0 => locations.lhs.push(usize::from_str(val)?),
                    1 => locations.rhs.push(usize::from_str(val)?),
                    _ => return Err(anyhow!("Invalid index: {i}")),
                }
            }
        }

        Ok(locations)
    }
}

fn main() -> Result<()> {
    let input = util::init()?;
    let locations = Locations::try_from(input)?;

    let total_distance = locations.total_distance();
    info!("Total Distance: {total_distance}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() -> Result<()> {
        util::init_test_logger()?;

        let locations = Locations::try_from(util::test_input()?)?;
        assert_eq!(11, locations.total_distance());

        Ok(())
    }
}
