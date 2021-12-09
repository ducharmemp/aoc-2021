use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Mutex;

const CURRENT_FILE: &str = file!();
const INPUT_FILE_PATH: &str = "../data/input.txt";

#[derive(Debug, Clone, Default)]
struct BasinTree {
    neighbors: [Option<Box<BasinTree>>; 4],
    value: u32,
}

impl BasinTree {
    fn from_grid(
        start_x: usize,
        start_y: usize,
        grid: &Vec<Vec<u32>>,
        visited: &Mutex<HashSet<(usize, usize)>>,
    ) -> Option<Box<Self>> {
        let current_value = grid
            .get(start_y)
            .map(|row| row.get(start_x))
            .flatten()
            .cloned();

        let current_value = current_value?;
        {
            let mut visited = visited.try_lock().ok()?;
            if current_value == 9 || visited.contains(&(start_x, start_y)) {
                return None;
            }
            visited.insert((start_x, start_y));
        }

        let neighbors = [
            BasinTree::from_grid(start_x, start_y + 1, grid, visited),
            BasinTree::from_grid(start_x + 1, start_y, grid, visited),
            BasinTree::from_grid(start_x - 1, start_y, grid, visited),
            BasinTree::from_grid(start_x, start_y - 1, grid, visited),
        ];

        Some(Box::new(Self {
            value: current_value,
            neighbors,
        }))
    }

    fn size(&self) -> usize {
        self.neighbors
            .iter()
            .map(|neighbor| neighbor.as_ref().map(|n| n.size()).unwrap_or_default())
            .sum::<usize>()
            + 1
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

fn parse_line(line: &str) -> Result<Vec<u32>> {
    let heights: Result<Vec<u32>> = line
        .chars()
        .map(|val| str::parse::<u32>(&val.to_string()).context("Could not parse number"))
        .collect();
    heights
}

fn get_neighbor_value(heights: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    heights
        .get(y)
        .map(|row| row.get(x))
        .flatten()
        .cloned()
        .unwrap_or(u32::MAX)
}

fn find_low_points(heights: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let points = heights.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().map(move |(x, column)| {
            let top = get_neighbor_value(heights, x, y - 1);
            let left = get_neighbor_value(heights, x - 1, y);
            let right = get_neighbor_value(heights, x + 1, y);
            let bottom = get_neighbor_value(heights, x, y + 1);

            if [top, left, right, bottom].iter().all(|val| column < val) {
                Some((x, y))
            } else {
                None
            }
        })
    });

    points.flatten().collect()
}

fn part_one(lines: &[String]) -> Result<u32> {
    let heights: Result<Vec<Vec<u32>>> = lines.iter().map(|line| parse_line(line)).collect();
    let heights = heights?;

    let low_points = find_low_points(&heights);

    Ok(low_points
        .iter()
        .map(|(x, y)| {
            heights
                .get(*y)
                .map(|row| row.get(*x))
                .flatten()
                .cloned()
                .unwrap()
                + 1
        })
        .sum())
}

fn part_two(lines: &[String]) -> Result<usize> {
    let heights: Result<Vec<Vec<u32>>> = lines.iter().map(|line| parse_line(line)).collect();
    let heights = heights?;

    let low_points = find_low_points(&heights);

    let trees: Result<Vec<Box<BasinTree>>> = low_points
        .iter()
        .map(|(x, y)| {
            BasinTree::from_grid(*x, *y, &heights, &Mutex::new(HashSet::new()))
                .ok_or_else(|| anyhow::anyhow!("Could not construct tree for basin"))
        })
        .collect();
    let trees = trees?;
    let mut sizes = trees.iter().map(|tree| tree.size()).collect::<Vec<usize>>();
    sizes.sort_unstable();
    let top_three = sizes.iter().rev().take(3);
    Ok(top_three.product())
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
