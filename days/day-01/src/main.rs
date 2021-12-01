use anyhow::Result;
use itertools::Itertools;

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
    Ok(io::BufReader::new(file)
        .lines()
        .map(|val| val.expect("Could not read line"))
        .collect())
}

fn part_one(lines: Vec<String>) -> Result<i32> {
    let lines: Vec<i32> = lines
        .iter()
        .map(|val| str::parse::<i32>(val).expect("Could not parse i32"))
        .collect();

    Ok(lines
        .iter()
        .tuple_windows()
        .filter(|(prev, current)| prev < current)
        .count() as i32)
}

fn part_two(lines: Vec<String>) -> Result<i32> {
    let lines: Vec<i32> = lines
        .iter()
        .map(|val| str::parse::<i32>(val).expect("Could not parse i32"))
        .collect();

    return Ok(lines
        .iter()
        .tuple_windows::<(_, _, _)>()
        .tuple_windows()
        .filter(|(prev, current)| prev.0 + prev.1 + prev.2 < current.0 + current.1 + current.2)
        .count() as i32);
}

fn main() -> Result<()> {
    let input_path = Path::new(CURRENT_FILE)
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Couldn't get parent directory"))?
        .join(INPUT_FILE_PATH);

    println!("{}", part_one(read_lines(&input_path)?)?);
    println!("{}", part_two(read_lines(&input_path)?)?);

    Ok(())
}
