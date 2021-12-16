use anyhow::{Context, Result};
use bitreader::BitReader;
use hex::decode;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const CURRENT_FILE: &str = file!();
const INPUT_FILE_PATH: &str = "../data/input.txt";

#[derive(Debug, Clone)]
enum PacketContents {
    Data(u64),
    Children(Vec<Packet>),
}

#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    r#type: u8,
    data: PacketContents,
}

impl Packet {
    fn try_parse_bytes(value: &[u8]) -> Result<Self> {
        let mut reader = BitReader::new(value);
        let version = reader.read_u8(3)?;
        let r#type = reader.read_u8(3)?;
        let data = Packet::_parse_data(version, r#type, &mut reader)?;
        Ok(Self {
            version,
            r#type,
            data,
        })
    }

    fn try_from_reader(reader: &mut BitReader) -> Result<Self> {
        let version = reader.read_u8(3)?;
        let r#type = reader.read_u8(3)?;
        let data = Packet::_parse_data(version, r#type, reader)?;
        Ok(Self {
            version,
            r#type,
            data,
        })
    }

    fn _parse_data(
        _version: u8,
        packet_type_id: u8,
        reader: &mut BitReader,
    ) -> Result<PacketContents> {
        let parsed = match packet_type_id {
            4 => {
                let mut needs_more = 1;
                let mut out = 0_u64;
                while needs_more == 1 {
                    needs_more = reader.read_u8(1)?;
                    out <<= 4;
                    out |= u64::from(reader.read_u8(4)?);
                }
                PacketContents::Data(out)
            }
            _ => {
                let length_type_id = reader.read_u8(1)?;
                match length_type_id {
                    0 => {
                        let total_bits = u64::from(reader.read_u16(15)?);
                        let start = reader.position();
                        let mut children = vec![];
                        while reader.position() < start + total_bits {
                            children.push(Packet::try_from_reader(reader)?);
                        }
                        PacketContents::Children(children)
                    }
                    1 => {
                        let total_packets = reader.read_u16(11)?;
                        let packets: Result<Vec<Packet>> = (0..total_packets)
                            .map(|_index| Packet::try_from_reader(reader))
                            .collect();
                        PacketContents::Children(packets?)
                    }
                    _ => unreachable!(),
                }
            }
        };

        Ok(parsed)
    }

    fn sum_versions(&self) -> u64 {
        let sub_versions = match &self.data {
            PacketContents::Children(children) => children.iter().map(Packet::sum_versions).sum(),
            _ => 0,
        };

        u64::from(self.version) + sub_versions
    }

    fn evaluate(&self) -> u64 {
        match &self.r#type {
            0 => self.evaluate_sum(),
            1 => self.evaluate_product(),
            2 => self.evaluate_minimum(),
            3 => self.evaluate_maximum(),
            4 => self.evaluate_literal(),
            5 => self.evaluate_gt(),
            6 => self.evaluate_lt(),
            7 => self.evaluate_eq(),
            _ => unreachable!(),
        }
    }

    fn evaluate_sum(&self) -> u64 {
        match &self.data {
            PacketContents::Children(children) => children.iter().map(Packet::evaluate).sum(),
            _ => unreachable!(),
        }
    }

    fn evaluate_product(&self) -> u64 {
        match &self.data {
            PacketContents::Children(children) => children.iter().map(Packet::evaluate).product(),
            _ => unreachable!(),
        }
    }

    fn evaluate_minimum(&self) -> u64 {
        match &self.data {
            PacketContents::Children(children) => children
                .iter()
                .map(Packet::evaluate)
                .min()
                .expect("Could not get minumum"),
            _ => unreachable!(),
        }
    }

    fn evaluate_maximum(&self) -> u64 {
        match &self.data {
            PacketContents::Children(children) => children
                .iter()
                .map(Packet::evaluate)
                .max()
                .expect("Could not get maximum"),
            _ => unreachable!(),
        }
    }

    fn evaluate_gt(&self) -> u64 {
        match &self.data {
            PacketContents::Children(children) => {
                let mut child_iter = children.iter().map(Packet::evaluate);
                let (first, second) = (child_iter.next().unwrap(), child_iter.next().unwrap());
                if first > second {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }

    fn evaluate_lt(&self) -> u64 {
        match &self.data {
            PacketContents::Children(children) => {
                let mut child_iter = children.iter().map(Packet::evaluate);
                let (first, second) = (child_iter.next().unwrap(), child_iter.next().unwrap());
                if first < second {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }

    fn evaluate_eq(&self) -> u64 {
        match &self.data {
            PacketContents::Children(children) => {
                let mut child_iter = children.iter().map(Packet::evaluate);
                let (first, second) = (child_iter.next().unwrap(), child_iter.next().unwrap());
                if first == second {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }

    fn evaluate_literal(&self) -> u64 {
        match &self.data {
            PacketContents::Data(val) => *val,
            _ => unreachable!(),
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

fn parse_line(line: &str) -> Result<Vec<u8>> {
    decode(line).context("Could not decode line")
}

fn part_one(lines: &[String]) -> Result<u64> {
    let data = lines
        .first()
        .ok_or_else(|| anyhow::anyhow!("Couldn't get data line"))?;
    let data = parse_line(data)?;
    let packet = Packet::try_parse_bytes(&data)?;

    Ok(packet.sum_versions())
}

fn part_two(lines: &[String]) -> Result<u64> {
    let data = lines
        .first()
        .ok_or_else(|| anyhow::anyhow!("Couldn't get data line"))?;
    let data = parse_line(data)?;
    let packet = Packet::try_parse_bytes(&data)?;

    Ok(packet.evaluate())
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
