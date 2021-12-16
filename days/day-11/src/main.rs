use anyhow::{Context, Result};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const CURRENT_FILE: &str = file!();
const INPUT_FILE_PATH: &str = "../data/input.txt";

#[derive(Default, Clone)]
struct OctoGraph {
    adjacency: HashMap<(usize, usize), Vec<(usize, usize)>>,
    value_map: HashMap<(usize, usize), u8>,
}

impl Display for OctoGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..10 {
            let line: Vec<String> = (0..10).map(|x| self.value_at((x, y)).to_string()).collect();
            f.write_str(&format!("{}\n", &line.join(" ")))?
        }

        Ok(())
    }
}

impl OctoGraph {
    fn add_node(&mut self, coords: (usize, usize), value: u8) {
        self.value_map.entry(coords).or_insert(value);
    }

    fn add_edge(&mut self, coords: (usize, usize), neighbor_coords: (usize, usize)) {
        let adjacency_list = self.adjacency.entry(coords).or_insert(vec![]);
        adjacency_list.push(neighbor_coords);
    }

    fn neighbors(&self, coords: (usize, usize)) -> Vec<(usize, usize)> {
        self.adjacency
            .get(&coords)
            .expect("Could not get neighbors for coords")
            .clone()
    }

    fn incr_value(&mut self, coords: (usize, usize)) -> u8 {
        let val = self.value_map.entry(coords).or_default();
        *val += 1;
        *val
    }

    fn value_at(&self, coords: (usize, usize)) -> u8 {
        *self
            .value_map
            .get(&coords)
            .expect("Couldn't get value at coords")
    }

    fn flash(&mut self, coords: (usize, usize)) {
        let val = self.value_map.entry(coords).or_default();
        *val = 0;
    }

    fn all_eq(&self) -> bool {
        HashSet::<&u8>::from_iter(self.value_map.values()).len() == 1
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

fn parse_line(line: &str) -> Result<Vec<u8>> {
    line.chars()
        .map(|val| str::parse::<u8>(&val.to_string()).context("Could not parse u8"))
        .collect()
}

fn item_exists_at(x: usize, y: usize, lines: &[Vec<u8>]) -> Option<u8> {
    let item = lines.get(y).map(|line| line.get(x)).flatten();
    item.cloned()
}

fn construct_graph(lines: &[Vec<u8>]) -> OctoGraph {
    let mut graph = OctoGraph::default();
    for (y, line) in lines.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            let neighbor_coords = [
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x - 1, y),
                (x + 1, y),
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
            ];

            for (neighbor_x, neighbor_y) in neighbor_coords.into_iter() {
                if item_exists_at(neighbor_x, neighbor_y, lines).is_some() {
                    graph.add_node((x, y), *value);
                    graph.add_edge((x, y), (neighbor_x, neighbor_y));
                }
            }
        }
    }

    graph
}

fn part_one(lines: &[String]) -> Result<i64> {
    let lines: Result<Vec<Vec<u8>>> = lines.iter().map(|line| parse_line(line)).collect();
    let lines = lines?;

    let mut graph = construct_graph(&lines);

    let mut total_flashes = 0;
    for _ in 0..100 {
        let mut flashed = HashSet::new();
        for y in 0..10_usize {
            for x in 0..10_usize {
                let mut to_visit = VecDeque::new();
                to_visit.push_front((x, y));

                while let Some((current_x, current_y)) = to_visit.pop_front() {
                    if flashed.contains(&(current_x, current_y)) {
                        continue;
                    }

                    let new_val = graph.incr_value((current_x, current_y));
                    if new_val > 9 {
                        total_flashes += 1;
                        graph.flash((current_x, current_y));
                        flashed.insert((current_x, current_y));
                        for neighbor in graph.neighbors((current_x, current_y)) {
                            to_visit.push_back(neighbor);
                        }
                    }
                }
            }
        }
    }

    Ok(total_flashes)
}

fn part_two(lines: &[String]) -> Result<i64> {
    let lines: Result<Vec<Vec<u8>>> = lines.iter().map(|line| parse_line(line)).collect();
    let lines = lines?;

    let mut graph = construct_graph(&lines);

    let mut total_steps = 0;
    while !graph.all_eq() {
        total_steps += 1;
        let mut flashed = HashSet::new();
        for y in 0..10_usize {
            for x in 0..10_usize {
                let mut to_visit = VecDeque::new();
                to_visit.push_front((x, y));

                while let Some((current_x, current_y)) = to_visit.pop_front() {
                    if flashed.contains(&(current_x, current_y)) {
                        continue;
                    }

                    let new_val = graph.incr_value((current_x, current_y));
                    if new_val > 9 {
                        graph.flash((current_x, current_y));
                        flashed.insert((current_x, current_y));
                        for neighbor in graph.neighbors((current_x, current_y)) {
                            to_visit.push_back(neighbor);
                        }
                    }
                }
            }
        }
    }

    Ok(total_steps)
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
