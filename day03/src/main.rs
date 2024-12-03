use std::str::FromStr;

use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Multiplication(usize, usize),
}

impl Instruction {
    fn apply(&self) -> usize {
        match self {
            Instruction::Multiplication(x, y) => *x * *y,
        }
    }
}

#[derive(Debug, Default)]
struct Instructions(Vec<Instruction>);

impl TryFrom<String> for Instructions {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let multiplication_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

        let mut instructions = Instructions::default();

        for (_, [x, y]) in multiplication_regex
            .captures_iter(&value)
            .map(|c| c.extract())
        {
            instructions.0.push(Instruction::Multiplication(
                usize::from_str(x)?,
                usize::from_str(y)?,
            ))
        }

        Ok(instructions)
    }
}

fn main() -> Result<()> {
    let input = util::init()?.join("\n");

    let instructions = Instructions::try_from(input)?;

    let result: usize = instructions.0.iter().map(|i| i.apply()).sum();

    println!("Multiplication Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = util::init_test()?.join("\n");

        let instructions = Instructions::try_from(input)?;

        assert_eq!(4, instructions.0.len());

        assert_eq!(161usize, instructions.0.iter().map(|i| i.apply()).sum());

        Ok(())
    }

    #[test]
    #[ignore]
    fn part_2_example() -> Result<()> {
        todo!()
    }
}
