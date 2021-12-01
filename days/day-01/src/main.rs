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
    let mut ctr = 0;
    let lines: Vec<i32> = lines
        .iter()
        .map(|val| str::parse::<i32>(val).expect("Could not parse i32"))
        .collect();

    let mut lines_iter = lines.iter();
    let mut prev = lines_iter.next().expect("No first entry");

    for current in lines_iter {
        if prev < current {
            ctr += 1;
        }
        prev = current;
    }
    Ok(ctr)
}

fn part_two(lines: Vec<String>) -> Result<i32> {
    let mut ctr = 0;

    let lines: Vec<i32> = lines
        .iter()
        .map(|val| str::parse::<i32>(val).expect("Could not parse i32"))
        .collect();

    let mut windows = lines.iter().tuple_windows::<(_, _, _)>();

    let prev_window = windows
        .next()
        .ok_or_else(|| anyhow::anyhow!("No first entry"))?;

    let mut prev_sum = prev_window.0 + prev_window.1 + prev_window.2;

    for current_window in windows {
        let current_sum = current_window.0 + current_window.1 + current_window.2;

        if prev_sum < current_sum {
            ctr += 1;
        }
        prev_sum = current_sum;
    }
    Ok(ctr)
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
