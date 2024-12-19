use anyhow::Result;
use aoc_utils::*;

pub const TEST_DATA: &str = r#"125 17"#;

pub const DATA: &str = r#"5910927 0 1 47 261223 94788 545 7771"#;

pub fn blink(nums: &[usize]) -> Result<Vec<usize>> {
    let mut result = Vec::new();

    for num in nums {
        // Rule 1: Number is zero
        if *num == 0 {
            result.push(1)
        }
        // Rule 2: Even number of digits
        // Split into 2 nums
        else if num.num_digits() % 2 == 0 {
            let val = format!("{}", num);
            let left = &val[0..val.len() / 2];
            let right = &val[val.len() / 2..];
            result.push(left.parse::<usize>()?);
            result.push(right.parse::<usize>()?);
        }
        // Rule 3: Multiply by 2024
        else {
            result.push(num * 2024)
        }
    }

    Ok(result)
}
