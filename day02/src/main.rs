use anyhow::Result;
use std::str::FromStr;

#[derive(Debug, Default)]
struct Levels(Vec<usize>);

impl Levels {
    fn is_safe(&self) -> bool {
        let mut overall_increasing = None;

        for (i, val) in self.0.iter().take(self.0.len() - 1).enumerate() {
            let x = *val;
            let y = self.0[i + 1];

            let increasing = y > x;

            match overall_increasing {
                Some(true) if increasing => (),
                Some(false) if !increasing => (),
                None => overall_increasing = Some(increasing),
                Some(_) => return false,
            }

            let diff = if increasing { y - x } else { x - y };

            if !(1..=3).contains(&diff) {
                return false;
            }
        }

        true
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

    let safety_count = input
        .into_iter()
        .map(|s| Levels::try_from(s).unwrap())
        .filter(Levels::is_safe)
        .count();
    println!("Safety Count: {safety_count}");

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
            assert_eq!(i == 0 || i == 5, is_safe, "Incorrect Safety: {:?} --> {}", levels, is_safe);
        }

        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2_example() -> Result<()> {
        todo!();

        Ok(())
    }
}
