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

fn parse_first_line(lines: &[String]) -> Result<Vec<i64>> {
    let line = lines
        .first()
        .ok_or_else(|| anyhow::anyhow!("Could not get input line"))?;
    let inputs: Result<Vec<i64>> = line
        .split(',')
        .map(|val| val.parse::<i64>().context("Could not parse i64"))
        .collect();
    inputs
}

fn part_one(lines: &[String]) -> Result<i64> {
    let inputs = parse_first_line(lines)?;

    let min_horizontal = *inputs
        .iter()
        .min()
        .ok_or_else(|| anyhow::anyhow!("Couldn't determine min"))?;
    let max_horizontal = *inputs
        .iter()
        .max()
        .ok_or_else(|| anyhow::anyhow!("Couldn't determine max"))?;
    let final_costs = (min_horizontal..=max_horizontal).map(|final_position| {
        inputs
            .iter()
            .map(move |val| (val - final_position).abs())
            .sum()
    });

    final_costs
        .min()
        .ok_or_else(|| anyhow::anyhow!("Could not determine final min cost"))
}

fn part_two(lines: &[String]) -> Result<i64> {
    let inputs = parse_first_line(lines)?;

    let min_horizontal = *inputs
        .iter()
        .min()
        .ok_or_else(|| anyhow::anyhow!("Couldn't determine min"))?;
    let max_horizontal = *inputs
        .iter()
        .max()
        .ok_or_else(|| anyhow::anyhow!("Couldn't determine max"))?;

    let final_costs = (min_horizontal..=max_horizontal).map(|final_position| {
        inputs
            .iter()
            .map(move |val| {
                let abs_fuel_cost = (val - final_position).abs();
                (abs_fuel_cost * (abs_fuel_cost + 1)) / 2
            })
            .sum()
    });

    final_costs
        .min()
        .ok_or_else(|| anyhow::anyhow!("Could not determine final min cost"))
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
