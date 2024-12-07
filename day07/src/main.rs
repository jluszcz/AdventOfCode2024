use std::str::FromStr;

use anyhow::{anyhow, Result};
use log::trace;

#[derive(Debug)]
struct Calibrations(Vec<CalibrationEquation>);

impl Calibrations {
    fn result(&self) -> usize {
        self.0.iter().filter(|c| c.is_valid()).map(|c| c.test).sum()
    }
}

impl TryFrom<Vec<String>> for Calibrations {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut equations = Vec::with_capacity(value.len());
        for line in value {
            equations.push(CalibrationEquation::from_str(&line)?);
        }

        Ok(Self(equations))
    }
}

#[derive(Debug)]
struct CalibrationEquation {
    test: usize,
    numbers: Vec<usize>,
}

impl CalibrationEquation {
    fn inner_is_valid(test: usize, numbers: &[usize]) -> bool {
        if numbers.is_empty() {
            return false;
        }

        if test == numbers[0] {
            return true;
        }

        let curr = *numbers.last().unwrap();
        let remaining = &numbers[..numbers.len() - 1];

        trace!("{curr} {remaining:?}");

        let addition = if let Some(test) = test.checked_sub(curr) {
            Self::inner_is_valid(test, remaining)
        } else {
            false
        };

        let multiplication = if let Some(test) = (test % curr == 0).then(|| test / curr) {
            Self::inner_is_valid(test, remaining)
        } else {
            false
        };

        addition || multiplication
    }

    fn is_valid(&self) -> bool {
        Self::inner_is_valid(self.test, &self.numbers)
    }
}

impl FromStr for CalibrationEquation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");
        let test = usize::from_str(
            parts
                .next()
                .ok_or_else(|| anyhow!("No test value in {s}"))?,
        )?;

        let mut numbers = Vec::new();
        for n in parts
            .next()
            .ok_or_else(|| anyhow!("No numbers in {s}"))?
            .split_ascii_whitespace()
        {
            numbers.push(usize::from_str(n)?);
        }

        if parts.next().is_none() {
            Ok(Self { test, numbers })
        } else {
            Err(anyhow!("Extra parts in {s}"))
        }
    }
}

fn main() -> Result<()> {
    let input = util::init()?;

    let equations = Calibrations::try_from(input)?;

    let result = equations.result();
    println!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = util::init_test()?;
        let equations = Calibrations::try_from(input)?;

        for equation in equations.0.iter() {
            assert_eq!(
                equation.test == 190 || equation.test == 3267 || equation.test == 292,
                equation.is_valid(),
                "failed on {equation:?}"
            );
        }

        assert_eq!(3749, equations.result());

        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2_example() -> Result<()> {
        let input = util::init_test()?;
        let equations = Calibrations::try_from(input)?;

        let equation = CalibrationEquation {
            test: 7290,
            numbers: vec![6, 8, 6, 15],
        };
        assert!(equation.is_valid());

        for equation in equations.0.iter() {
            assert_eq!(
                equation.test == 190
                    || equation.test == 3267
                    || equation.test == 292
                    || equation.test == 156
                    || equation.test == 7290
                    || equation.test == 192,
                equation.is_valid(),
                "failed on {equation:?}"
            );
        }

        assert_eq!(11387, equations.result());

        Ok(())
    }
}
