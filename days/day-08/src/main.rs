use anyhow::{Context, Result};
use std::collections::HashSet;
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

fn parse_line(line: &str) -> Result<(Vec<&str>, Vec<&str>)> {
    let (patterns, output) = line
        .split_once("|")
        .ok_or_else(|| anyhow::anyhow!("Could not split patterns and output"))?;

    Ok((
        patterns.split_whitespace().collect(),
        output.split_whitespace().collect(),
    ))
}

fn part_one(lines: &[String]) -> Result<usize> {
    let lines: Result<Vec<_>> = lines.iter().map(|line| parse_line(line)).collect();
    let lines = lines?;
    let lines = lines.iter().flat_map(|(_, output)| output);

    let pattern_lengths: HashSet<&usize> = HashSet::from_iter([2, 3, 4, 7].iter());

    // dbg!(&lines.map(|val| val.len()).collect::<Vec<usize>>());
    Ok(lines
        .filter(|val| pattern_lengths.contains(&val.len()))
        .count())
}

fn part_two(lines: &[String]) -> Result<i32> {
    let mut final_output_value = 0;

    for line in lines {
        let (signal, outputs) = parse_line(line)?;

        let one_pattern = signal
            .iter()
            .find(|val| val.len() == 2)
            .ok_or_else(|| anyhow::anyhow!("Could not find pattern for one"))?;
        let four_pattern = signal
            .iter()
            .find(|val| val.len() == 4)
            .ok_or_else(|| anyhow::anyhow!("Could not find pattern for four"))?;
        let seven_pattern = signal
            .iter()
            .find(|val| val.len() == 3)
            .ok_or_else(|| anyhow::anyhow!("Could not find pattern for seven"))?;
        let eight_pattern = signal
            .iter()
            .find(|val| val.len() == 7)
            .ok_or_else(|| anyhow::anyhow!("Could not find pattern for eight"))?;

        let one_pattern: HashSet<char> = HashSet::from_iter(one_pattern.chars());
        let four_pattern: HashSet<char> = HashSet::from_iter(four_pattern.chars());
        let seven_pattern: HashSet<char> = HashSet::from_iter(seven_pattern.chars());
        let eight_pattern: HashSet<char> = HashSet::from_iter(eight_pattern.chars());

        let six_segment_candidates = signal.iter().filter(|val| val.len() == 6);
        let five_segment_candidates = signal.iter().filter(|val| val.len() == 5);

        let nine_pattern = HashSet::from_iter(
            six_segment_candidates
                .clone()
                .find(|val| HashSet::from_iter(val.chars()).is_superset(&four_pattern))
                .unwrap()
                .chars(),
        );
        let zero_pattern = HashSet::from_iter(
            six_segment_candidates
                .clone()
                .find(|val| {
                    HashSet::from_iter(val.chars()).is_superset(&one_pattern)
                        && HashSet::from_iter(val.chars()) != nine_pattern
                })
                .unwrap()
                .chars(),
        );
        let six_pattern = HashSet::from_iter(
            six_segment_candidates
                .clone()
                .find(|val| {
                    HashSet::from_iter(val.chars()) != zero_pattern
                        && HashSet::from_iter(val.chars()) != nine_pattern
                })
                .unwrap()
                .chars(),
        );

        let three_pattern = HashSet::from_iter(
            five_segment_candidates
                .clone()
                .find(|val| HashSet::from_iter(val.chars()).is_superset(&seven_pattern))
                .unwrap()
                .chars(),
        );

        let five_pattern = HashSet::from_iter(
            five_segment_candidates
                .clone()
                .find(|val| HashSet::from_iter(val.chars()).is_subset(&six_pattern))
                .unwrap()
                .chars(),
        );

        let two_pattern = HashSet::from_iter(
            five_segment_candidates
                .clone()
                .find(|val| {
                    HashSet::from_iter(val.chars()) != three_pattern
                        && HashSet::from_iter(val.chars()) != five_pattern
                })
                .unwrap()
                .chars(),
        );

        let patterns = [
            (one_pattern, 1),
            (seven_pattern, 7),
            (four_pattern, 4),
            (two_pattern, 2),
            (five_pattern, 5),
            (three_pattern, 3),
            (six_pattern, 6),
            (nine_pattern, 9),
            (zero_pattern, 0),
            (eight_pattern, 8),
        ];

        let mut final_number = 0;
        for (power, num) in outputs.iter().enumerate() {
            let (_, value) = patterns
                .iter()
                .find(|(mapping, _)| HashSet::from_iter(num.chars()).eq(mapping))
                .ok_or_else(|| anyhow::anyhow!("Couldn't find the number captain"))?;
            final_number += value * (10_i32.pow(3 - power as u32));
        }
        final_output_value += final_number;
    }

    Ok(final_output_value)
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
