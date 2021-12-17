use anyhow::{Context, Result};
use itertools::Itertools;

use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Add, AddAssign, SubAssign};
use std::path::Path;

const CURRENT_FILE: &str = file!();
const INPUT_FILE_PATH: &str = "../data/input.txt";

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Point {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

impl Add<Velocity> for Point {
    type Output = Self;

    fn add(self, rhs: Velocity) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Velocity> for Point {
    fn add_assign(&mut self, other: Velocity) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Velocity {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Velocity {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

impl<V: Into<Velocity>> SubAssign<V> for Velocity {
    fn sub_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

struct BoundingBox {
    top_left: Point,
    bottom_right: Point,
}

impl BoundingBox {
    fn from_coords(xleft: i64, xright: i64, ytop: i64, ybottom: i64) -> Self {
        Self {
            top_left: Point { x: xleft, y: ytop },
            bottom_right: Point {
                x: xright,
                y: ybottom,
            },
        }
    }

    fn has_collision(&self, probe: &Probe) -> bool {
        let point = probe.position();

        (self.top_left.x <= point.x && point.x <= self.bottom_right.x)
            && (self.bottom_right.y <= point.y && point.y <= self.top_left.y)
    }

    fn overshot(&self, probe: &Probe) -> bool {
        let point = probe.position();

        point.x > self.bottom_right.x || point.y < self.bottom_right.y
    }
}

#[derive(Debug, Clone)]
struct Probe {
    position: Point,
    velocity: Velocity,
}

impl Probe {
    fn new<P: Into<Point>, V: Into<Velocity>>(position: P, velocity: V) -> Self {
        Self {
            position: position.into(),
            velocity: velocity.into(),
        }
    }

    fn step(&mut self) -> (i64, i64) {
        self.position += self.velocity;
        self.velocity -= (1, 1);
        self.velocity.x = self.velocity.x.clamp(0, i64::MAX);
        (self.position.x, self.position.y)
    }

    fn position(&self) -> &Point {
        &self.position
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

fn parse_line(line: &str) -> Result<BoundingBox> {
    let (_, coords) = line
        .split_once(":")
        .ok_or_else(|| anyhow::anyhow!("Could not split coords"))?;
    let (x_vals, y_vals) = coords
        .split_once(",")
        .ok_or_else(|| anyhow::anyhow!("Could not split coords"))?;
    let (_, x_range) = x_vals
        .split_once("=")
        .ok_or_else(|| anyhow::anyhow!("Could not split x coords"))?;
    let (_, y_range) = y_vals
        .split_once("=")
        .ok_or_else(|| anyhow::anyhow!("Could not split y coords"))?;
    let (min_x, max_x) = x_range
        .split_once("..")
        .ok_or_else(|| anyhow::anyhow!("Could not get max or min x coords"))?;
    let (min_y, max_y) = y_range
        .split_once("..")
        .ok_or_else(|| anyhow::anyhow!("Could not get max or min x coords"))?;
    Ok(BoundingBox::from_coords(
        str::parse::<i64>(min_x)?,
        str::parse::<i64>(max_x)?,
        str::parse::<i64>(max_y)?,
        str::parse::<i64>(min_y)?,
    ))
}

fn part_one(lines: &[String]) -> Result<i64> {
    let target = parse_line(lines.first().expect("Could not get first line"))?;

    let vals = (0..=1000_i64)
        .cartesian_product(-1000..=1000_i64)
        .flat_map(|(x, y)| {
            let mut probe = Probe::new((0, 0), (x, y));
            let mut steps = vec![];
            while !target.has_collision(&probe) {
                if target.overshot(&probe) {
                    return None;
                }
                steps.push(probe.step());
            }
            Some(steps)
        });

    let max_y_val = vals
        .flatten()
        .max_by(|first, second| first.1.cmp(&second.1))
        .expect("Expected to find a maximum");

    Ok(max_y_val.1)
}

fn part_two(lines: &[String]) -> Result<usize> {
    let target = parse_line(lines.first().expect("Could not get first line"))?;

    let vals = (0..=1000_i64)
        .cartesian_product(-1000..=1000_i64)
        .flat_map(|(x, y)| {
            let mut probe = Probe::new((0, 0), (x, y));
            let mut steps = vec![];
            while !target.has_collision(&probe) {
                if target.overshot(&probe) {
                    return None;
                }
                steps.push(probe.step());
            }
            Some(steps)
        });

    Ok(vals.count())
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
