use anyhow::{Context, Result};
use std::collections::HashMap;
use std::collections::HashSet;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const CURRENT_FILE: &str = file!();
const INPUT_FILE_PATH: &str = "../data/input.txt";

#[derive(Debug, Clone)]
enum NodeType {
    Small,
    Large,
}

#[derive(Default, Debug)]
struct Graph<'a> {
    adjacency: HashMap<&'a str, Vec<&'a str>>,
    value_map: HashMap<&'a str, NodeType>,
}

impl<'a> Graph<'a> {
    fn add_node(&mut self, name: &'a str, node_type: NodeType) {
        self.value_map.entry(name).or_insert(node_type);
    }

    fn add_edge(&mut self, name: &'a str, neighbor_name: &'a str) {
        let adjacency_list = self.adjacency.entry(name).or_insert_with(Vec::new);
        adjacency_list.push(neighbor_name);
    }

    fn neighbors(&self, name: &'a str) -> Vec<&'a str> {
        self.adjacency.get(name).cloned().unwrap_or_default()
    }

    fn value_at(&self, name: &'a str) -> NodeType {
        self.value_map
            .get(&name)
            .expect("Couldn't get value for name")
            .clone()
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

fn parse_line(line: &str) -> Result<(&str, &str)> {
    line.split_once("-")
        .ok_or_else(|| anyhow::anyhow!("Could not parse line"))
}

fn construct_graph<'a>(lines: Vec<(&'a str, &'a str)>) -> Graph<'a> {
    let mut graph = Graph::default();
    for (node, neighbor) in lines.into_iter() {
        graph.add_node(
            node,
            if node.to_ascii_uppercase() == node {
                NodeType::Large
            } else {
                NodeType::Small
            },
        );
        graph.add_node(
            neighbor,
            if neighbor.to_ascii_uppercase() == neighbor {
                NodeType::Large
            } else {
                NodeType::Small
            },
        );
        graph.add_edge(node, neighbor);
        graph.add_edge(neighbor, node);
    }

    graph
}

fn find_paths_for_target<'a>(
    graph: &Graph<'a>,
    current_node: &'a str,
    target: &'a str,
    visits: &mut HashSet<&'a str>,
    mut seen_twice: Option<&'a str>,
) -> usize {
    if current_node == target {
        return 1;
    }

    match graph.value_at(current_node) {
        NodeType::Small => {
            if !visits.insert(current_node) {
                if seen_twice.is_some() || current_node == "start" {
                    return 0;
                }
                seen_twice = Some(current_node);
            }
        }
        NodeType::Large => (),
    };

    let path_sum = graph
        .neighbors(current_node)
        .into_iter()
        .map(|node| find_paths_for_target(graph, node, target, visits, seen_twice))
        .sum();

    if seen_twice.unwrap_or("") != current_node {
        visits.remove(current_node);
    }
    path_sum
}

fn part_one(lines: &[String]) -> Result<usize> {
    let lines: Result<Vec<(&str, &str)>> = lines.iter().map(|line| parse_line(line)).collect();
    let lines = lines?;
    let graph = construct_graph(lines);
    let start = "start";
    let end = "end";
    Ok(find_paths_for_target(
        &graph,
        start,
        end,
        &mut HashSet::new(),
        Some(""),
    ))
}

fn part_two(lines: &[String]) -> Result<usize> {
    let lines: Result<Vec<(&str, &str)>> = lines.iter().map(|line| parse_line(line)).collect();
    let lines = lines?;
    let graph = construct_graph(lines);
    let start = "start";
    let end = "end";
    Ok(find_paths_for_target(
        &graph,
        start,
        end,
        &mut HashSet::new(),
        None,
    ))
}

fn main() -> Result<()> {
    let input_path = Path::new(CURRENT_FILE)
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Couldn't get parent directory"))?
        .join(INPUT_FILE_PATH);

    let input = read_lines(&input_path)?;
    println!("{:?}", part_one(&input)?);
    println!("{:?}", part_two(&input)?);
    assert_eq!(5920, part_one(&input)?);
    assert_eq!(155477, part_two(&input)?);

    Ok(())
}
