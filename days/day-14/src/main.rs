use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;

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

fn partition_instructions<'a>(lines: &'a [String]) -> (String, Vec<&'a str>) {
    let lines = lines
        .iter()
        .batching(|it| {
            let mut chunk = vec![];
            for line in &mut *it {
                if line.is_empty() {
                    return Some(chunk);
                }

                chunk.push(line.as_ref())
            }

            if !chunk.is_empty() {
                Some(chunk)
            } else {
                None
            }
        })
        .collect::<Vec<Vec<&'a str>>>();
    (
        lines
            .get(0)
            .expect("Could not get template")
            .first()
            .cloned()
            .expect("Could not get template")
            .to_string(),
        lines
            .get(1)
            .cloned()
            .expect("Could not get insertion instructions"),
    )
}

fn create_pair_mapping<'a>(lines: &'a [&str]) -> HashMap<(&'a str, &'a str), &'a str> {
    HashMap::from_iter(
        lines
            .iter()
            .map(|line| line.split_once("->").expect("Could not parse line"))
            .map(|(key, value)| (key.trim(), value.trim()))
            .map(|(key, value)| {
                let characters: Vec<_> = key.graphemes(false).collect();
                let (first, second) = (
                    characters
                        .get(0)
                        .cloned()
                        .expect("Could not get first instruction element"),
                    characters
                        .get(1)
                        .cloned()
                        .expect("Could not get second instruction element"),
                );
                ((first, second), value)
            }),
    )
}

fn part_one(lines: &[String]) -> Result<usize> {
    let (template, pair_insertions) = partition_instructions(lines);
    let pair_insertions = create_pair_mapping(&pair_insertions);

    let final_string = (0..10).fold(template, |current_template, _| {
        let pairs = current_template.graphemes(false).tuple_windows::<(_, _)>();
        let mut pairs = pairs.peekable();
        let (first, _) = pairs.peek().cloned().unwrap();

        let mut final_template = vec![first];

        for (first, second) in pairs {
            if let Some(value) = pair_insertions.get(&(first, second)) {
                final_template.push(value);
            }
            final_template.push(second);
        }

        String::from_iter(final_template)
    });

    let counts = final_string.graphemes(false).counts();
    let max_value = counts
        .values()
        .max_by(|x, y| x.cmp(y))
        .ok_or_else(|| anyhow::anyhow!("Could not get max value"))?;
    let min_value = counts
        .values()
        .min_by(|x, y| x.cmp(y))
        .ok_or_else(|| anyhow::anyhow!("Could not get max value"))?;

    Ok(max_value - min_value)
}

fn part_two(lines: &[String]) -> Result<usize> {
    let (template, pair_insertions) = partition_instructions(lines);
    let pair_insertions = create_pair_mapping(&pair_insertions);
    let template_pairs = template.graphemes(false).tuple_windows::<(_, _)>();

    let initial_counts = HashMap::from_iter(template_pairs.counts());
    let final_pair_counts = (0..40).fold(initial_counts, |current_count, _| {
        current_count
            .iter()
            .fold(HashMap::new(), |mut acc, ((first, second), total)| {
                if let Some(value) = pair_insertions.get(&(first, second)) {
                    *acc.entry((first, *value)).or_insert(0) += total;
                    *acc.entry((*value, second)).or_insert(0) += total;
                }

                acc
            })
    });

    let mut final_char_counts =
        final_pair_counts
            .iter()
            .fold(HashMap::new(), |mut acc, (&(first, _), pair_total)| {
                *acc.entry(first).or_default() += pair_total;
                acc
            });

    // Need to grab the end of the string too, we were iterating over all the pairs and only grabbing the first of the pair
    *final_char_counts
        .entry(template.graphemes(false).last().unwrap())
        .or_insert(0) += 1;
    let max_value = final_char_counts.values().max().unwrap();
    let min_value = final_char_counts.values().min().unwrap();

    Ok(max_value - min_value)
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
