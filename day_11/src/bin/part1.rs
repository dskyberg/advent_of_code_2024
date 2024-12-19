use anyhow::Result;
use std::time::Instant;

use day_11::*;

use aoc_utils::*;

fn main() -> Result<()> {
    let now = Instant::now();
    let mut data = line_to_numbers::<usize>(DATA, ' ')?;
    for _ in 0..25 {
        data = blink(&data)?;
    }

    println!("Part1: {} - {:.2?}", data.len(), now.elapsed());
    Ok(())
}
