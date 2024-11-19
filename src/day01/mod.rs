use crate::utils::file_reader;
use crate::utils::solution::Solution;
use anyhow::anyhow;
use anyhow::Result;

const DAY_STR: &str = "1";

pub fn run(part: u8) -> Result<()> {
    let input = file_reader::read_text_from_file(DAY_STR)?;
    let answer = match part {
        1 => part1(&input),
        2 => part2(&input),
        val => Err(anyhow!("got a part thats not 1 or 2: {}", val)),
    }?;

    println!("The answer for part {part} on {DAY_STR} is probably: {answer}");

    Ok(())
}

fn part1(input: &str) -> Result<Solution> {
    todo!()
}

fn part2(input: &str) -> Result<Solution> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::utils::solution::Solution;

    use super::*;

    const TEST_INPUT_ONE: &str = r"";
    const TEST_INPUT_TWO: &str = r"";

    #[test]
    fn test_part_1() -> Result<()> {
        let fasit = Solution::from(142u32);
        let part_solution = part1(TEST_INPUT_ONE)?;
        assert_eq!(fasit, part_solution);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let fasit = Solution::from(281u32);
        let my_soultion = part2(TEST_INPUT_TWO)?;
        assert_eq!(fasit, my_soultion);
        Ok(())
    }
}
