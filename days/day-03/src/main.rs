use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use std::path::Path;

const CURRENT_FILE: &str = file!();
const INPUT_FILE_PATH: &str = "../data/input.txt";

enum FilterOrder {
    MostSignificant,
    LeastSignificant,
}

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

fn bit_at(number: &u32, pos: usize) -> u32 {
    (number & (1 << pos)) >> pos
}

fn part_one(numbers: &[u32], num_bits: usize) -> Result<u32> {
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for shift in (0..num_bits).rev() {
        let count: HashMap<u32, usize> = numbers.iter().map(|line| bit_at(line, shift)).counts();

        if count.get(&0) > count.get(&1) {
            gamma <<= 1;
            epsilon <<= 1;
            epsilon += 1
        } else {
            epsilon <<= 1;
            gamma <<= 1;
            gamma += 1;
        }
    }

    Ok(gamma * epsilon)
}

fn filter_by(mut numbers: Vec<u32>, num_bits: usize, order: FilterOrder) -> Vec<u32> {
    for shift in (0..num_bits).rev() {
        if numbers.len() == 1 {
            break;
        }

        let count: HashMap<u32, usize> = numbers.iter().map(|line| bit_at(line, shift)).counts();

        let filter_val = match (&order, count.get(&0) > count.get(&1)) {
            (FilterOrder::MostSignificant, true) => 0,
            (FilterOrder::MostSignificant, false) => 1,
            (FilterOrder::LeastSignificant, false) => 0,
            (FilterOrder::LeastSignificant, true) => 1,
        };

        numbers = numbers
            .iter()
            .filter(|val| bit_at(*val, shift) == filter_val)
            .cloned()
            .collect();
    }

    numbers
}

fn part_two(lines: &[u32], num_bits: usize) -> Result<u32> {
    let o2_numbers = filter_by(lines.to_vec(), num_bits, FilterOrder::MostSignificant);
    let co2_numbers = filter_by(lines.to_vec(), num_bits, FilterOrder::LeastSignificant);

    Ok(o2_numbers.iter().sum::<u32>() * co2_numbers.iter().sum::<u32>())
}

fn main() -> Result<()> {
    let input_path = Path::new(CURRENT_FILE)
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Couldn't get parent directory"))?
        .join(INPUT_FILE_PATH);

    let input = read_lines(&input_path)?;
    let first_line = input
        .first()
        .ok_or_else(|| anyhow::anyhow!("No first line"))?;
    let num_bits = first_line.len();

    let input: Result<Vec<u32>> = input
        .iter()
        .map(|val| u32::from_str_radix(val, 2).context("Could not parse number"))
        .collect();
    let input = input?;

    println!("{:?}", part_one(&input, num_bits)?);
    println!("{:?}", part_two(&input, num_bits)?);

    Ok(())
}
