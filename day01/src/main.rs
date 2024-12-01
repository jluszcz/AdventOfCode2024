use std::collections::HashMap;
use std::str::FromStr;
use anyhow::{anyhow, Result};
use log::info;

#[derive(Debug, Default)]
struct Locations {
    lhs: Vec<usize>,
    rhs: Vec<usize>,
}

impl Locations {
    fn total_distance(&self) -> usize {
        let mut lhs = self.lhs.clone();
        lhs.sort();

        let mut rhs = self.rhs.clone();
        rhs.sort();

        let mut distance = 0;
        for (i, j) in lhs.into_iter().zip(rhs.into_iter()) {
            if i > j {
                distance += i - j;
            } else {
                distance += j - i;
            }
        }

        distance
    }

    fn occurrences(items: &[usize]) -> HashMap<usize, usize> {
        let mut occurrences = HashMap::new();

        for item in items {
            occurrences.entry(*item).and_modify(|c| *c += 1).or_insert(1);
        }

        occurrences
    }

    fn similarity_score(&self) -> usize {
        let occurrences = Self::occurrences(&self.rhs);

        let mut similarity = 0;
        for item in self.lhs.iter() {
            similarity += occurrences.get(item).cloned().unwrap_or(0) * *item;
        }

        similarity
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

    let similarity_score = locations.similarity_score();
    info!("Similarity Score: {similarity_score}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = util::init_test()?;

        let locations = Locations::try_from(input)?;
        assert_eq!(11, locations.total_distance());

        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = util::init_test()?;

        let locations = Locations::try_from(input)?;
        assert_eq!(31, locations.similarity_score());

        Ok(())
    }
}
