use anyhow::Result;
use day_3::{process_line, read_data};

const DO: &str = "do()";
const DONT: &str = "don't()";

fn main() -> Result<()> {
    let file_name = "data.dat";
    let mut result = 0;
    let data = read_data(file_name)?;
    let mut data = data.join("");

    // We need to partition along do() and don't().
    loop {
        // Look for the next don't()
        match data.find(DONT) {
            Some(idx) => {
                // Pull the slice for processing
                let slice = &data[0..(idx + DONT.len())];
                result += process_line(slice);
                // Throw away the slice
                data = data[(idx + DONT.len() + 1)..].to_string();
            }
            None => {
                // Looks like we reached the last slice
                result += process_line(&data);
                break;
            }
        }

        // Throw away until the next do()
        if let Some(idx) = data.find(DO) {
            data = data[idx..].to_string();
        };
    }

    println!("Part 2: {}", result);
    Ok(())
}
