use anyhow::{Context, Result};
use itertools::Itertools;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const CURRENT_FILE: &str = file!();
const INPUT_FILE_PATH: &str = "../data/input.txt";

#[derive(Debug, Clone, Default)]
struct BingoCard {
    matrix: Vec<Vec<u32>>,
    marks: [[bool; 5]; 5],
}

impl BingoCard {
    fn new(matrix: Vec<Vec<u32>>) -> Self {
        Self {
            matrix,
            ..Default::default()
        }
    }

    fn mark(&mut self, val: &u32) {
        for (row_idx, row) in self.matrix.iter().enumerate() {
            for (column_idx, column) in row.iter().enumerate() {
                if column == val {
                    self.marks[row_idx][column_idx] = true;
                }
            }
        }
    }

    fn winner(&self) -> bool {
        for row in self.marks {
            if row.iter().all(|val| *val) {
                return true;
            }
        }

        for column in 0..5 {
            if self.marks.iter().map(|row| row[column]).all(|val| val) {
                return true;
            }
        }

        false
    }

    fn unmarked_numbers(&self) -> Vec<u32> {
        let mut unmarked = vec![];
        for (row_idx, row) in self.matrix.iter().enumerate() {
            for (column_idx, column) in row.iter().enumerate() {
                if !self.marks[row_idx][column_idx] {
                    unmarked.push(*column);
                }
            }
        }

        unmarked
    }
}

#[derive(Debug, Clone, Default)]
struct BingoCardBuilder {
    rows: Vec<Vec<u32>>,
}

impl BingoCardBuilder {
    fn parse_line_to_row<'a>(&'a mut self, line: &str) -> Result<&'a mut Self> {
        assert!(self.rows.len() != 5);
        let columns: Result<Vec<u32>> = line
            .split_whitespace()
            .map(|val| str::parse::<u32>(val).context("Could not parse u32"))
            .collect();
        let columns = columns?;
        self.rows.push(columns);

        Ok(self)
    }

    fn from_rows<'a>(&'a mut self, card_lines: &[&str]) -> Result<&'a mut Self> {
        for line in card_lines {
            self.parse_line_to_row(line)?;
        }
        Ok(self)
    }

    fn build(&mut self) -> BingoCard {
        BingoCard::new(self.rows.clone())
    }
}

struct Game {
    pub cards: Vec<BingoCard>,
    pub drawn_numbers: Vec<u32>,
}

impl Game {
    fn new(cards: Vec<BingoCard>, drawn_numbers: Vec<u32>) -> Self {
        Self {
            cards,
            drawn_numbers,
        }
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

fn chunk_cards<'a, I: Iterator<Item = &'a String>>(lines: I) -> Vec<Vec<&'a str>> {
    lines
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
        .collect()
}

fn find_winning_cards(mut cards: Vec<BingoCard>, drawn_numbers: Vec<u32>) -> Vec<(u32, BingoCard)> {
    let mut winners = vec![];
    for number in &drawn_numbers {
        for card in &mut cards {
            card.mark(number);
            if card.winner() {
                winners.push((*number, card.clone()));
            }
        }

        cards = cards
            .iter()
            .filter(|card| !card.winner())
            .cloned()
            .collect();
    }
    winners
}

fn get_drawn_numbers(line: &str) -> Result<Vec<u32>> {
    line.split(',')
        .map(|val| val.parse::<u32>().context("Could not parse u32"))
        .collect()
}

fn setup_game(lines: &[String]) -> Result<Game> {
    let mut line_iter = lines.iter();
    let first_line = line_iter
        .next()
        .ok_or_else(|| anyhow::anyhow!("Could not get first line"))?;
    let drawn_numbers = get_drawn_numbers(first_line)?;

    let mut cards = vec![];
    for card_lines in chunk_cards(line_iter) {
        let mut current_builder = BingoCardBuilder::default();
        let current_builder = current_builder.from_rows(&card_lines)?;
        cards.push(current_builder.build());
    }

    Ok(Game::new(cards, drawn_numbers))
}

fn part_one(lines: &[String]) -> Result<u32> {
    let game = setup_game(lines)?;

    let winning_cards = find_winning_cards(game.cards, game.drawn_numbers);
    let (drawn_number, winning_card) = winning_cards
        .first()
        .ok_or_else(|| anyhow::anyhow!("Could not find expected card"))?;
    Ok(winning_card.unmarked_numbers().iter().sum::<u32>() * drawn_number)
}

fn part_two(lines: &[String]) -> Result<u32> {
    let game = setup_game(lines)?;

    let winning_cards = find_winning_cards(game.cards, game.drawn_numbers);
    let (drawn_number, winning_card) = winning_cards
        .last()
        .ok_or_else(|| anyhow::anyhow!("Could not find expected card"))?;
    Ok(winning_card.unmarked_numbers().iter().sum::<u32>() * drawn_number)
}

fn main() -> Result<()> {
    let input_path = Path::new(CURRENT_FILE)
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Couldn't get parent directory"))?
        .join(INPUT_FILE_PATH);

    let input = read_lines(&input_path)?;
    println!("{:?}", part_one(&input)?);
    println!("{:?}", part_two(&input)?);

    assert_eq!(part_one(&input)?, 33462);
    assert_eq!(part_two(&input)?, 30070);

    Ok(())
}
