use anyhow::Result;
use std::time::Instant;

use day_7::*;

fn main() -> Result<()> {
    let equations = read_data(DATA)?;
    let now = Instant::now();
    let result = equations
        .iter()
        .map(|eq| match eq.is_valid() {
            true => eq.result,
            false => 0,
        })
        .sum::<ValueType>();

    println!("Part1: {} - {:.2?}", result, now.elapsed());
    Ok(())
}
