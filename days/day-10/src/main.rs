use anyhow::{Context, Result};
use std::collections::{HashMap, VecDeque};
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

fn parse_line(line: &str) -> Vec<String> {
    line.chars().map(|val| val.to_string()).collect()
}

fn validate_line(line: &[String]) -> Result<Vec<String>, String> {
    let mut stack = VecDeque::new();
    let expected_closings: HashMap<_, _> = [("{", "}"), ("(", ")"), ("[", "]"), ("<", ">")]
        .into_iter()
        .collect();

    for character in line.iter() {
        match character.as_str() {
            "{" | "(" | "[" | "<" => stack.push_front(character),
            "}" | ")" | "]" | ">" => {
                let opening = stack.pop_front().ok_or_else(|| character.clone())?;
                if expected_closings.get(opening.as_str()) != Some(&character.as_str()) {
                    return Err(character.clone());
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(line.to_vec())
}

fn complete_line(line: &[String]) -> Vec<String> {
    let mut stack = VecDeque::new();
    let expected_closings: HashMap<_, _> = [("{", "}"), ("(", ")"), ("[", "]"), ("<", ">")]
        .into_iter()
        .collect();

    for character in line.iter() {
        match character.as_str() {
            "{" | "(" | "[" | "<" => stack.push_front(character),
            "}" | ")" | "]" | ">" => {
                stack.pop_front();
            }
            _ => unreachable!(),
        }
    }

    stack
        .iter()
        .map(|opening| expected_closings[opening.as_str()].to_string())
        .collect()
}

fn part_one(lines: &[String]) -> Result<u32> {
    let errors = lines
        .iter()
        .map(|line| parse_line(line))
        .map(|line| validate_line(&line))
        .filter(|res| res.is_err())
        .map(|res| res.unwrap_err());

    let total_points = errors.fold(0, |acc, error| {
        acc + match error.as_str() {
            ")" => 3,
            "]" => 57,
            "}" => 1197,
            ">" => 25137,
            _ => unreachable!(),
        }
    });
    Ok(total_points)
}

fn part_two(lines: &[String]) -> Result<i64> {
    let incomplete_lines = lines
        .iter()
        .map(|line| parse_line(line))
        .map(|line| validate_line(&line))
        .filter(|res| res.is_ok());

    let complete_lines = incomplete_lines.map(|line| complete_line(&line.expect("Expected line")));
    let total_scores = complete_lines.map(|line| {
        line.iter().fold(0_i64, |acc, expected_closing| {
            let acc = acc * 5;
            acc + match expected_closing.as_str() {
                ")" => 1,
                "]" => 2,
                "}" => 3,
                ">" => 4,
                _ => unreachable!(),
            }
        })
    });
    let mut total_scores: Vec<_> = total_scores.collect();
    total_scores.sort_unstable();
    Ok(total_scores[total_scores.len() / 2])
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
