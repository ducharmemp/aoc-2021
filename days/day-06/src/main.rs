use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const CURRENT_FILE: &str = file!();
const INPUT_FILE_PATH: &str = "../data/input.txt";

const RESET_VALUE: u64 = 6;
const SPAWN_VALUE: u64 = 8;

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

fn hash_simulate(inputs: &[u64], max_steps: u64) -> Result<u64> {
    let final_map = (0..max_steps).fold(inputs.iter().cloned().counts(), |acc, _| {
        let mut new_hash: HashMap<u64, usize> = HashMap::new();

        for (key, value) in acc {
            if key == 0 {
                *new_hash.entry(RESET_VALUE).or_insert(0) += value;
                *new_hash.entry(SPAWN_VALUE).or_insert(0) += value;
            } else {
                let next_key = key - 1;
                *new_hash.entry(next_key).or_insert(0) += value;
            };
        }

        new_hash
    });

    Ok(final_map.values().sum::<usize>() as u64)
}

fn part_one(lines: &[String]) -> Result<u64> {
    let line = lines
        .first()
        .ok_or_else(|| anyhow::anyhow!("Could not get input line"))?;
    let inputs: Result<Vec<u64>> = line
        .split(',')
        .map(|val| val.parse::<u64>().context("Could not parse u64"))
        .collect();
    let inputs = inputs?;

    hash_simulate(&inputs, 80)
}

fn part_two(lines: &[String]) -> Result<u64> {
    let line = lines
        .first()
        .ok_or_else(|| anyhow::anyhow!("Could not get input line"))?;
    let inputs: Result<Vec<u64>> = line
        .split(',')
        .map(|val| val.parse::<u64>().context("Could not parse u64"))
        .collect();
    let inputs = inputs?;
    hash_simulate(&inputs, 256)
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
