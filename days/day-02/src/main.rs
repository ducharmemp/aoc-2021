use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const CURRENT_FILE: &str = file!();
const INPUT_FILE_PATH: &str = "../data/input.txt";

#[derive(Default, Clone, PartialEq, Debug)]
struct SubPosition {
    pub depth_position: i32,
    pub horizontal_position: i32,
    pub aim: i32,
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

fn part_one(lines: &Vec<String>) -> Result<i32> {
    let mut position = SubPosition::default();

    for line in lines.iter() {
        let (instruction, value) = line.split_once(' ').expect("Invalid instruction");
        let value = value.parse::<i32>()?;

        match instruction {
            "forward" => position.horizontal_position += value,
            "down" => position.depth_position += value,
            "up" => position.depth_position -= value,
            _ => unreachable!(),
        };
    }
    Ok(position.depth_position * position.horizontal_position)
}

fn part_two(lines: &Vec<String>) -> Result<i32> {
    let mut position = SubPosition::default();

    for line in lines.iter() {
        let (instruction, value) = line.split_once(' ').expect("Invalid instruction");
        let value = value.parse::<i32>()?;

        match instruction {
            "forward" => {
                position.horizontal_position += value;
                position.depth_position += position.aim * value;
            }
            "down" => position.aim += value,
            "up" => position.aim -= value,
            _ => unreachable!(),
        };
    }
    Ok(position.depth_position * position.horizontal_position)
}

fn main() -> Result<()> {
    let input_path = Path::new(CURRENT_FILE)
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Couldn't get parent directory"))?
        .join(INPUT_FILE_PATH);

    let input = read_lines(&input_path)?;
    println!("{}", part_one(&input)?);
    println!("{}", part_two(&input)?);

    Ok(())
}
