use std::str::FromStr;

use anyhow::{anyhow, Result};
use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Multiplication(usize, usize),
    Do,
    DoNot,
}

#[derive(Debug, Default)]
struct Instructions(Vec<Instruction>);

impl Instructions {
    fn apply(&self, unconditional: bool) -> usize {
        let mut enabled = true;

        let mut result = 0;
        for instruction in self.0.iter() {
            match instruction {
                Instruction::Do => enabled = true,
                Instruction::DoNot => enabled = false,
                Instruction::Multiplication(x, y) => {
                    if unconditional || enabled {
                        result += *x * *y
                    }
                }
            }
        }

        result
    }

    fn apply_unconditionally(&self) -> usize {
        self.apply(true)
    }

    fn apply_conditionally(&self) -> usize {
        self.apply(false)
    }
}

impl TryFrom<String> for Instructions {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let instruction_regex =
            Regex::new(r"(mul\((?<x>\d{1,3}),(?<y>\d{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))")?;

        let mut instructions = Instructions::default();

        for m in instruction_regex.captures_iter(&value) {
            if m.name("do").is_some() {
                instructions.0.push(Instruction::Do);
            } else if m.name("dont").is_some() {
                instructions.0.push(Instruction::DoNot);
            } else {
                let x = usize::from_str(
                    m.name("x")
                        .ok_or_else(|| anyhow!("Missing x in multiplication"))?
                        .as_str(),
                )?;

                let y = usize::from_str(
                    m.name("y")
                        .ok_or_else(|| anyhow!("Missing y in multiplication"))?
                        .as_str(),
                )?;

                instructions.0.push(Instruction::Multiplication(x, y));
            }
        }

        Ok(instructions)
    }
}

fn main() -> Result<()> {
    let input = util::init()?.join("\n");

    let instructions = Instructions::try_from(input)?;

    let result = instructions.apply_unconditionally();
    println!("Unconditional Result: {result}");

    let result = instructions.apply_conditionally();
    println!("Conditional Result: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = util::init_test()?.join("\n");

        let instructions = Instructions::try_from(input)?;

        assert_eq!(161, instructions.apply_unconditionally());

        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        util::init_test_logger()?;

        let input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();

        let instructions = Instructions::try_from(input)?;

        assert_eq!(48, instructions.apply_conditionally());

        Ok(())
    }
}
