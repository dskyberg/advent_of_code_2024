use anyhow::Result;
use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum Error {
    #[error("General ERror")]
    GeneralError,
}

pub fn read_data(file_name: &str) -> Result<Vec<String>> {
    let mut result = Vec::new();
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        result.push(line?);
    }
    Ok(result)
}

pub fn process_line(line: &str) -> u32 {
    let mut result = 0;
    let Ok(re) = Regex::new(r"(mul\((\d+),(\d+)\))+?") else {
        println!("Bad regex");
        return 0;
    };
    for (_, [_, lh, rh]) in re.captures_iter(line).map(|c| c.extract()) {
        let Ok(lh) = lh.parse::<u32>() else {
            return 0;
        };
        let Ok(rh) = rh.parse::<u32>() else {
            return 0;
        };
        result += lh * rh;
    }
    result
}
