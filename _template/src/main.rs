use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const CURRENT_FILE: &str = file!();
const INPUT_FILE_PATH: &str = "../data/input.txt";

fn read_lines<P>(filename: &P) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    io::BufReader::new(file)
        .lines()
        .map(|val| val.context("Could not read line"))
        .collect()
}

fn parse_line(line: &str) -> Result<i64> {

}

fn part_one(lines: &[String]) -> Result<()> {
    let lines = lines.iter().map(|line| parse_line(line));
    Ok(())
}

fn part_two(lines: &[String]) -> Result<()> {
    let lines = lines.iter().map(|line| parse_line(line));
    Ok(())
}

fn main() -> Result<()> {
    let input_path = Path::new(CURRENT_FILE)
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Couldn't get parent directory"))?
        .join(INPUT_FILE_PATH);

    let input = read_lines(&input_path)?;
    println!("{:?}", part_one(&input)?);
    println!("{:?}", part_two(&input)?);

    Ok(())
}
