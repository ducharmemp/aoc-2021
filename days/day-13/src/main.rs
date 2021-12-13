use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

const CURRENT_FILE: &str = file!();
const INPUT_FILE_PATH: &str = "../data/input.txt";

type Coord = (usize, usize);

#[derive(Debug, Clone)]
enum FoldInstruction {
    Horizontal(usize),
    Vertical(usize),
}

impl FromStr for FoldInstruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let last_word = s.split_ascii_whitespace().rev().next();
        let last_word =
            last_word.ok_or_else(|| anyhow::anyhow!("Could not get line for fold instruction"))?;
        let (direction, line) = last_word
            .split_once("=")
            .ok_or_else(|| anyhow::anyhow!("Could not get instructions"))?;

        let line = str::parse::<usize>(line)?;

        match direction {
            "y" => Ok(Self::Horizontal(line)),
            "x" => Ok(Self::Vertical(line)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Grid {
    points: HashSet<Coord>,
}

impl Grid {
    fn fold(&mut self, instruction: &FoldInstruction) {
        match instruction {
            FoldInstruction::Horizontal(new_y_size) => {
                let (points_greater_than_fold, points_less_than_fold): (Vec<Coord>, Vec<Coord>) =
                    self.points.iter().partition(|(_, y)| new_y_size < y);
                let adjusted_points: Vec<Coord> = points_greater_than_fold
                    .into_iter()
                    .map(|(x, y)| (x, new_y_size - (y - new_y_size)))
                    .collect();
                self.points = HashSet::from_iter(
                    points_less_than_fold
                        .iter()
                        .cloned()
                        .chain(adjusted_points.iter().cloned()),
                );
            }
            FoldInstruction::Vertical(new_x_size) => {
                let (points_greater_than_fold, points_less_than_fold): (Vec<Coord>, Vec<Coord>) =
                    self.points.iter().partition(|(x, _)| new_x_size < x);
                let adjusted_points: Vec<Coord> = points_greater_than_fold
                    .into_iter()
                    .map(|(x, y)| (new_x_size - (x - new_x_size), y))
                    .collect();
                self.points = HashSet::from_iter(
                    points_less_than_fold
                        .iter()
                        .cloned()
                        .chain(adjusted_points.iter().cloned()),
                );
            }
        }
    }

    fn total_points(&self) -> usize {
        self.points.len()
    }
}

impl FromIterator<Coord> for Grid {
    fn from_iter<T: IntoIterator<Item = Coord>>(iter: T) -> Self {
        Self {
            points: HashSet::from_iter(iter),
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (max_x, _) = self
            .points
            .iter()
            .max_by(|first, second| (first.0).cmp(&second.0))
            .expect("Could not determine max x value");
        let (_, max_y) = self
            .points
            .iter()
            .max_by(|first, second| (first.1).cmp(&second.1))
            .expect("Could not determine max y value");

        for y in 0..=*max_y {
            let points: HashSet<Coord> = HashSet::from_iter(
                self.points
                    .iter()
                    .filter(|(_, point_y)| y == *point_y)
                    .cloned(),
            );
            for x in 0..=*max_x {
                if points.contains(&(x, y)) {
                    f.write_str("X")?;
                } else {
                    f.write_str(".")?;
                }
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
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

fn parse_coords(line: &str) -> Result<Coord> {
    let (x, y) = line
        .split_once(',')
        .ok_or_else(|| anyhow::anyhow!("Could not split coord line"))?;
    Ok((str::parse::<usize>(x)?, str::parse::<usize>(y)?))
}

fn partition_instructions<'a>(lines: &'a [String]) -> (Vec<&'a str>, Vec<&'a str>) {
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
        lines.get(0).cloned().expect("Could not get coords"),
        lines
            .get(1)
            .cloned()
            .expect("Could not get folding instructions"),
    )
}

fn part_one(lines: &[String]) -> Result<usize> {
    let (coords, instructions) = partition_instructions(lines);
    let coords: Result<Vec<Coord>> = coords.iter().map(|val| parse_coords(val)).collect();
    let coords = coords?;
    let mut grid = Grid::from_iter(coords);
    let instructions: Result<Vec<FoldInstruction>> = instructions
        .into_iter()
        .map(FoldInstruction::from_str)
        .collect();
    let instructions = instructions?;

    let first_instruction = instructions
        .first()
        .ok_or_else(|| anyhow::anyhow!("Could not get first instruction"))?;
    grid.fold(first_instruction);

    Ok(grid.total_points())
}

fn part_two(lines: &[String]) -> Result<String> {
    let (coords, instructions) = partition_instructions(lines);
    let coords: Result<Vec<Coord>> = coords.iter().map(|val| parse_coords(val)).collect();
    let coords = coords?;
    let mut grid = Grid::from_iter(coords);
    let instructions: Result<Vec<FoldInstruction>> = instructions
        .into_iter()
        .map(FoldInstruction::from_str)
        .collect();
    let instructions = instructions?;

    for instruction in instructions {
        grid.fold(&instruction);
    }

    Ok(grid.to_string())
}

fn main() -> Result<()> {
    let input_path = Path::new(CURRENT_FILE)
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Couldn't get parent directory"))?
        .join(INPUT_FILE_PATH);

    let input = read_lines(&input_path)?;
    println!("{:?}", part_one(&input)?);
    println!("{}", part_two(&input)?);

    Ok(())
}
