use anyhow::{Context, Result};
use itertools::Itertools;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Add, Sub};
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn direction(&self) -> Self {
        Self {
            x: self.x.clamp(-1, 1),
            y: self.y.clamp(-1, 1),
        }
    }
}

impl TryFrom<&str> for Point {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self> {
        let (x, y) = line
            .split_once(",")
            .ok_or_else(|| anyhow::anyhow!("Could not get coords"))?;
        let (x, y) = (x.trim().parse::<i32>()?, y.trim().parse::<i32>()?);

        Ok(Self { x, y })
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}

impl TryFrom<&str> for Line {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self> {
        let (start, end) = line
            .split_once("->")
            .ok_or_else(|| anyhow::anyhow!("Could not get points"))?;
        let (start, end) = (start.trim(), end.trim());

        Ok(Line::new(start.try_into()?, end.try_into()?))
    }
}

fn sorted_range(start: i32, end: i32) -> std::ops::RangeInclusive<i32> {
    if start > end {
        std::ops::RangeInclusive::new(end, start)
    } else {
        std::ops::RangeInclusive::new(start, end)
    }
}

impl IntoIterator for Line {
    type Item = Point;

    type IntoIter = LineIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

struct LineIterator {
    line: Line,
    current_position: Point,
}

impl LineIterator {
    fn new(line: Line) -> LineIterator {
        LineIterator {
            current_position: line.start.clone(),
            line,
        }
    }
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_position == self.line.end {
            return None;
        }

        let vector = self.line.end.clone() - self.line.start.clone();
        let direction = vector.direction();
        let position = self.current_position.clone();
        dbg!(&vector, &direction, &position);
        self.current_position = self.current_position.clone() + direction;
        Some(position)
    }
}

fn part_one(lines: &[String]) -> Result<i32> {
    let lines: Result<Vec<Line>> = lines
        .iter()
        .map(|line| Line::try_from(line.as_str()))
        .collect();
    let lines = lines?;
    let max_x = lines
        .iter()
        .map(|line| std::cmp::max(line.start.x, line.end.x))
        .max()
        .ok_or_else(|| anyhow::anyhow!("Couldn't determine biggest x value"))?;

    let max_y = lines
        .iter()
        .map(|line| std::cmp::max(line.start.y, line.end.y))
        .max()
        .ok_or_else(|| anyhow::anyhow!("Couldn't determine biggest y value"))?;

    let mut grid: Vec<Vec<i32>> = vec![vec![0; (max_x + 1) as usize]; (max_y + 1) as usize];

    for line in lines {
        if line.start.x == line.end.x {
            for y in sorted_range(line.start.y, line.end.y) {
                grid[y as usize][line.start.x as usize] += 1;
            }
        } else if line.start.y == line.end.y {
            for x in sorted_range(line.start.x, line.end.x) {
                grid[line.start.y as usize][x as usize] += 1;
            }
        }
    }

    Ok(grid
        .iter()
        .flatten()
        .filter(|val| **val >= 2)
        .cloned()
        .count() as i32)
}

fn part_two(lines: &[String]) -> Result<i32> {
    let lines: Result<Vec<Line>> = lines
        .iter()
        .map(|line| Line::try_from(line.as_str()))
        .collect();
    let lines = lines?;
    let max_x = lines
        .iter()
        .map(|line| std::cmp::max(line.start.x, line.end.x))
        .max()
        .ok_or_else(|| anyhow::anyhow!("Couldn't determine biggest x value"))?;

    let max_y = lines
        .iter()
        .map(|line| std::cmp::max(line.start.y, line.end.y))
        .max()
        .ok_or_else(|| anyhow::anyhow!("Couldn't determine biggest y value"))?;

    let mut grid: Vec<Vec<i32>> = vec![vec![0; (max_x + 1) as usize]; (max_y + 1) as usize];

    for line in lines.clone() {
        for point in line.into_iter() {
            dbg!(point);
        }
    }

    // for line in lines {
    //     if line.start.x == line.end.x {
    //         for y in sorted_range(line.start.y, line.end.y) {
    //             grid[y as usize][line.start.x as usize] += 1;
    //         }
    //     } else if line.start.y == line.end.y {
    //         for x in sorted_range(line.start.x, line.end.x) {
    //             grid[line.start.y as usize][x as usize] += 1;
    //         }
    //     } else {
    //         let mut x_val: i32 = line.start.x as i32;
    //         let x_add_value: i32 = if line.start.x > line.end.x { -1 } else { 1 };
    //         let mut y_val: i32 = line.start.y as i32;
    //         let y_add_value: i32 = if line.start.y > line.end.y { -1 } else { 1 };

    //         while y_val != (line.end.y as i32 + y_add_value) {
    //             grid[y_val as usize][x_val as usize] += 1;
    //             y_val += y_add_value;
    //             x_val += x_add_value;
    //         }
    //     }
    // }
    for line in lines {
        for point in line {
            grid[point.y as usize][point.x as usize] += 1;
        }
    }

    // dbg!(&grid);
    Ok(grid
        .iter()
        .flatten()
        .filter(|val| **val >= 2)
        .cloned()
        .count() as i32)
}

fn main() -> Result<()> {
    let input_path = Path::new(CURRENT_FILE)
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Couldn't get parent directory"))?
        .join(INPUT_FILE_PATH);

    let input = read_lines(&input_path)?;
    println!("{:?}", part_one(&input)?);
    println!("{:?}", part_two(&input)?);
    // assert_eq!(part_one(&input)?, 5147);
    assert_eq!(part_two(&input)?, 16925);

    Ok(())
}
