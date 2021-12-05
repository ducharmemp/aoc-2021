use anyhow::{Context, Result};
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

#[derive(Debug, Clone, Default, PartialEq, Eq, Ord, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

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
pub struct Line {
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

pub struct LineIterator {
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
        let vector = self.line.end.clone() - self.line.start.clone();
        let direction = vector.direction();

        if self.current_position == (self.line.end.clone() + direction.clone()) {
            return None;
        }

        let position = self.current_position.clone();
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iterator_produces_correct_values() {
        let line: Line = "8,0 -> 0,8".try_into().unwrap();
        let points = line.into_iter().collect::<Vec<Point>>();
        assert_eq!(
            points,
            vec![
                Point::new(8, 0),
                Point::new(7, 1),
                Point::new(6, 2),
                Point::new(5, 3),
                Point::new(4, 4),
                Point::new(3, 5),
                Point::new(2, 6),
                Point::new(1, 7),
                Point::new(0, 8)
            ]
        )
    }

    #[test]
    fn iterator_produces_correct_values_rev() {
        let line: Line = "0,8 -> 8,0".try_into().unwrap();
        let points = line.into_iter().collect::<Vec<Point>>();
        assert_eq!(
            points,
            vec![
                Point::new(8, 0),
                Point::new(7, 1),
                Point::new(6, 2),
                Point::new(5, 3),
                Point::new(4, 4),
                Point::new(3, 5),
                Point::new(2, 6),
                Point::new(1, 7),
                Point::new(0, 8)
            ]
            .iter()
            .rev()
            .cloned()
            .collect::<Vec<Point>>()
        )
    }
}
