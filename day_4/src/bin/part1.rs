use anyhow::Result;

use day_4::*;

trait OrEquals {
    fn or_eq(&mut self, or: u32, eq: u32);
}

impl OrEquals for u32 {
    #[inline]
    fn or_eq(&mut self, eq: u32, or: u32) {
        if *self == eq {
            *self |= or;
        } else {
            *self = 0;
        }
    }
}

fn find_words(row: &[u8]) -> u32 {
    let mut result = 0;
    let mut found: u32 = 0;
    for &val in row {
        match val {
            X => found = 1,
            M => found.or_eq(1, 2),
            A => found.or_eq(3, 4),
            S => found.or_eq(7, 8),
            _ => found = 0,
        }
        if found == 15 {
            result += 1;
            found = 0;
        }
    }
    result
}

fn main() -> Result<()> {
    let data = read_data(DATA);
    let mut result = 0;

    // Horizontal
    for row in &data {
        result += find_words(row);
        let m = rev_row(row);
        result += find_words(&m);
    }

    // Vertical
    let matrix = transpose(&data);
    for row in &matrix {
        result += find_words(row);
        let m = rev_row(row);
        result += find_words(&m);
    }

    // Diagonal L-R
    let matrix = diagonal(&data);
    for row in &matrix {
        result += find_words(row);
        let m = rev_row(row);
        result += find_words(&m);
    }

    // Diagonal R-L
    let matrix = diagonal(&reverse(&data));
    for row in &matrix {
        result += find_words(row);
        let m = rev_row(row);
        result += find_words(&m);
    }

    println!("Part 1: {}", result);
    Ok(())
}
