use anyhow::Result;

use day_2::{read_data, safe_diff};

fn is_safe(row: &[u16]) -> bool {
    if row.len() < 2 {
        return true;
    }

    let ord = row[0].cmp(&row[1]);

    for i in 0..row.len() - 1 {
        let Ok(o) = safe_diff(row[i], row[i + 1]) else {
            return false;
        };
        if o != ord {
            return false;
        }
    }

    true
}

fn walk_to_safe(row: &[u16]) -> bool {
    for i in 0..row.len() {
        let mut row = row.to_vec();
        row.remove(i);
        if is_safe(&row) {
            return true;
        }
    }
    false
}

fn main() -> Result<()> {
    let file_name = "data.dat";
    let data = read_data(file_name)?;
    let mut safe_rows = 0;
    for row in data {
        if is_safe(&row) || walk_to_safe(&row) {
            safe_rows += 1;
        }
    }
    println!("Part 2: {}", safe_rows);
    Ok(())
}
