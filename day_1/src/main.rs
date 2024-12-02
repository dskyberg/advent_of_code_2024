use anyhow::Result;
use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("General error")]
    GeneralError,
    #[error("Regex error")]
    RegExError(#[from] regex::Error),
    #[error("Bad input line")]
    BadInputLine,
}

#[derive(Debug, Default)]
struct Data {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl Data {
    fn add(&mut self, left: &str, right: &str) -> Result<()> {
        self.left.push(left.trim().parse::<usize>()?);
        self.right.push(right.trim().parse::<usize>()?);
        self.left.sort();
        self.right.sort();
        Ok(())
    }
}

fn read_data(file_name: &str) -> Result<Data> {
    let mut result = Data::default();
    let re = Regex::new(r"^\s*(?<left>\d+)\s+(?<right>\d+)\s*$")?;

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let caps = re.captures(&line).ok_or(Error::BadInputLine)?;
        result.add(&caps["left"], &caps["right"])?;
    }
    Ok(result)
}

fn part_1(file_name: &str) -> Result<()> {
    let data = read_data(file_name)?;

    let mut total = 0;
    let len = data.left.len();
    for i in 0..len {
        let left = data.left[i];
        let right = data.right[i];
        let max = left.max(right);
        let min = left.min(right);
        total += max - min;
    }
    println!("Part 1: {}", total);
    Ok(())
}

fn part_2(file_name: &str) -> Result<()> {
    let data = read_data(file_name)?;

    let mut total = 0;
    for val in data.left {
        let len = data
            .right
            .iter()
            .filter(|&&x| x == val)
            .collect::<Vec<&usize>>()
            .len();
        total += val * len;
    }
    println!("Part 2: {}", total);
    Ok(())
}

fn main() -> Result<()> {
    let data_file = "data.dat";
    part_1(data_file)?;
    part_2(data_file)?;
    Ok(())
}
