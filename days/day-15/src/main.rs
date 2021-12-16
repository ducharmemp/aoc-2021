use anyhow::{Context, Result};

use pathfinding::directed::dijkstra::dijkstra;
use pathfinding::grid::Grid;

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

fn parse_line(line: &str) -> Result<Vec<usize>> {
    line.graphemes(false)
        .map(|val| str::parse::<usize>(val).context("Could not parse input value"))
        .collect()
}

fn build_graph(lines: &[Vec<usize>]) -> Grid {
    let mut grid = Grid::new(lines[0].len(), lines.len());
    grid.fill();
    grid
}

fn part_one(lines: &[String]) -> Result<usize> {
    let lines: Result<Vec<Vec<_>>> = lines.iter().map(|line| parse_line(line)).collect();
    let lines = lines?;
    let graph = build_graph(&lines);
    let _end = (lines[0].len() - 1, lines.len() - 1);
    let (_, cost): (Vec<(usize, usize)>, usize) = dijkstra(
        &(0, 0),
        |node| -> Vec<((usize, usize), usize)> {
            let neighbors = graph.neighbours(*node);
            neighbors
                .iter()
                .map(|&(x, y)| ((x, y), lines[y][x]))
                .collect()
        },
        |target| *target == (graph.width - 1, graph.height - 1),
    )
    .unwrap();

    Ok(cost)
}

fn part_two(lines: &[String]) -> Result<usize> {
    let lines: Result<Vec<Vec<_>>> = lines.iter().map(|line| parse_line(line)).collect();
    let lines = lines?;

    let row_len = lines.len();
    let col_len = lines[0].len();

    let lines = (0..(5 * row_len))
        .map(|x| {
            (0..(5 * col_len))
                .map(|y| {
                    let cost =
                        lines[x % lines.len()][y % col_len] + (x / lines.len()) + (y / col_len);
                    if cost < 10 {
                        cost
                    } else {
                        cost - 9
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let graph = build_graph(&lines);

    let (_path, cost): (Vec<(usize, usize)>, usize) = dijkstra(
        &(0, 0),
        |node| -> Vec<((usize, usize), usize)> {
            let neighbors = graph.neighbours(*node);
            neighbors
                .iter()
                .map(|&(x, y)| ((x, y), lines[y][x]))
                .collect()
        },
        |target| *target == (graph.width - 1, graph.height - 1),
    )
    .unwrap();

    Ok(cost)
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
