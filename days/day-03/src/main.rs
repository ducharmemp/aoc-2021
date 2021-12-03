use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
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

fn part_one(lines: &Vec<String>) -> Result<u32> {
    let first_line = lines
        .first()
        .ok_or_else(|| anyhow::anyhow!("No first line"))?;
    let number_length = first_line.len();

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for index in 0..number_length {
        let mut map = HashMap::new();

        for line in lines {
            let bit = line
                .chars()
                .nth(index)
                .ok_or_else(|| anyhow::anyhow!("Could not get bit"))?;
            let count = map.entry(bit).or_insert_with(|| 0);
            *count += 1;
        }

        if map.get(&'0') > map.get(&'1') {
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

fn part_two(lines: &Vec<String>) -> Result<u32> {
    let first_line = lines
        .first()
        .ok_or_else(|| anyhow::anyhow!("No first line"))?;
    let number_length = first_line.len();

    let lines = lines
        .iter()
        .map(|val| u32::from_str_radix(val, 2).expect("Could not parse number"));
    let mut o2_numbers = HashSet::<u32>::from_iter(lines);
    let mut co2_numbers = o2_numbers.clone();

    for index in 0..number_length {
        if o2_numbers.len() == 1 {
            break;
        }
        let shift = number_length - index - 1;

        let nth_bits: Vec<u32> = o2_numbers
            .iter()
            .map(|line| (line & (1 << shift)) >> shift)
            .collect();
        let mut map = HashMap::new();
        for bit in nth_bits.iter() {
            let count = map.entry(bit).or_insert_with(|| 0);
            *count += 1;
        }
        if map.get(&0) > map.get(&1) {
            for number in o2_numbers.clone() {
                if (number & (1 << shift)) >> shift == 0 {
                    o2_numbers.remove(&number);
                }
            }
        } else {
            for number in o2_numbers.clone() {
                if (number & (1 << shift)) >> shift == 1 {
                    o2_numbers.remove(&number);
                }
            }
        }
    }

    for index in 0..number_length {
        if co2_numbers.len() == 1 {
            break;
        }
        let shift = number_length - index - 1;
        let nth_bits: Vec<u32> = co2_numbers
            .iter()
            .map(|line| (line & (1 << shift)) >> shift)
            .collect();
        let mut map = HashMap::new();
        for bit in nth_bits.iter() {
            let count = map.entry(bit).or_insert_with(|| 0);
            *count += 1;
        }
        dbg!(&map);
        if map.get(&0) <= map.get(&1) {
            dbg!(&map);
            for number in co2_numbers.clone() {
                if (number & (1 << shift)) >> shift == 0 {
                    co2_numbers.remove(&number);
                }
            }
        } else {
            for number in co2_numbers.clone() {
                if (number & (1 << shift)) >> shift == 1 {
                    co2_numbers.remove(&number);
                }
            }
        }
    }

    Ok(o2_numbers.iter().sum::<u32>() * co2_numbers.iter().sum::<u32>())
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
