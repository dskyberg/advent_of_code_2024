use anyhow::Result;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Bad data line")]
    BadInputLine,
    #[error("Unsafe")]
    Unsafe,
}

pub fn read_data(file_name: &str) -> Result<Vec<Vec<u16>>> {
    let mut result = Vec::new();
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let mut row: Vec<u16> = Vec::new();
        for part in line.split(' ') {
            row.push(part.parse::<u16>()?);
        }
        result.push(row);
    }
    Ok(result)
}

/// Compare 2 values to ensure the diff is between 1 and 3.
/// Returns Ordering::Less if `lh` < `rh`, else Ordering::Greater
pub fn safe_diff(lh: u16, rh: u16) -> Result<Ordering> {
    let ord = lh.cmp(&rh);
    let diff = lh.max(rh) - lh.min(rh);
    if diff > 0 && diff <= 3 {
        return Ok(ord);
    }
    Err(Error::Unsafe.into())
}
