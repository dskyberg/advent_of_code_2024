use anyhow::Result;

use day_3::{process_line, read_data};

fn main() -> Result<()> {
    let file_name = "data.dat";
    let mut result = 0;
    let data = read_data(file_name)?;
    for line in data {
        result += process_line(&line);
    }
    println!("Part 1 {}", result);
    Ok(())
}
