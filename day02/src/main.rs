use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, Default)]
struct Levels(Vec<usize>);

impl Levels {
    fn are_readings_safe_and_increasing(
        x: usize,
        y: usize,
        overall_increasing: &Option<bool>,
    ) -> (bool, bool) {
        let increasing = y > x;

        match overall_increasing {
            Some(overall_increasing) if increasing != *overall_increasing => {
                return (false, increasing)
            }
            Some(_) | None => (),
        }

        let diff = if increasing { y - x } else { x - y };

        ((1..=3).contains(&diff), increasing)
    }

    fn are_readings_safe(readings: &[usize]) -> bool {
        let mut overall_increasing = None;

        for (i, val) in readings.iter().take(readings.len() - 1).enumerate() {
            let x = *val;
            let y = readings[i + 1];

            let (safe, increasing) =
                Self::are_readings_safe_and_increasing(x, y, &overall_increasing);

            if !safe {
                return false;
            }

            overall_increasing = Some(increasing);
        }

        true
    }

    fn is_safe(&self) -> bool {
        Self::are_readings_safe(&self.0)
    }

    fn is_safe_with_problem_dampener(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for i in 0..self.0.len() {
            let mut readings = self.0.clone();
            readings.remove(i);

            if Self::are_readings_safe(&readings) {
                return true;
            }
        }

        false
    }
}

impl TryFrom<String> for Levels {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut levels = Levels::default();

        for val in value.split_ascii_whitespace() {
            levels.0.push(usize::from_str(val)?);
        }

        Ok(levels)
    }
}

fn main() -> Result<()> {
    let input = util::init()?;

    let levels = input
        .into_iter()
        .map(|s| Levels::try_from(s).unwrap())
        .collect::<Vec<_>>();

    let safety_count = levels.iter().filter(|l| (*l).is_safe()).count();

    println!("Safety Count: {safety_count}");

    let safety_count = levels
        .iter()
        .filter(|l| (*l).is_safe_with_problem_dampener())
        .count();

    println!("Safety Count w/ Problem Dampener: {safety_count}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = util::init_test()?;
        assert_eq!(6, input.len());

        for (i, line) in input.into_iter().enumerate() {
            let levels = Levels::try_from(line)?;
            let is_safe = levels.is_safe();
            assert_eq!(
                i == 0 || i == 5,
                is_safe,
                "Incorrect Safety: {:?} --> {}",
                levels,
                is_safe
            );
        }

        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let mut input = util::init_test()?;

        // Add an example where we need to skip the first reading
        input.push("10 2 3 4 5".to_string());

        assert_eq!(7, input.len());

        for (i, line) in input.into_iter().enumerate() {
            let levels = Levels::try_from(line)?;
            let is_safe = levels.is_safe_with_problem_dampener();
            assert_eq!(
                !(i == 1 || i == 2),
                is_safe,
                "Incorrect Safety: {:?} --> {}",
                levels,
                is_safe
            );
        }

        Ok(())
    }
}
